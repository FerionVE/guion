use std::marker::PhantomData;
use crate::core::ctx::Handler;
use crate::core::ctx::*;

pub mod imp;

pub struct StandardCtx<S,D> where S: Handler<D> {
    pub sup: S,
    pub selected: Option<S>,
    _d: PhantomData<D>,
}