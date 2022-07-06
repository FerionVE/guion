use super::*;

/// This trait is only for bridging thru trait objects
pub trait QueronDyn<E> {
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery>);
}
impl<T,E> QueronDyn<E> for T where T: Queron<E> + ?Sized {
    fn _query_dyn<'a>(&'a self, builder: QueryStack<'_,'a,DynQuery>) {
        self._query(builder)
    }
}

/// The call into dyn querylon stack, the last static propagation
impl<E> Queron<E> for dyn QueronDyn<E> + '_ {
    #[inline]
    fn _query<'a,Q>(&'a self, mut builder: QueryStack<'_,'a,Q>) where Self: 'a {
        // Using a hidden optimizable cache right here.
        // The compiler must be able to completely optimize away cache accesses and state (doesn't work with maps or vecs, may work with fixed arrays)
        // At best it's also conditionally compiled on used optimizations (whether it would actually optimize away)
        // On hit it would replace the builder in b with the cached builder
        self._query_dyn(builder.fork_dyn())
    }
    #[inline]
    fn erase<'s,'ss>(&'s self) -> &'s (dyn QueronDyn<E>+'ss) where 'ss: 's, Self: 'ss {
        self
    }
}
