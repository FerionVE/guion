use crate::*;
use key::*;
use kbd::*;
use mouse::*;
use std::{any::TypeId, collections::HashMap};

pub mod key;
pub mod kbd;
pub mod mouse;

pub struct StdState<E> where E: Env {
    pub key: KeyState<E>,
    pub kbd: KbdState<E>,
    pub mouse: MouseState<E>,
    pub remote_states: HashMap<(E::WidgetID,TypeId),Box<dyn Any>>,
}

impl<E> StdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            key: KeyState::new(),
            kbd: KbdState::new(),
            mouse: MouseState::new(),
            remote_states: HashMap::new(),
        }
    }
}