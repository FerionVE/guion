use crate::core::ctx::*;

pub trait ContextMeta<C,E>: Sized where E: Env<Context=C>, C: Context<Meta=Self> {
    fn default_style(&self) -> &E::Style;
}