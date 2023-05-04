use std::marker::PhantomData;
use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::pathslice::PathSliceOwned;
use crate::util::bounds::Offset;
use crate::widget::id::WidgetID;

use self::drag::DragItem;

pub mod drag;

pub struct MouseState<E> where E: Env {
    pub drag: Option<DragItem>,
    pub pos: Option<Offset>,
    pub hovered: Option<(PathSliceOwned,WidgetID)>,
    pub prev_hovered: Option<(PathSliceOwned,WidgetID)>,
    pub hover_last_seen: Option<WidgetID>,
    _p: PhantomData<E>,
}

impl<E> MouseState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            drag: None,
            pos: None,
            hovered: None,
            prev_hovered: None,
            hover_last_seen: None,
            _p: PhantomData,
        }
    }
}
