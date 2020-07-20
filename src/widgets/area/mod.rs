use super::*;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod widget;
pub mod imp;

pub struct Area<'w,E,W,Scroll> where
    E: Env,
    W: 'w,
    Scroll: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdTag>,
    pub inner: W,
    pub scroll: Scroll,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E,W> Area<'w,E,W,(u32,u32)> where
    E: Env,
    W: 'w,
{
    pub fn new(id: E::WidgetID, inner: W) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
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
    pub fn with_state<PScroll>(self, scroll: PScroll) -> Area<'w,E,W,PScroll> where PScroll: Statize<E>+'w {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            inner: self.inner,
            scroll: scroll,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    pub fn with_style(mut self, s: EStyle<E>) -> Self {
        self.style = s;
        self
    }
}

unsafe impl<'w,E,W,Scroll> Statize<E> for Area<'w,E,W,Scroll> where
    E: Env,
    W: Statize<E>, W::Statur: Sized,
    Scroll: Statize<E>, Scroll::Statur: Sized,
{
    type Statur = Area<'static,E,W::Statur,Scroll::Statur>;
}
