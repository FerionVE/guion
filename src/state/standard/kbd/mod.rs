use crate::*;

pub mod tabulate;

pub struct KbdState<E> where E: Env {
    pub focused: Option<WidgetIdent<E>>,
}

impl<E> KbdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            focused: None,
        }
    }
}
