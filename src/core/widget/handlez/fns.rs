use crate::core::ctx::*;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;

pub struct WidgetFns<E> where E: Env {
    pub render: fn(Link<E>, E::Renderer),
    pub event: fn(Link<E>, E::Event),
    pub size: fn(Link<E>) -> Size,
}