use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::ctx::Context;

pub trait IHandler<E> where E: Context {
    fn render(l: Link<E>, r: E::Renderer);
    fn event(l: Link<E>, e: E::Event);
    fn size(l: Link<E>) -> Size;
}