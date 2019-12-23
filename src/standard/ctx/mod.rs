use crate::core::*;
use std::marker::PhantomData;
use ctx::Handler;
use ctx::*;

pub mod imp;

pub struct StandardCtx<S,C> where S: Handler<C>, C: Context, C::Handler: AsHandler<Self,C> {
    pub sup: S,
    //pub selected: Option<S>,
    _c: PhantomData<C>,
}