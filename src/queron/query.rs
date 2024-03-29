use std::any::TypeId;
use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Queron;

//TODO Builder with lifetime
pub trait Query<E>: Clone + 'static where E: 'static {
    type Out<'b>: 'b;
    type Builder<'b>: Clone + 'b;

    fn query_in<'b,S>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
        self._query_direct(stack)
    }

    fn _query_direct<'b,S>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
        let mut builder = self.new_builder();
        let qstack = QueryStack::new(self, &mut builder);
        stack._query(qstack);
        self.end_builder(builder)
    }

    fn new_builder<'b>(&self) -> Self::Builder<'b>;
    fn end_builder<'b>(&self, b: Self::Builder<'b>) -> Option<Self::Out<'b>>;
}

/// Dummy type for marking QueryStack with dyn query type
pub enum DynQuery {}

// 'b MUST be invariant
pub struct QueryStack<'a,'b,Q,E> where Q: ?Sized + 'static, E: 'static, 'b: 'a {
    type_id: TypeId,
    query: NonNull<()>,
    builder: NonNull<()>,
    _p: PhantomData<(&'a mut &'b mut Q,E)>,
}

impl<'a,'b,Q,E> QueryStack<'a,'b,Q,E> where Q: ?Sized + 'static, E: 'static, 'b: 'a {
    #[inline(always)]
    pub fn new(query: &'a Q, builder: &'a mut Q::Builder<'b>) -> Self where Q: Query<E>, 'b: 'a {
        Self {
            type_id: TypeId::of::<Q>(),
            query: NonNull::<Q>::from(query).cast::<()>(),
            builder: NonNull::<Q::Builder<'b>>::from(builder).cast::<()>(),
            _p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn current_type_id(&self) -> TypeId {
        if TypeId::of::<Q>() == TypeId::of::<DynQuery>() {
            self.type_id
        } else {
            // if TypeId::of::<Q>() != self.type_id {
            //     unsafe { std::hint::unreachable_unchecked() }
            // }
            TypeId::of::<Q>()
        }
    }

    #[inline(always)]
    pub fn downcast<'s,T>(&'s mut self) -> Option<(&'s T,&'s mut T::Builder<'b>)> where T: Query<E>, 'b: 's {
        if TypeId::of::<T>() == self.current_type_id() {
            Some(unsafe{
                (
                    self.query.cast::<T>().as_ref(),
                    self.builder.cast::<T::Builder<'b>>().as_mut(),
                )
            })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn fork<'s>(&'s mut self) -> QueryStack<'s,'b,Q,E> where 'b: 's {
        debug_assert!(self.type_id == self.current_type_id());
        QueryStack {
            type_id: self.current_type_id(),
            query: self.query,
            builder: self.builder,
            _p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn fork_dyn<'s>(&'s mut self) -> QueryStack<'s,'b,DynQuery,E> where 'b: 's {
        debug_assert!(self.type_id == self.current_type_id());
        QueryStack {
            type_id: self.current_type_id(),
            query: self.query,
            builder: self.builder,
            _p: PhantomData,
        }
    }
}
