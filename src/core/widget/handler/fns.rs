use crate::core::ctx::Context;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;

pub struct HandlerFns<E> where E: Context {
    pub render: fn(Link<E>, E::Renderer),
    pub event: fn(Link<E>, E::Event),
    pub size: fn(Link<E>) -> Size,
}