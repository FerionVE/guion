use crate::standard::state::kbd::KbdState;
use crate::standard::state::mouse::MouseState;
use crate::core::*;
use key::KeyState;

pub mod key;
pub mod kbd;
pub mod mouse;
pub mod text_box;

pub struct StdState<E> where E: Env {
    pub key: KeyState<E>,
    pub kbd: KbdState<E>,
    pub mouse: MouseState<E>,
}

impl<E> StdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            key: KeyState::new(),
            kbd: KbdState::new(),
            mouse: MouseState::new(),
        }
    }
}