use crate::env::Env;
use crate::event::variant::Variant;
use crate::queron::Queron;
use crate::queron::query::QueryStack;
use crate::util::bounds::Offset;

use super::filter::{QueryVariant, StdEventMode, QueryStdEventMode};

pub struct StdVariant<V,E> where V: Variant<E> + Clone, E: Env {
    pub variant: V,
    pub ts: u64,
    pub filter_path: Option<E::WidgetPath>,
    pub filter_point: Option<Offset>,
}

impl<V,E> StdVariant<V,E> where V: Variant<E> + Clone, E: Env { 
    /// nofilter
    #[inline]
    pub fn new(variant: V, ts: u64) -> Self {
        Self { variant, ts, filter_path: None, filter_point: None }
    }
    #[inline]
    pub fn with_filter_path(mut self, filter_path: E::WidgetPath) -> Self {
        self.filter_path = Some(filter_path);
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
            let mut mode = StdEventMode {
                receive_self: todo!(),
                route_to_childs: todo!(),
                child_filter_point: todo!(),
                child_filter_path: todo!(),
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