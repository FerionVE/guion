use super::*;
use std::fmt::Debug;

pub trait PressedKey<E> where E: Env {
    fn key(&self) -> EEKey<E>;
    /// the widget at which the keypress started
    fn widget(&self) -> E::WidgetPath;
    /// the timestamp at which the keypress started
    fn ts(&self) -> u64;
    fn cursor(&self) -> Option<Offset>;
}

pub trait Key: Clone + PartialEq + Debug {
    type Origin;
    const MOUSE_LEFT: Self;
    const ENTER: Self;
    const SPACE: Self;
    const TAB: Self;
    const SHIFT: Self;
    const CTRL: Self;
    const BACKSPACE: Self;
    const LEFT: Self;
    const RIGHT: Self;
    const UP: Self;
    const DOWN: Self;
    const A: Self;
    const X: Self;
    const C: Self;
    const V: Self;

    fn origin(&self) -> Self::Origin;
}
