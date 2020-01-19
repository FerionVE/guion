use super::*;

pub mod access;
pub use access::*;

pub trait Handler<C>: Sized + 'static where C: Context {
    /// PANICKS if widget doesn't exists
    fn _render<E>(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) where E: Env<Context=C>;
    /// PANICKS if widget doesn't exists
    fn _event<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>;
    /// PANICKS if widget doesn't exists
    fn _event_root<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>;
    /// PANICKS if widget doesn't exists
    fn _size<E>(l: Link<E>) -> Size where E: Env<Context=C>;
}

impl<C> Handler<C> for () where C: Context {
    #[inline] 
    fn _render<E>(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) where E: Env<Context=C> {
        l.widget().render(l,r);
    }
    #[inline] 
    fn _event<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C> {
        l.widget().event(l,e);
    }
    #[inline] 
    fn _event_root<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C> {
        l.ctx._event(l.stor,l.path,e)
    }
    #[inline] 
    fn _size<E>(l: Link<E>) -> Size where E: Env<Context=C> {
        l.widget().size(l)
    }
}