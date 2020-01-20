use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use ctx::*;

pub mod imp;

pub struct StandardCtx<S,E> where S: Handler<E>, E: Env, ECHandler<E>: AsHandler<Self,E> {
    pub sup: S,
    //pub selected: Option<S>,
    _c: PhantomData<E>,
}