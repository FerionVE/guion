use super::*;
use std::marker::PhantomData;

pub mod widget;
pub mod imp;

pub struct Area<'w,E,W,Scroll> where
    E: Env,
    W: 'w,
    Scroll: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub inner: W,
    pub scroll: Scroll,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,W> Area<'w,E,W,(u32,u32)> where
    E: Env,
    W: 'w,
{
    #[inline]
    pub fn new(id: E::WidgetID, inner: W) -> Self {
        Self{
            id,
            size: Gonstraints::empty(),
            style: Default::default(),
            inner,
            scroll: (0,0),
            p: PhantomData,
        }
    }
}

impl<'w,E,W,Scroll> Area<'w,E,W,Scroll> where
    E: Env,
    W: 'w,
    Scroll: 'w,
{
    //TODO use a unified state object
    #[inline]
    pub fn with_state<PScroll>(self, scroll: PScroll) -> Area<'w,E,W,PScroll> where PScroll: 'w {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            inner: self.inner,
            scroll: scroll,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style(mut self, style: EStyle<E>) -> Self {
        self.style = style;
        self
    }
}
