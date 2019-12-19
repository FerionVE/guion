use crate::core::*;
use event::Variant;
use util::bounds::Offset;
use ctx::*;
use ctx::aliases::*;
use event::key::Key;

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
}
#[derive(Clone)]
pub struct MouseUp<K> where K: Key {
    pub key: K,
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

impl<E> Variant<E> for KbdDown<EKey<E>> where E: Env {}
impl<E> Variant<E> for KbdUp<EKey<E>> where E: Env {}
impl<E> Variant<E> for MouseDown<EKey<E>> where E: Env {}
impl<E> Variant<E> for MouseUp<EKey<E>> where E: Env {}
impl<E> Variant<E> for MouseMove where E: Env {}
impl<E> Variant<E> for MouseEnter where E: Env {}
impl<E> Variant<E> for MouseLeave where E: Env {}

