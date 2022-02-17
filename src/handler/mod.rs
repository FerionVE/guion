//! Handlers can be chained and dispatch events and other stuff
use std::any::TypeId;
use std::sync::Arc;

use super::*;

pub mod standard;

//TODO SUPER DIFFICULT support non-'static handlers
pub trait HandlerBuilder<E>: 'static where E: Env {
    type Built: Handler<E>;

    //TODO arc slow
    fn build(f: Arc<dyn for<'c,'cc> Fn(&'c mut E::Context<'cc>)->&'c mut Self>) -> Self::Built;
}

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: 'static where E: Env {
    //TODO move into feature traits
    fn _render(
        &self,
        l: Link<E>,
        r: &mut ERenderer<'_,E>,
        tail: &mut dyn FnMut(Link<E>,&mut ERenderer<'_,E>),
    );
    fn _event_direct(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp;
    fn _event_root(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp;
    fn _size(
        &self,
        l: Link<E>,
        e: &EStyle<E>,
        tail: &mut dyn FnMut(Link<E>,&EStyle<E>)->ESize<E>,
    ) -> ESize<E>;
    fn _send_event(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        child: E::WidgetPath,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>,E::WidgetPath)->Result<EventResp,E::Error>
    ) -> Result<EventResp,E::Error>;

    fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's;

    #[inline]
    fn is_tail(&self) -> bool {
        false
    }

    /// The [`impl_traitcast`] macro should be used to implement this function
    #[allow(unused)]
    #[doc(hidden)]
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        None
    }
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(
        &self,
        l: Link<E>,
        r: &mut ERenderer<'_,E>,
        tail: &mut dyn FnMut(Link<E>,&mut ERenderer<'_,E>),
    ) {
        (tail)(l,r)
    }
    #[inline] 
    fn _event_direct(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp {
        (tail)(l,e)
    }
    #[inline] 
    fn _event_root(
        &self,
        mut l: Link<E>,
        e: &EventCompound<E>,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>)->EventResp,
    ) -> EventResp {
        if !e.event._root_only() {//TODO warn eprint??
            //TODO everything wrong here with event root propagation and tail
            l.event_direct(e)
            //l.ctx.event_direct(l.widget,e)
        }else{
            false
        }
    }
    #[inline] 
    fn _size(
        &self,
        l: Link<E>,
        e: &EStyle<E>,
        tail: &mut dyn FnMut(Link<E>,&EStyle<E>)->ESize<E>,
    ) -> ESize<E> {
        (tail)(l,e)
    }
    #[inline]
    fn _send_event(
        &self,
        l: Link<E>,
        e: &EventCompound<E>,
        child: E::WidgetPath,
        tail: &mut dyn FnMut(Link<E>,&EventCompound<E>,E::WidgetPath)->Result<EventResp,E::Error>
    ) -> Result<EventResp,E::Error> {
        (tail)(l,e,child)
    }

    fn inner<'s>(&self) -> &(dyn Handler<E>+'s) where Self: 's {
        todo!()
    }
    #[inline]
    fn is_tail(&self) -> bool {
        true
    }
}

impl<E> HandlerBuilder<E> for () where E: Env {
    type Built = ();

    fn build(_: Arc<dyn for<'c,'cc> Fn(&'c mut <E as Env>::Context<'cc>)->&'c mut Self>) {
        ()
    }
}
