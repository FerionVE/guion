use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;

pub struct KbdState<E> where E: Env {
    pub focused: Option<Arc<dyn PathResolvusDyn<E>>>,
}

impl<E> KbdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            focused: None,
        }
    }
}
