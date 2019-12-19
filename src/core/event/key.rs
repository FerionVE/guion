use crate::core::*;
use ctx::id::WidgetID;
use ctx::*;
use ctx::aliases::*;

pub trait PressedKey<E> where E: Env {
    fn key(&self) -> &EKey<E>;
    fn widget(&self) -> &E::WidgetID;
    fn ts(&self) -> u64;
}

pub trait Key: Clone + PartialEq {
    const MOUSE_LEFT: Self;
    const ENTER: Self;
    const TAB: Self;
}