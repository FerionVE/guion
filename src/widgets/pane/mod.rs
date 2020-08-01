use super::*;
use std::{marker::PhantomData};

use calc::calc_bounds;

pub mod widget;

pub struct Pane<'w,E,T,Stil> where
    E: Env,
    T: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub childs: T,
    pub orientation: Orientation,
    pub style: Stil,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,T> Pane<'w,E,T,()> where
    E: Env,
    T: 'w,
{
    pub fn new(id: E::WidgetID, orientation: Orientation, childs: T) -> Pane<'w,E,T,()> {
        Pane{
            id,
            childs,
            orientation,
            style: (),
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,T,Stil> Statize<E> for Pane<'w,E,T,Stil> where
    E: Env,
    T: StatizeSized<E>+'w,
    Stil: StatizeSized<E>+'w,
{
    type Statur = Pane<'static,E,T::StaturSized,Stil::StaturSized>;
}
