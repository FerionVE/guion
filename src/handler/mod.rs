//! Handlers can be chained and dispatch events and other stuff
use super::*;

pub mod standard;

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: Sized + 'static where E: Env {
    fn _render(l: Link<E>, r: &mut RenderLink<E>);
    fn _event_direct(l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool)) -> EventResp;
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool)) -> EventResp;
    fn _size(l: Link<E>) -> ESize<E>;
    fn _route_event(l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool), child: E::WidgetPath) -> Result<EventResp,()>;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: &mut RenderLink<E>) {
        l._render(r)
    }
    #[inline] 
    fn _event_direct(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool)) -> EventResp {
        l._event_direct(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool)) -> EventResp {
        if !e.0._root_only() {//TODO warn eprint??
            l.ctx.route_event(l.widget,e,<E::WidgetPath as WidgetPath<E>>::empty()).expect("TODO")

        }else{
            false
        }
    }
    #[inline] 
    fn _size(mut l: Link<E>) -> ESize<E> {
        l._size()
    }
    #[inline]
    fn _route_event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64,bool), child: E::WidgetPath) -> Result<EventResp,()> {
        l._route_event(e,child)
    }
}