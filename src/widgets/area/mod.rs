use super::*;
use super::util::state::AtomStateMut;
use std::marker::PhantomData;

pub mod widget;
pub mod imp;

pub struct Area<'w,E,W,Scroll,TrMut> where
    E: Env,
    Self: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub inner: W,
    pub scroll: Scroll,
    scroll_updater: TrMut,
    p: PhantomData<&'w (W,Scroll,TrMut)>,
}

impl<'w,E,W> Area<'w,E,W,(i32,i32),()> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID, inner: W) -> Self {
        Self{
            id,
            size: Gonstraints::empty(),
            style: Default::default(),
            inner,
            scroll: (0,0),
            scroll_updater: (),
            p: PhantomData,
        }
    }
}

impl<'w,E,W,Scroll,TrMut> Area<'w,E,W,Scroll,TrMut> where
    E: Env,
{
    //TODO use a unified state object
    #[inline]
    pub fn with_state<PScroll>(self, scroll: PScroll) -> Area<'w,E,W,PScroll,TrMut> where PScroll: 'w {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            inner: self.inner,
            scroll: scroll,
            scroll_updater: self.scroll_updater,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_scroll_updater<T>(self, mutor: T) -> Area<'w,E,W,Scroll,T> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,ScrollUpdate) + Clone + Send + Sync + 'static {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            inner: self.inner,
            scroll: self.scroll,
            scroll_updater: mutor,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_scroll_atomstate<T>(self, mutor: T) -> Area<'w,E,W,Scroll,impl TriggerMut<E>> where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>) -> &'r mut (dyn AtomStateMut<E,(i32,i32)>) + Clone + Send + Sync + 'static {
        self.with_scroll_updater(move |r,x,c,ScrollUpdate { offset: (ax,ay) }| {
            let state = mutor(r,x,c);
            let (ox,oy) = state.get(c);
            state.set((ox+ax,oy+ay),c);
        })
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

/// blanket-implemented on all `FnMut(&mut E::Context<'_>)`
pub trait TriggerMut<E> where E: Env {
    fn boxed(&self, value: ScrollUpdate) -> Option<BoxMutEvent<E>>;
}

impl<E> TriggerMut<E> for () where E: Env {
    #[inline]
    fn boxed(&self, _: ScrollUpdate) -> Option<BoxMutEvent<E>> {
        None
    }
}

impl<T,E> TriggerMut<E> for T where T: for<'r> FnOnce(E::RootMut<'r>,&'r (),&mut E::Context<'_>,ScrollUpdate) + Clone + Send + Sync + 'static, E: Env {
    #[inline]
    fn boxed(&self, value: ScrollUpdate) -> Option<BoxMutEvent<E>> {
        let s = self.clone();
        Some(Box::new(move |r,x,c| s(r,x,c,value) ))
    }
}

pub struct ScrollUpdate {
    pub offset: (i32,i32),
}
