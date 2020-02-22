use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;

pub mod imp;

pub struct StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> {
    pub sup: S,
    //pub selected: Option<S>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            _c: PhantomData,
        }
    }
}