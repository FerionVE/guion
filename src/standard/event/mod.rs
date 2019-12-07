use crate::standard::event::kbd::KbdState;
use crate::standard::event::mouse::MouseState;
use crate::core::ctx::Context;

pub mod mouse;
pub mod kbd;

pub struct StandardDriver<E> where E: Context {
    m: MouseState<E>,
    k: KbdState<E>,
}