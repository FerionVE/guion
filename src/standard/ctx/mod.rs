use std::marker::PhantomData;
use crate::core::ctx::ContextLayer;
use crate::core::ctx::Context;

pub mod imp;

pub struct StandardCtx<S,E> where S: ContextLayer<E>, E: Context {
    pub sup: S,
    _e: PhantomData<E>,
}