use std::marker::PhantomData;

use crate::aliases::EStyle;
use crate::env::Env;
use crate::layout::Orientation;

pub mod widget;
pub mod decl;

impl<E,T> decl::Pane<E,T> where
    E: Env,
{
    #[inline]
    pub fn new(orientation: Orientation, childs: T) -> Self {
        decl::Pane {
            childs,
            orientation,
            style: None,
        }
    }
    
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = Some(style);
        self
    }
}