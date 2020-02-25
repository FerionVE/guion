use crate::core::*;
use crate::standard::state::mouse::drag::DragItem;
use util::bounds::Offset;

pub mod drag;

pub struct MouseState<E> where E: Env {
    pub pressed: Vec<MousePressedKey<E>>,
    pub drag: Option<DragItem<E>>,
    pub pos: Option<Offset>,
    pub hovered: Option<E::WidgetPath>,
}

pub struct MousePressedKey<E> where E: Env {
    pub key: u32,
    pub start: Offset,
    pub id: E::WidgetID,
    pub ts: u64,
}

impl<E> MouseState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: u32, start: Offset, id: E::WidgetID, ts: u64) {
        self.up(key);
        self.pressed.push(
            MousePressedKey{
                key,
                start,
                id,
                ts,
            }
        );
    }
    #[inline]
    pub fn up(&mut self, key: u32) {
        self.pressed.retain(#[inline] |e| e.key != key );
    }
    #[inline]
    pub fn get(&self, key: u32) -> Option<&MousePressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: u32) -> Option<&mut MousePressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }

    pub fn new() -> Self {
        Self{
            pressed: Vec::new(),
            drag: None,
            pos: None,
            hovered: None,
        }
    }
}