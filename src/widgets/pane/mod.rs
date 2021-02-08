use super::*;
use std::{marker::PhantomData};

use calc::calc_bounds;

pub mod widget;

pub struct Pane<'w,E,T> where
    E: Env,
    T: 'w,
{
    id: E::WidgetID,
    pub childs: T,
    pub orientation: Orientation,
    pub style: EStyle<E>,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,T> Pane<'w,E,T> where
    E: Env,
    T: 'w,
{
    #[inline]
    pub fn new(id: E::WidgetID, orientation: Orientation, childs: T) -> Self {
        Pane{
            id,
            childs,
            orientation,
            style: Default::default(),
            p: PhantomData,
        }
    }
    
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }
}