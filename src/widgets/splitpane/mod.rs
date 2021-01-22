use super::*;
use std::{marker::PhantomData};

pub mod widget;

pub struct SplitPane<'w,E,L,R,V> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    id: E::WidgetID,
    pub childs: (L,R),
    pub state: V,
    pub orientation: Orientation,
    pub width: u32, //TODO with from style
    pub style: EStyle<E>,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,L,R,V> SplitPane<'w,E,L,R,V> where
    E: Env,
    L: 'w,
    R: 'w,
    V: 'w,
{
    #[inline]
    pub fn new(id: E::WidgetID, orientation: Orientation, state: V, childs: (L,R)) -> Self {
        Self{
            id,
            childs,
            state,
            orientation,
            width: 8,
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

