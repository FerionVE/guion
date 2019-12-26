use crate::core::ctx::aliases::*;
use crate::core::*;
use ctx::*;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub pressed: Vec<KbdPressedKey<E>>,
}

pub struct KbdPressedKey<E> where E: Env {
    pub key: EEventKey<E>,
    ///the widget which was selected (focused) where the key press started
    pub id: E::WidgetID,
    ///the time the key press started
    pub ts: u64,
}

impl<E> KbdState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: EEventKey<E>, id: E::WidgetID, ts: u64) {
        self.up(key.clone());
        self.pressed.push(
            KbdPressedKey{
                key,
                id,
                ts,
            }
        );
    }
    #[inline]
    pub fn up(&mut self, key: EEventKey<E>) {
        self.pressed.retain(#[inline] |e| e.key != key );
    }
    #[inline]
    pub fn get(&self, key: EEventKey<E>) -> Option<&KbdPressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: EEventKey<E>) -> Option<&mut KbdPressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }
}