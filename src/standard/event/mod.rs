use crate::standard::event::mouse::MouseState;
use crate::core::env::Env;

pub mod mouse;

pub struct StandardDriver<E> where E: Env {
    m: MouseState<E>,
}