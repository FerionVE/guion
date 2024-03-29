use crate::env::Env;

use self::kbd::KbdState;
use self::key::KeyState;
use self::mouse::MouseState;

pub mod key;
pub mod kbd;
pub mod mouse;

pub struct StdStdState<E> where E: Env {
    pub key: KeyState<E>,
    pub kbd: KbdState<E>,
    pub mouse: MouseState<E>,
    //pub remote_states: HashMap<(E::WidgetID,TypeId),Box<dyn Any>>,
}

impl<E> StdStdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            key: KeyState::new(),
            kbd: KbdState::new(),
            mouse: MouseState::new(),
            //remote_states: HashMap::new(),
        }
    }
}
