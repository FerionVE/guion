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
    pub key: EEKey<E>,
    pub start: Offset,
    pub id: E::WidgetPath,
    pub ts: u64,
}

impl<E> MouseState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: EEKey<E>, start: Offset, id: E::WidgetPath, ts: u64) -> Option<MousePressedKey<E>> {
        let old = self.up(key.clone());
        self.pressed.push(
            MousePressedKey{
                key,
                start,
                id,
                ts,
            }
        );
        old
    }
    #[inline]
    pub fn up(&mut self, key: EEKey<E>) -> Option<MousePressedKey<E>> {
        //self.pressed.retain(#[inline] |e| e.key != key );
        for (i,k) in self.pressed.iter().enumerate() {
            if k.key == key {
                return Some(self.pressed.remove(i));
            }
        }
        None
    }
    #[inline]
    pub fn get(&self, key: EEKey<E>) -> Option<&MousePressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: EEKey<E>) -> Option<&mut MousePressedKey<E>> {
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