use crate::core::ctx::Context;

pub mod tabulate;

pub struct KbdState<E> where E: Context {
    pub pressed: Vec<KbdPressedKey<E>>,
}

pub struct KbdPressedKey<E> where E: Context {
    pub key: u32,
    pub id: E::WidgetID,
    pub ts: u64,
}

impl<E> KbdState<E> where E: Context {
    #[inline]
    pub fn down(&mut self, key: u32, id: E::WidgetID, ts: u64) {
        self.up(key);
        self.pressed.push(
            KbdPressedKey{
                key,
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
    pub fn get(&self, key: u32) -> Option<&KbdPressedKey<E>> {
        self.pressed.iter().find(#[inline] |i| i.key == key )
    }
    #[inline]
    pub fn get_mut(&mut self, key: u32) -> Option<&mut KbdPressedKey<E>> {
        self.pressed.iter_mut().find(#[inline] |i| i.key == key )
    }
}