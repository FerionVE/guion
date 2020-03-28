use super::*;
use std::{ops::Deref, ops::DerefMut, marker::PhantomData};

use calc::calc_bounds;
use wpps::*;

pub mod imp;

pub struct Pane<'w,T,E> where E: Env, T: Statize+Sized+'w {
    id: E::WidgetID,
    pub childs: T,
    pub orientation: Orientation,
    pub border: Option<Border>,
    p: PhantomData<&'w mut ()>,
}

impl<'w,T,E> Pane<'w,T,E> where E: Env, T: Statize+Sized+'w {
    pub fn new(id: E::WidgetID, childs: T, orientation: Orientation) -> Pane<'w,T,E> {
        Pane{
            id,
            childs,
            orientation,
            border: None,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,T,E> Statize for Pane<'w,T,E> where T: Statize, T::Statur: Statize+Sized, E: Env {
    type Statur = Pane<'static,T::Statur,E>;
}

/*pub fn brokion<'a,W,E>(id: E::WidgetID, e: W) -> Pane<'a,W,E> where W: WidgetImmediate<'a,E>, E: Env {
    Pane::immediate(
        id,
        vec![e],
        Orientation::Horizontal,
    )
}
pub fn bockion<'a,W,E>(id: E::WidgetID, e: W) -> bool where W: WidgetImmediate<'a,E>, E: Env {
    let pane = brokion::<'a,W,E>(id, e);
    pane.invalid()
}*/