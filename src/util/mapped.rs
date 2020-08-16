use super::*;
use std::marker::PhantomData;

/// Wrapper type for stateless mapping
#[repr(transparent)]
pub struct Mapped<E,T,U,FRef,FMut,FInto> where FRef: for<'a> MapFun<&'a T,&'a U> {
    inner: T,
    _p: PhantomData<(E,U,FRef,FMut,FInto)>,
}

pub trait MapFun<T,U> {
    fn mapfun(v: T) -> U;
}
