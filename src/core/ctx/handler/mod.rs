use super::*;

pub mod access;
pub use access::*;

pub trait Handler<C>: Sized + 'static where C: Context {
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: &mut C, i: &E::WidgetID, r: (&mut ERenderer<E>,&Bounds));
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event_root<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: &mut C, i: &E::WidgetID) -> Size;
}

