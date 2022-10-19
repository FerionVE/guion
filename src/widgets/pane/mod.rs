use std::marker::PhantomData;

use crate::aliases::EStyle;
use crate::env::Env;
use crate::layout::Orientation;

pub mod widget;

pub struct Pane<E,T> where
    E: Env,
{
    pub childs: T,
    pub orientation: Orientation,
    pub style: EStyle<E>,
    p: PhantomData<T>,
}

impl<E,T> Pane<E,T> where
    E: Env,
{
    #[inline]
    pub fn new(orientation: Orientation, childs: T) -> Self {
        Pane{
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