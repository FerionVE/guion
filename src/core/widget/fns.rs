use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use ctx::*;
use lazout::size::Size;
use widget::link::Link;

pub struct WidgetFns<E> where E: Env {
    pub render: fn(Link<E>, (&mut ERenderer<E>,&Bounds)),
    pub event: fn(Link<E>, EEvent<E>),
    pub size: fn(Link<E>) -> Size,
}