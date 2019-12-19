use crate::core::*;
use lazout::size::Size;
use widget::link::Link;
use ctx::*;

pub trait IHandler<E> where E: Env {
    fn render(l: Link<E>, r: E::Renderer);
    fn event(l: Link<E>, e: E::Event);
    fn size(l: Link<E>) -> Size;
}