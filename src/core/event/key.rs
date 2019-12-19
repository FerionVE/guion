use crate::core::*;
use ctx::id::WidgetID;

pub trait PressedKey<W> where W: WidgetID {
    type K: Key;

    fn key(&self) -> &Self::K;
    fn widget(&self) -> &W;
    fn ts(&self) -> u64;
}

pub trait Key: Clone + PartialEq {
    //TODO variant enum
    fn mouse_left() -> Self;
    fn enter() -> Self;
}