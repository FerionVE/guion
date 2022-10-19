use std::marker::PhantomData;

use crate::aliases::{ESize, EStyle};
use crate::env::Env;
use crate::error::ResolveResult;
use crate::layout::Gonstraints;
use crate::util::ScrollOff;
use crate::view::mut_target::{DynAtomStateMutTarget, MuTarget};
use crate::view::mutor_trait::{MutorToBuilder, MutorEndBuilder, MutorToBuilderExt};

use super::util::state::AtomStateMut;

pub mod widget;
pub mod imp;

pub struct Area<E,W,Scroll,TrMut> where
    E: Env,
{
    pub size: ESize<E>,
    pub style: EStyle<E>,
    pub inner: W,
    pub scroll: Scroll,
    pub negative_scroll: bool,
    scroll_updater: TrMut,
    p: PhantomData<()>,
}

impl<E,W> Area<E,W,ScrollOff,()> where
    E: Env,
{
    #[inline]
    pub fn new(inner: W) -> Self {
        Self{
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

impl<E,W,Scroll,TrMut> Area<E,W,Scroll,TrMut> where
    E: Env,
{
    //TODO use a unified state object
    #[inline]
    pub fn with_state<PScroll>(self, scroll: PScroll) -> Area<E,W,PScroll,TrMut> {
        Area{
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
    pub fn with_scroll_updater<T>(self, mutor: T) -> Area<E,W,Scroll,T> where T: MutorEndBuilder<ScrollUpdate,E> {
        Area{
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
    pub fn with_scroll_atomstate<T>(self, mutor: T) -> Area<E,W,Scroll,impl MutorEndBuilder<ScrollUpdate,E>>
    where
        T: MutorToBuilder<(),DynAtomStateMutTarget<ScrollOff>,E>,
    {
        self.with_scroll_updater_if(
            mutor, (),
            |state,_,ScrollUpdate { offset: (ax,ay) },ctx| {
                //TODO ResolveResult handling
                let (ox,oy) = state.get(ctx);
                state.set((ox+ax,oy+ay),ctx);
            }
        )
    }

    /// Shortcut for Area::with_scroll_updater(left_mutor.mutor_end_if(left_arg,right_fn))
    #[inline]
    pub fn with_scroll_updater_if<LeftMutor,LeftArgs,LeftTarget,RightFn>(self, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: RightFn) -> Area<E,W,Scroll,impl MutorEndBuilder<ScrollUpdate,E>>
    where 
        LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
        LeftTarget: MuTarget<E> + ?Sized,
        LeftArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut LeftTarget::Mutable<'ss>,&'ss (),
            ScrollUpdate,
            &'c mut E::Context<'cc>
        ) + Clone + Send + Sync + 'static
    {
        self.with_scroll_updater(
            left_mutor.mutor_end_if(left_arg, right_fn)
        )
    }


    #[inline]
    pub fn with_scroll_atomstate_if<LeftMutor,LeftArgs,LeftTarget,RightFn>(self, left_mutor: LeftMutor, left_arg: LeftArgs, right_fn: RightFn) -> Area<E,W,Scroll,impl MutorEndBuilder<ScrollUpdate,E>>
    where 
        LeftMutor: MutorToBuilder<LeftArgs,LeftTarget,E> + Sized,
        LeftTarget: MuTarget<E> + ?Sized,
        LeftArgs: Clone + Sized + Send + Sync + 'static,
        RightFn: for<'s,'ss,'c,'cc> Fn(
            &'s mut LeftTarget::Mutable<'ss>,&'ss (),
            &'c mut E::Context<'cc>
        ) -> &'s mut (dyn AtomStateMut<E,ScrollOff> + 's) + Clone + Send + Sync + 'static
    {
        self.with_scroll_updater_if(
            left_mutor, left_arg,
            move |state,_,ScrollUpdate { offset: (ax,ay) },ctx| {
                let state = (right_fn)(state,&(),ctx);
                //TODO ResolveResult handling
                let (ox,oy) = state.get(ctx);
                state.set((ox+ax,oy+ay),ctx);
            }
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
