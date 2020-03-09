use super::*;

/// Handlers are stacked inside a Context and any render/event/size action goes through the handler stack
pub trait Handler<E>: Sized + 'static where E: Env {
    fn _render(l: Link<E>, r: &mut RenderLink<E>) -> bool;
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds,u64));
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64));
    fn _size(l: Link<E>) -> ESize<E>;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: &mut RenderLink<E>) -> bool {
        l._render(r)
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        l._event(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        if !e.0._root_only() {//TODO warn eprint??
            l.ctx.event(l.widget,e)
        }
    }
    #[inline] 
    fn _size(mut l: Link<E>) -> ESize<E> {
        l._size()
    }
}