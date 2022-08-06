use crate::env::Env;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::util::bounds::{Bounds, Offset};

pub struct WithEventFilterPath<'a,S,E> where E: Env {
    pub inner: S,
    pub filter_path: &'a E::WidgetPath,
}

impl<'o,S,E> Queron<E> for WithEventFilterPath<'o,S,E> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: crate::queron::query::QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryEventFilterPath>() {
            *builder = Some(self.filter_path)
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn crate::queron::dyn_tunnel::QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[derive(Clone)]
pub struct QueryEventFilterPath;

impl<E> Query<E> for QueryEventFilterPath where E: Env {
    type Out<'b> = &'b E::WidgetPath;
    type Builder<'b> = Option<&'b E::WidgetPath>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}

pub struct WithEventFilterPos<S> {
    pub inner: S,
    pub filter_pos: Offset,
}

impl<S,E> Queron<E> for WithEventFilterPos<S> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, builder: crate::queron::query::QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryEventFilterPos>() {
            *builder = Some(self.filter_pos)
        } else {
            self.inner._query(builder)
        }
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn crate::queron::dyn_tunnel::QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}

#[derive(Clone)]
pub struct QueryEventFilterPos;

impl<E> Query<E> for QueryEventFilterPos where E: Env {
    type Out<'b> = Offset;
    type Builder<'b> = Option<Offset>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}
