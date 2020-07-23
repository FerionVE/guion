use super::*;
use std::{marker::PhantomData};

use calc::calc_bounds;

pub mod imp;

pub struct Pane<'w,T,E> where E: Env, T: 'w {
    id: E::WidgetID,
    pub childs: T,
    pub orientation: Orientation,
    pub border: Option<Border>,
    p: PhantomData<&'w mut ()>,
}

impl<'w,T,E> Pane<'w,T,E> where E: Env, T: 'w {
    pub fn new(id: E::WidgetID, orientation: Orientation, childs: T) -> Pane<'w,T,E> {
        Pane{
            id,
            childs,
            orientation,
            border: None,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,T,E> Statize<E> for Pane<'w,T,E> where T: StatizeSized<E>, E: Env {
    type Statur = Pane<'static,T::StaturSized,E>;
}
