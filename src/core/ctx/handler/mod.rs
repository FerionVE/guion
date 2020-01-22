use super::*;

pub mod access;
pub use access::*;

pub trait Handler<E>: Sized + 'static where E: Env {
    /// PANICKS if widget doesn't exists
    fn _render(l: Link<E>, r: (&mut ERenderer<E>,&Bounds));
    /// PANICKS if widget doesn't exists
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds));
    /// PANICKS if widget doesn't exists
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds));
    /// PANICKS if widget doesn't exists
    fn _size(l: Link<E>) -> Size;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
        l.resolve_render(r)
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds)) {
        l.resolve_event(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        l.ctx._event(l.stor,l.path,e)
    }
    #[inline] 
    fn _size(mut l: Link<E>) -> Size {
        l.resolve_size()
    }
}