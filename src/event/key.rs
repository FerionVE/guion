use super::*;
use std::fmt::Debug;

pub trait PressedKey<E> where E: Env {
    fn key(&self) -> EEKey<E>;
    /// the widget at which the keypress started
    fn widget(&self) -> WidgetIdent<E>;
    /// the timestamp at which the keypress started
    fn timestamp(&self) -> u64;
    fn cursor(&self) -> Option<Offset>;
}

pub trait Key: Clone + PartialEq + Debug {
    type Origin;
    const MOUSE_LEFT: Self;
    const ENTER: Self;
    const SPACE: Self;
    const TAB: Self;
    const BACKSPACE: Self;

    fn origin(&self) -> Self::Origin;
}