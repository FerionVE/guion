use crate::core::*;
use crate::standard::state::mouse::drag::DragItem;
use util::bounds::Offset;
use ctx::Env;

pub mod kbd;
pub mod mouse;
pub mod text_box;

pub struct State<E> where E: Env {
    pub pressed: Vec<PressedKey<E>>,
    pub mouse: Option<DragItem<E>>,
    pub drag: Option<Offset>,
}

pub struct PressedKey<E> where E: Env {
    pub key: u32,
    pub start_cursor: Option<Offset>,
    pub id: E::WidgetID,
    pub ts: u64,
}