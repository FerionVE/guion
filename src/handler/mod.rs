//! Handlers can be chained and dispatch events and other stuff
use super::*;

pub mod standard;

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: Sized + 'static where E: Env {
    fn _render(l: Link<E>, r: &mut RenderLink<E>);
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) -> bool;
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) -> bool;
    fn _size(l: Link<E>) -> ESize<E>;
    fn _route_event(l: Link<E>, e: (EEvent<E>,&Bounds,u64), child: E::WidgetPath) -> Result<bool,()>;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: &mut RenderLink<E>) {
        l._render(r)
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) -> bool {
        l._event(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) -> bool {
        if !e.0._root_only() {//TODO warn eprint??
            l.ctx.event(l.widget,e)
        }else{
            false
        }
    }
    #[inline] 
    fn _size(mut l: Link<E>) -> ESize<E> {
        l._size()
    }
    #[inline]
    fn _route_event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64), child: E::WidgetPath) -> Result<bool,()> {
        l._route_event(e,child)
    }
}