use crate::core::ctx::Context;

pub trait ContextMeta<E>: Sized where E: Context<Meta=Self> {
    fn default_style(&self) -> &E::Style;
}