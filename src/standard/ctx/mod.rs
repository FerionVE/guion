use std::marker::PhantomData;
use crate::core::ctx::Handler;
use crate::core::ctx::*;

pub mod imp;

pub struct StandardCtx<S> where S: Handler {
    pub sup: S,
}