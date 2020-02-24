use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use super::state::StdState;

pub mod imp;

pub struct StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> {
    pub sup: S,
    pub s: StdState<E>,
    _c: PhantomData<E>,
}

impl<S,E> StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> {
    pub fn new(sup: S) -> Self {
        Self{
            sup,
            s: StdState::new(),
            _c: PhantomData,
        }
    }
}