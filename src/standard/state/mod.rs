use crate::standard::state::kbd::KbdState;
use crate::standard::state::mouse::MouseState;
use crate::core::*;
use crate::standard::state::mouse::drag::DragItem;
use util::bounds::Offset;
use ctx::Env;

pub mod kbd;
pub mod mouse;
pub mod text_box;

pub struct State<E> where E: Env {
    pub kbd: KbdState<E>,
    pub mouse: MouseState<E>,
}