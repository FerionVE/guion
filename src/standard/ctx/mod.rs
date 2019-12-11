use crate::core::ctx::Context;

pub mod imp;

pub struct StandardCtx<E> where E: Context {
    pub sup: E,
}