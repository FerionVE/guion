use crate::core::lazout::size::Size;
use crate::core::*;
use event::Variant;
use util::bounds::Offset;
use ctx::*;
use event::key::Key;
use super::*;

#[derive(Clone)]
pub struct KbdDown<K> where K: Key {
    pub key: K,
}
#[derive(Clone)]
pub struct KbdUp<K> where K: Key {
    pub key: K,
}

#[derive(Clone)]
pub struct MouseDown<K> where K: Key {
    pub key: K,
    pub pos: Offset,
}
#[derive(Clone)]
pub struct MouseUp<K> where K: Key {
    pub key: K,
    pub pos: Offset,
}

#[derive(Clone)]
pub struct MouseMove {
    pub dest: Offset,
}

#[derive(Clone)]
pub struct MouseEnter {
    pub dest: Offset,
}
#[derive(Clone)]
pub struct MouseLeave {
    pub dest: Offset,
}

#[derive(Clone)]
pub struct WindowMove {
    pub pos: Offset,
}

#[derive(Clone)]
pub struct WindowResize {
    pub size: Size,
}

macro_rules! consuming {
    () => {
        #[inline]
        fn consuming(&self) -> bool {
            true
        }
    };
}

macro_rules! selected {
    () => {
        #[inline]
        fn destination(&self) -> E::EventDest {
            Destination::SELECTED
        }
    };
}

macro_rules! root {
    () => {
        #[inline]
        fn destination(&self) -> E::EventDest {
            Destination::ROOT
        }
    };
}

impl<E> Variant<E> for KbdDown<E::EventKey> where E: Env {selected!();}
impl<E> Variant<E> for KbdUp<E::EventKey> where E: Env {selected!();}

impl<E> Variant<E> for MouseDown<E::EventKey> where E: Env {consuming!();root!();}
impl<E> Variant<E> for MouseUp<E::EventKey> where E: Env {consuming!();root!();}
impl<E> Variant<E> for MouseMove where E: Env {consuming!();root!();}
impl<E> Variant<E> for MouseEnter where E: Env {consuming!();root!();}
impl<E> Variant<E> for MouseLeave where E: Env {consuming!();root!();}