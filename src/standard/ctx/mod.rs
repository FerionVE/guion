use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use ctx::*;

pub mod imp;

pub struct StandardCtx<S,C> where S: Handler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> {
    pub sup: S,
    //pub selected: Option<S>,
    _c: PhantomData<C>,
}