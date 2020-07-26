use super::*;
use std::{marker::PhantomData};

use calc::calc_bounds;

pub mod widget;

pub struct Pane<'w,T,E,Stil> where E: Env, T: 'w {
    id: E::WidgetID,
    pub childs: T,
    pub orientation: Orientation,
    pub border: Option<Border>,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,T,E> Pane<'w,T,E,()> where E: Env, T: 'w {
    pub fn new(id: E::WidgetID, orientation: Orientation, childs: T) -> Pane<'w,T,E,()> {
        Pane{
            id,
            childs,
            orientation,
            border: None,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,T,E,Stil> Statize<E> for Pane<'w,T,E,Stil> where
    E: Env,
    T: StatizeSized<E>+'w,
    Stil: StatizeSized<E>+'w,
{
    type Statur = Pane<'static,T::StaturSized,E,Stil::StaturSized>;
}
