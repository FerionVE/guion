use crate::standard::ctx::imp::StandardCtxEnv;
use crate::standard::event::StandardDriver;
use crate::core::env::*;

pub mod imp;

pub struct StandardCtx<E> where E: Env {
    pub sup: E::Ctx,
    drv: StandardDriver<StandardCtxEnv<E>>,
}