use crate::error::ResolveResult;
use crate::view::mut_target::DynAtomStateMutTarget;
use crate::view::mutor_trait::{MutorEnd, MutorTo};

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
    pub negative_scroll: bool,
    scroll_updater: TrMut,
    p: PhantomData<&'w (W,Scroll,TrMut)>,
}

impl<'w,E,W> Area<'w,E,W,ScrollOff,()> where
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
            negative_scroll: false,
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
            negative_scroll: self.negative_scroll,
            scroll_updater: self.scroll_updater,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_scroll_updater<T>(self, mutor: T) -> Area<'w,E,W,Scroll,T> where T: MutorEnd<ScrollUpdate,E> {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            inner: self.inner,
            scroll: self.scroll,
            negative_scroll: self.negative_scroll,
            scroll_updater: mutor,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_scroll_atomstate<T>(self, mutor: T) -> Area<'w,E,W,Scroll,impl MutorEnd<ScrollUpdate,E>>
    where
        T: MutorTo<(),DynAtomStateMutTarget<ScrollOff>,E>,
    {
        self.with_scroll_updater(
            mutor.mutor_end_if((), |state,_,ScrollUpdate { offset: (ax,ay) },ctx| {
                //TODO ResolveResult handling
                let (ox,oy) = state.get(ctx);
                state.set((ox+ax,oy+ay),ctx);
            })
        )
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
    #[inline]
    pub fn with_negative_scroll(mut self, negative_scroll: bool) -> Self {
        self.negative_scroll = negative_scroll;
        self
    }
}

#[derive(Clone)]
pub struct ScrollUpdate {
    /// scroll offset offset. offset from previous scroll offset to new scroll offset
    pub offset: (i32,i32),
}
