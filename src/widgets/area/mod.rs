use super::*;
use std::marker::PhantomData;

pub mod widget;
pub mod imp;

pub struct Area<'w,E,W,Scroll,Stil> where
    E: Env,
    W: 'w,
    Scroll: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub inner: W,
    pub scroll: Scroll,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E,W> Area<'w,E,W,(u32,u32),()> where
    E: Env,
    W: 'w,
{
    #[inline]
    pub fn new(id: E::WidgetID, inner: W) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: (),
            inner,
            scroll: (0,0),
            p: PhantomData,
        }
    }
}

impl<'w,E,W,Scroll,Stil> Area<'w,E,W,Scroll,Stil> where
    E: Env,
    W: 'w,
    Scroll: 'w,
    Stil: 'w,
{
    //TODO use a unified state object
    #[inline]
    pub fn with_state<PScroll>(self, scroll: PScroll) -> Area<'w,E,W,PScroll,Stil> where PScroll: Statize<E>+'w {
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
    pub fn with_style<SStil>(self, style: SStil) -> Area<'w,E,W,Scroll,SStil> where SStil: Statize<E>+'w {
        Area{
            id: self.id,
            size: self.size,
            style,
            inner: self.inner,
            scroll: self.scroll,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,W,Scroll,Stil> Statize<E> for Area<'w,E,W,Scroll,Stil> where
    E: Env,
    W: StatizeSized<E>,
    Scroll: StatizeSized<E>,
    Stil: StatizeSized<E>,
{
    type Statur = Area<'static,E,W::StaturSized,Scroll::StaturSized,Stil::StaturSized>;
}
