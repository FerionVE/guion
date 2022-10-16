use crate::env::Env;
use crate::queron::Queron;
use crate::queron::query::Query;
use crate::util::bounds::Bounds;

use super::Widget;

pub struct WithCurrentBounds<S> {
    pub inner: S,
    pub bounds: Bounds,
    pub viewport: Bounds,
}

impl<S,E> Queron<E> for WithCurrentBounds<S> where S: Queron<E>, E: Env {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: crate::queron::query::QueryStack<'_,'a,Q,E>) where Self: 'a {
        if let Some((_,builder)) = builder.downcast::<'_,QueryCurrentBounds>() {
            *builder = Some(QueriedCurrentBounds{
                bounds: &self.bounds,
                viewport: &self.viewport,
            })
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
pub struct QueriedCurrentBounds<'a> {
    pub bounds: &'a Bounds,
    pub viewport: &'a Bounds,
}

#[derive(Clone)]
pub struct QueryCurrentBounds;

impl<E> Query<E> for QueryCurrentBounds where E: Env {
    type Out<'b> = QueriedCurrentBounds<'b>;
    type Builder<'b> = Option<QueriedCurrentBounds<'b>>;

    #[inline]
    fn new_builder<'b>(&self) -> Self::Builder<'b> {
        None
    }
    #[inline]
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>> {
        b
    }
}
