use super::*;

pub mod access;
pub use access::*;

pub trait Handler<C>: Sized + 'static where C: Context, C::Link: for<'a> AsHandler<'a,Self> {
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: C::Link, i: &E::WidgetID, r: E::Renderer);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: C::Link, i: &E::WidgetID, e: E::Event);
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: C::Link, i: &E::WidgetID) -> Size;
}

