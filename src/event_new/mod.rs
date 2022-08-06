use std::marker::PhantomData;

use crate::env::Env;
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::{Query, QueryStack, DynQuery};

use self::filter::{StdEventMode, QueryStdEventMode, QueryVariant};

pub mod filter;
pub mod variants;

pub trait Event<E> where E: Env {
    /// Prefetch appended to stack
    type WithPrefetch<R>: Queron<E> where R: Queron<E>;

    #[inline]
    fn query<'a,Q,S>(&'a self, query: &Q, stack: &S) -> Option<Q::Out<'a>> where Q: Query<E> + ?Sized, S: Queron<E> + ?Sized, Self: 'a {
        let mut builder = query.new_builder();
        let qstack = QueryStack::new(query, &mut builder);
        stack._query(qstack);
        query.end_builder(builder)
    }

    fn _query<'a,Q,S>(&'a self, builder: QueryStack<'_,'a,Q,E>, stack: &S) where S: Queron<E> + ?Sized, Self: 'a;

    //TODO move to QBase
    fn erase<'s,'ss>(&'s self) -> &'s (dyn EventDyn<E>+'ss) where 'ss: 's, Self: 'ss;

    #[deprecated]
    #[inline]
    fn query_std_event_mode<'a,S>(&'a self, stack: &S) -> Option<&'a StdEventMode<E>> where S: Queron<E> + ?Sized, Self: 'a {
        self.query(&QueryStdEventMode, stack)
    }

    #[deprecated]
    #[inline]
    fn query_variant<'a,V,S>(&'a self, stack: &S) -> Option<&'a StdEventMode<E>> where S: Queron<E> + ?Sized, Self: 'a {
        self.query(&QueryVariant(PhantomData), stack)
    }

    #[deprecated]
    /// Timestamp
    fn ts(&self) -> u64;

    /// Append prefetch to stack
    fn with_prefetch<R>(&self, stack: R) -> Self::WithPrefetch<R> where R: Queron<E>;
}

/// This trait is only for bridging thru trait objects
pub trait EventDyn<E> {
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery,E>, stack: &dyn QueronDyn<E>);
    fn ts_dyn(&self) -> u64;
}
impl<T,E> EventDyn<E> for T where T: Event<E> + ?Sized, E: Env {
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery,E>, stack: &dyn QueronDyn<E>) {
        self._query(builder,stack)
    }
    fn ts_dyn(&self) -> u64 {
        self.ts()
    }
}

/// The call into dyn querylon stack, the last static propagation
impl<E> Event<E> for dyn EventDyn<E> + '_ where E: Env {
    type WithPrefetch<R> = R where R: Queron<E>;

    #[inline]
    fn _query<'a,Q,S>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, stack: &S) where S: Queron<E> + ?Sized, Self: 'a {
        self._query_dyn(builder.fork_dyn(),stack.erase())
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn EventDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
    #[inline]
    fn ts(&self) -> u64 {
        self.ts_dyn()
    }
    #[inline]
    fn with_prefetch<R>(&self, stack: R) -> Self::WithPrefetch<R> where R: Queron<E> {
        stack
    }
}
