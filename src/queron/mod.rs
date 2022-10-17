use self::dyn_tunnel::QueronDyn;
use self::query::{QueryStack, Query};

pub mod dyn_tunnel;
pub mod imp;
pub mod query;

/// This is the one trait everyone should use and/or implement
/// Both _query and _query_dyn should be implemented. query/_query should not call into _query_dyn, unless it's bridging into trait objects, because both stacks should run independently, until going into trait object
pub trait Queron<E> {
    #[inline]
    fn query<'a,Q>(&'a self, query: &Q) -> Option<Q::Out<'a>> where Q: Query<E> + ?Sized, Self: 'a {
        query.query_in(self)
    }

    fn _query<'a,Q>(&'a self, builder: QueryStack<'_,'a,Q,E>) where Self: 'a;

    //TODO move to QBase
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss;
}
