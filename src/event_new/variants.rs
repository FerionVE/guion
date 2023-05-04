use std::fmt::Debug;
use std::marker::PhantomData;

use crate::env::Env;
use crate::event::variant::Variant;
use crate::newpath::PathStack;
use crate::pathslice::NewPathStack;
use crate::queron::Queron;
use crate::queron::query::{QueryStack, Query};
use crate::util::bounds::Offset;
use crate::widget::stack::QueryCurrentBounds;

use super::filter::{QueryVariant, QueryStdEventMode, StdEventMode};

#[derive(Clone)]
pub struct StdVariant<V,E> where V: Variant<E> + Clone + 'static, E: Env {
    pub variant: V,
    pub ts: u64,
    //pub filter_path: Option<PathSliceOwned>,
    //pub filter_path_strict: bool,
    pub direct_only: bool,
    pub filter_point: Option<Offset>,
    pub _p: PhantomData<E>,
}

impl<V,E> Debug for StdVariant<V,E> where V: Variant<E> + Clone + 'static, E: Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("StdVariant")
                .field("variant", &self.variant)
                .field("ts", &self.ts)
                //.field("filter_path", &self.filter_path) //TODO make PathResolvus debugable
                //.field("filter_path_strict", &self.filter_path_strict)
                .field("direct_only", &self.direct_only)
                .field("filter_point", &self.filter_point)
            .finish()
    }
}

impl<V,E> StdVariant<V,E> where V: Variant<E> + Clone + 'static, E: Env { 
    /// nofilter
    #[inline]
    pub fn new(variant: V, ts: u64) -> Self {
        Self { variant, ts, filter_point: None, direct_only: false, _p: PhantomData }
    }
    // #[inline]
    // pub fn with_filter_path(mut self, filter_path: PathSliceOwned) -> Self {
    //     self.filter_path = Some(filter_path);
    //     self.filter_path_strict = false;
    //     self
    // }
    // #[inline]
    // pub fn with_filter_path_strict(mut self, filter_path: PathSliceOwned) -> Self {
    //     self.filter_path = Some(filter_path);
    //     self.filter_path_strict = true;
    //     self
    // }
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

impl<V,E> super::Event<E> for StdVariant<V,E> where V: Variant<E> + Clone + 'static, E: Env {
    type WithPrefetch<R> = R where R: Queron<E>;

    fn _query<'a,Q,S>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, path: &mut NewPathStack, stack: &S) where S: Queron<E> + ?Sized, Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryVariant<V>>() {
            *builder = Some(&self.variant);
        } else if let Some((_,builder)) = builder.downcast::<'_,QueryStdEventMode>() {
            let mut receive_self = true;
            let mut route_to_childs = true; // !self.direct_only; //TODO fix route_to_path_strict

            let mut child_filter_point = None;
            //let mut child_filter_absolute_path = None;

            // if let Some(filter_path) = &self.filter_path {
            //     let current_path = path;
            //     match path.fwd_compare(&**filter_path) {
            //         FwdCompareStat::ChildOfSelf {
            //             // currently in parent of route-to widget
            //             receive_self = false;
            //             //child_filter_sub_path = Some(self_in_parent);
            //         },
            //         FwdCompareStat::Equal => {
            //             if self.filter_path_strict {
            //                 route_to_childs = false;
            //             }
            //         },
            //         FwdCompareStat::ParentOfSelf(_) => {
            //             if self.filter_path_strict {
            //                 receive_self = false;
            //                 route_to_childs = false;
            //                 debug_assert!(false, "Misrouted");
            //             }
            //         },
            //         FwdCompareStat::Falsified => {
            //             // misrouted
            //             receive_self = false;
            //             route_to_childs = false;
            //             //debug_assert!(false, "Misrouted");
            //         },
            //     }
            // }

            if let Some(filter_point) = self.filter_point {
                let current_bounds = QueryCurrentBounds.query_in(stack).unwrap();

                if filter_point.is_inside(current_bounds.bounds) { //TODO also check viewport
                    child_filter_point = Some(filter_point);
                } else {
                    receive_self = false;
                    route_to_childs = false;
                    //TODO check for misroute
                }
            }

            let mut mode = StdEventMode {
                receive_self,
                route_to_childs,
                child_filter_point,
                childs_after_resolve: !self.direct_only,
                //child_filter_sub_path,
                _p: PhantomData,
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

    fn _debug(&self) -> &dyn Debug {
        self
    }

    fn _as_any(&self) -> &dyn std::any::Any {
        self
    }
}
