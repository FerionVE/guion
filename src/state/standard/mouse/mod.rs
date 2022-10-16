use std::sync::Arc;

use crate::*;
use crate::newpath::PathResolvusDyn;
use drag::*;
use util::bounds::Offset;

pub mod drag;

pub struct MouseState<E> where E: Env {
    pub drag: Option<DragItem<E>>,
    pub pos: Option<Offset>,
    pub hovered: Option<Arc<dyn PathResolvusDyn<E>>>,
}

impl<E> MouseState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            drag: None,
            pos: None,
            hovered: None,
        }
    }
}
