use std::fmt::Debug;

use crate::env::Env;
use crate::event::variant::Variant;
use crate::path::{WidgetPath, RelationToSelfRelation};
use crate::queron::Queron;
use crate::queron::query::{QueryStack, Query};
use crate::util::bounds::Offset;
use crate::widget::stack::{QueryCurrentWidget, QueryCurrentBounds};

use super::filter::{QueryVariant, StdEventMode, QueryStdEventMode};

#[derive(Clone,Debug)]
pub struct StdVariant<V,E> where V: Variant<E> + Clone, E: Env {
    pub variant: V,
    pub ts: u64,
    pub filter_path: Option<E::WidgetPath>,
    pub filter_path_strict: bool,
    pub direct_only: bool,
    pub filter_point: Option<Offset>,
}

impl<V,E> StdVariant<V,E> where V: Variant<E> + Clone, E: Env { 
    /// nofilter
    #[inline]
    pub fn new(variant: V, ts: u64) -> Self {
        Self { variant, ts, filter_path: None, filter_path_strict: false, filter_point: None, direct_only: false }
    }
    #[inline]
    pub fn with_filter_path(mut self, filter_path: E::WidgetPath) -> Self {
        self.filter_path = Some(filter_path);
        self.filter_path_strict = false;
        self
    }
    #[inline]
    pub fn with_filter_path_strict(mut self, filter_path: E::WidgetPath) -> Self {
        self.filter_path = Some(filter_path);
        self.filter_path_strict = true;
        self
    }
    #[inline]
    pub fn direct_only(mut self) -> Self {
        self.direct_only = true;
        self
    }
    #[inline]
    pub fn no_direct_only(mut self) -> Self {
        self.direct_only = false;
        self
    }
    #[inline]
    pub fn with_filter_point(mut self, filter_point: Offset) -> Self {
        self.filter_point = Some(filter_point);
        self
    }
}

impl<V,E> super::Event<E> for StdVariant<V,E> where V: Variant<E> + Clone, E: Env {
    type WithPrefetch<R> = R where R: Queron<E>;

    fn _query<'a,Q,S>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, stack: &S) where S: Queron<E> + ?Sized, Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryVariant<V>>() {
            *builder = Some(&self.variant);
        } else if let Some((_,builder)) = builder.downcast::<'_,QueryStdEventMode>() {
            let mut receive_self = true;
            let mut route_to_childs = !self.direct_only;

            let mut child_filter_point = None;
            let mut child_filter_sub_path = None;

            if let Some(filter_path) = &self.filter_path {
                let current_path = QueryCurrentWidget.query_in(stack).unwrap();
                match filter_path.relation_to_self(current_path.path) {
                    RelationToSelfRelation::ParentOfSelf(self_in_parent) => {
                        // currently in parent of route-to widget
                        receive_self = false;
                        route_to_childs = !self.filter_path_strict;
                        child_filter_sub_path = Some(self_in_parent);
                    },
                    RelationToSelfRelation::Identical => {},
                    RelationToSelfRelation::ChildOfSelf(_) => {
                        if self.filter_path_strict {
                            debug_assert!(false, "Misrouted");
                        }
                    },
                    RelationToSelfRelation::Invalid => {
                        // misrouted
                        receive_self = false;
                        route_to_childs = false;
                        debug_assert!(false, "Misrouted");
                    },
                }
            }

            if let Some(filter_point) = self.filter_point {
                let current_bounds = QueryCurrentBounds.query_in(stack).unwrap();

                if filter_point.is_inside(current_bounds.bounds) { //TODO also check viewport
                    child_filter_point = Some(filter_point);
                } else {
                    debug_assert!(false,"Misrouted");
                }
            }

            let mut mode = StdEventMode {
                receive_self,
                route_to_childs,
                child_filter_point,
                child_filter_sub_path,
            };

            *builder = Some(mode);
        }
    }

    fn erase<'s,'ss>(&'s self) -> &'s (dyn super::EventDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }

    fn ts(&self) -> u64 {
        self.ts
    }

    fn with_prefetch<R>(&self, stack: R) -> Self::WithPrefetch<R> where R: Queron<E> {
        stack
    }

    fn _root_only(&self) -> bool {
        self.variant._root_only()
    }
}
