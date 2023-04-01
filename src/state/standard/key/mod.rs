use std::mem::size_of;
use std::sync::Arc;

use crate::aliases::EEKey;
use crate::env::Env;
use crate::event::key::PressedKey;
use crate::newpath::PathResolvusDyn;
use crate::util::bounds::Offset;
use crate::widget::id::WidgetID;

pub struct KeyState<E> where E: Env {
    pub pressed: Vec<StdPressedKey<E>>,
}

pub struct StdPressedKey<E> where E: Env {
    pub key: EEKey<E>,
    ///the widget which was selected (focused) where the key press started
    pub down: (Arc<dyn PathResolvusDyn<E>>,WidgetID),
    ///the time the key press started
    pub ts: u64,
    pub cursor: Option<Offset>,
}

impl<E> KeyState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: EEKey<E>, down: (Arc<dyn PathResolvusDyn<E>>,WidgetID), ts: u64, cursor: Option<Offset>) -> Option<StdPressedKey<E>> {
        let old = self.up(key.clone());
        self.pressed.push(
            StdPressedKey{
                key,
                down,
                ts,
                cursor,
            }
        );
        old
    }
    #[inline]
    pub fn up(&mut self, key: EEKey<E>) -> Option<StdPressedKey<E>> {
        //self.pressed.retain(#[inline] |e| e.key != key );
        for (i,k) in self.pressed.iter().enumerate() {
            if k.key == key {
                return Some(self.pressed.remove(i));
            }
        }
        None
    }
    #[inline]
    pub fn get(&self, key: EEKey<E>) -> Option<&StdPressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: EEKey<E>) -> Option<&mut StdPressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }

    pub fn new() -> Self {
        Self{
            pressed: Vec::with_capacity(4096 / size_of::<StdPressedKey<E>>()),
        }
    }
}

impl<E> PressedKey<E> for StdPressedKey<E> where E: Env {
    #[inline]
    fn key(&self) -> EEKey<E> {
        self.key.clone()
    }
    #[inline]
    fn widget(&self) -> (&(dyn PathResolvusDyn<E>+'_),WidgetID) {
        (&*self.down.0, self.down.1)
    }
    #[inline]
    fn ts(&self) -> u64 {
        self.ts
    }
    #[inline]
    fn cursor(&self) -> Option<Offset> {
        self.cursor.clone()
    }
}
