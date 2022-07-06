use std::ptr::NonNull;

use super::*;

//TODO Builder with lifetime
pub trait Query: Clone + 'static {
    type Out<'b>: 'b;
    type Builder<'b>: Clone + 'b;

    fn query_in<'b,S,E>(&self, stack: &'b S) -> Option<Self::Out<'b>> where S: Queron<E> + ?Sized + 'b {
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
pub struct QueryStack<'a,'b,Q> where Q: ?Sized + 'static, 'b: 'a {
    type_id: TypeId,
    query: NonNull<()>,
    builder: NonNull<()>,
    _p: PhantomData<&'a mut &'b Q>,
}

impl<'a,'b,Q> QueryStack<'a,'b,Q> where Q: ?Sized + 'static, 'b: 'a {
    #[inline(always)]
    pub fn new(query: &'a Q, builder: &'a mut Q::Builder<'b>) -> Self where Q: Query, 'b: 'a {
        Self {
            type_id: TypeId::of::<Q>(),
            query: NonNull::<Q>::from(query).cast::<()>(),
            builder: NonNull::<Q::Builder<'b>>::from(builder).cast::<()>(),
            _p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn is_dyn(&self) -> bool {
        TypeId::of::<Q>() == TypeId::of::<DynQuery>()
    }

    #[inline(always)]
    pub fn downcast<'s,T>(&'s mut self) -> Option<(&'s T,&'s mut T::Builder<'b>)> where T: Query, 'b: 's {
        let matches = if TypeId::of::<Q>() == TypeId::of::<DynQuery>() {
            TypeId::of::<T>() == self.type_id
        } else {
            TypeId::of::<T>() == TypeId::of::<Q>()
        };

        if matches {
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
    pub fn fork<'s>(&'s mut self) -> QueryStack<'s,'b,Q> where 'b: 's {
        QueryStack {
            type_id: self.type_id,
            query: self.query,
            builder: self.builder,
            _p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn fork_dyn<'s>(&'s mut self) -> QueryStack<'s,'b,DynQuery> where 'b: 's {
        QueryStack {
            type_id: self.type_id,
            query: self.query,
            builder: self.builder,
            _p: PhantomData,
        }
    }
}
