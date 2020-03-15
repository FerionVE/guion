use crate::core::*;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub focused: Option<E::WidgetPath>,
}

impl<E> KbdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            focused: None,
        }
    }
}
