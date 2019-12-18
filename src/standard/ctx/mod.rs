use std::marker::PhantomData;
use crate::core::ctx::Handler;
use crate::core::ctx::*;

pub mod imp;

pub struct StandardCtx<S,C> where S: Handler<C>, C: Context, C::Link: AsMut<Self> + AsMut<S> {
    pub sup: S,
    pub selected: Option<S>,
    _c: PhantomData<C>,
}