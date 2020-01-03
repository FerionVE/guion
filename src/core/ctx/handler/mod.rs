use super::*;

pub mod access;
pub use access::*;

pub trait Handler<C>: Sized + 'static where C: Context {
    /// PANICKS if widget doesn't exists
    fn _render<E>(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E>;
    /// PANICKS if widget doesn't exists
    fn _event<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E>;
    /// PANICKS if widget doesn't exists
    fn _event_root<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, C: Widgets<E>;
    /// PANICKS if widget doesn't exists
    fn _size<E>(l: Link<E>) -> Size where E: Env<Context=C>, C: Widgets<E>;
}

impl<C> Handler<C> for () where C: Context {
    #[inline] 
    fn _render<E>(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) where E: Env<Context=C>, for<'e> &'e E: EnvLt<'e>, C: Widgets<E> {
        (l.widget_fns().render)(l,r);
    }
    #[inline] 
    fn _event<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, for<'e> &'e E: EnvLt<'e>, C: Widgets<E> {
        (l.widget_fns().event)(l,e);
    }
    #[inline] 
    fn _event_root<E>(l: Link<E>, e: (EEvent<E>,&Bounds)) where E: Env<Context=C>, for<'e> &'e E: EnvLt<'e>, C: Widgets<E> {
        l.ctx._event(&l.path,e)
    }
    #[inline] 
    fn _size<E>(l: Link<E>) -> Size where E: Env<Context=C>, for<'e> &'e E: EnvLt<'e>, C: Widgets<E> {
        (l.widget_fns().size)(l)
    }
}