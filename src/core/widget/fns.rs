use crate::core::util::bounds::Bounds;
use crate::core::*;
use ctx::*;
use lazout::size::Size;
use widget::link::Link;

pub struct WidgetFns<E> where E: Env {
    pub render: fn(Link<E>, (&mut E::Renderer,&Bounds)),
    pub event: fn(Link<E>, E::Event),
    pub size: fn(Link<E>) -> Size,
}