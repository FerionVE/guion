use crate::core::*;
use ctx::*;

pub trait PressedKey<E> where E: Env {
    fn key(&self) -> &E::EventDest;
    /// the widget at which the keypress started
    fn widget(&self) -> &E::WidgetID;
    /// the timestamp at which the keypress started
    fn timestamp(&self) -> u64;
}

pub trait Key: Clone + PartialEq {
    const MOUSE_LEFT: Self;
    const ENTER: Self;
    const TAB: Self;
}