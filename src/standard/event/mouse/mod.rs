use crate::core::util::bounds::Offset;
use crate::standard::event::mouse::drag::DragItem;
use crate::core::env::Env;

pub mod drag;

pub struct MouseState<E> where E: Env {
    pressed: Vec<u32>,
    drag: Option<DragItem<E>>,
    pos: Option<Offset>,
}