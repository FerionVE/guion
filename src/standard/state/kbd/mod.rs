use crate::core::*;
use ctx::*;
use ctx::aliases::*;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub pressed: Vec<KbdPressedKey<E>>,
}

pub struct KbdPressedKey<E> where E: Env {
    pub key: EKey<E>,
    ///the widget which was selected (focused) where the key press started
    pub id: E::WidgetID,
    ///the time the key press started
    pub ts: u64,
}

impl<E> KbdState<E> where E: Env {
    #[inline]
    pub fn down(&mut self, key: EKey<E>, id: E::WidgetID, ts: u64) {
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
    pub fn up(&mut self, key: EKey<E>) {
        self.pressed.retain(#[inline] |e| e.key != key );
    }
    #[inline]
    pub fn get(&self, key: EKey<E>) -> Option<&KbdPressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: EKey<E>) -> Option<&mut KbdPressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }
}