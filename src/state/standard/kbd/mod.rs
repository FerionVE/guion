use std::marker::PhantomData;
use std::sync::Arc;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::pathslice::PathSliceOwned;
use crate::widget::id::WidgetID;

pub struct KbdState<E> where E: Env {
    pub focused: Option<(PathSliceOwned,WidgetID)>,
    _p: PhantomData<E>,
}

impl<E> KbdState<E> where E: Env {
    pub fn new() -> Self {
        Self{
            focused: None,
            _p: PhantomData,
        }
    }
}
