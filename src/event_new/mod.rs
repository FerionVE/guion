use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::{PathStack, PathStackDyn};
use crate::queron::Queron;
use crate::queron::dyn_tunnel::QueronDyn;
use crate::queron::query::{Query, QueryStack, DynQuery};

use self::filter::{StdEventMode, QueryStdEventMode, QueryVariant};

pub mod filter;
pub mod variants;
pub mod downcast_map;

pub trait Event<E> where E: Env {
    /// Prefetch appended to stack
    type WithPrefetch<R>: Queron<E> where R: Queron<E>;

    #[inline]
    fn query<'a,Q,Ph,S>(&'a self, query: &Q, path: &Ph, stack: &S) -> Option<Q::Out<'a>> where Q: Query<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Self: 'a {
        let mut builder = query.new_builder();
        let qstack = QueryStack::new(query, &mut builder);
        self._query(qstack,path,stack);
        query.end_builder(builder)
    }

    fn _query<'a,Q,Ph,S>(&'a self, builder: QueryStack<'_,'a,Q,E>, path: &Ph, stack: &S) where Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Self: 'a;

    //TODO move to QBase
    fn erase<'s,'ss>(&'s self) -> &'s (dyn EventDyn<E>+'ss) where 'ss: 's, Self: 'ss;

    #[deprecated]
    #[inline]
    fn query_std_event_mode<'a,Ph,S>(&'a self, path: &Ph, stack: &S) -> Option<StdEventMode<E>> where Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Self: 'a {
        self.query(&QueryStdEventMode, path, stack)
    }

    /// query legacy variant
    #[deprecated="The old \"variants\" will be replaced"]
    #[inline]
    fn query_variant<'a,V,Ph,S>(&'a self, path: &Ph, stack: &S) -> Option<&'a V> where Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, V: Clone + 'static, Self: 'a {
        self.query(&QueryVariant(PhantomData), path, stack)
    } //TODO can we turn path and stack into impl Trait instead of generics to reduce the '_,'_>ness

    #[deprecated]
    /// Timestamp
    fn ts(&self) -> u64;

    #[deprecated]
    fn _root_only(&self) -> bool;

    /// Append prefetch to stack
    fn with_prefetch<R>(&self, stack: R) -> Self::WithPrefetch<R> where R: Queron<E>;

    fn _debug(&self) -> &dyn Debug;

    fn _as_any(&self) -> &dyn Any; //TODO proper non-'static downcast
}

/// This trait is only for bridging thru trait objects
pub trait EventDyn<E> {
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery,E>, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_));
    fn ts_dyn(&self) -> u64;
    fn _root_only_dyn(&self) -> bool;
    fn _debug_dyn(&self) -> &dyn Debug;
    fn _as_any_dyn(&self) -> &dyn Any;
}
impl<T,E> EventDyn<E> for T where T: Event<E> + ?Sized, E: Env {
    #[inline]
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery,E>, path: &(dyn PathStackDyn<E>+'_), stack: &(dyn QueronDyn<E>+'_)) {
        self._query(builder,path,stack)
    }
    #[inline]
    fn ts_dyn(&self) -> u64 {
        self.ts()
    }
    #[inline]
    fn _root_only_dyn(&self) -> bool {
        self._root_only()
    }

    fn _debug_dyn(&self) -> &dyn Debug {
        self._debug()
    }

    fn _as_any_dyn(&self) -> &dyn Any {
        self._as_any()
    }
}

/// The call into dyn querylon stack, the last static propagation
impl<E> Event<E> for dyn EventDyn<E> + '_ where E: Env {
    type WithPrefetch<R> = R where R: Queron<E>;

    #[inline]
    fn _query<'a,Q,Ph,S>(&'a self, mut builder: QueryStack<'_,'a,Q,E>, path: &Ph, stack: &S) where Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Self: 'a {
        self._query_dyn(builder.fork_dyn(),path._erase(),stack.erase())
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
    #[inline]
    fn _root_only(&self) -> bool {
        self._root_only_dyn()
    }

    fn _debug(&self) -> &dyn Debug {
        self._debug_dyn()
    }

    fn _as_any(&self) -> &dyn Any {
        self._as_any_dyn()
    }
}
