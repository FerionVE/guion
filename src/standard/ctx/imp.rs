use crate::core::ctx::id::WidgetID;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::*;
use crate::core::lazout::size::Size;
use super::*;

impl<S,C> Handler<C> for StandardCtx<S,C> where S: Handler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> + 'static {
    #[inline] 
    fn _render<E: Env>(senf: &mut C, i: &E::WidgetID, r: E::Renderer) {
        S::_render::<E>(senf,i,r);
        unimplemented!()
    }
    #[inline] 
    fn _event<E: Env>(senf: &mut C, i: &E::WidgetID, e: E::Event) {
        S::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _size<E: Env>(senf: &mut C, i: &E::WidgetID) -> Size {
        unimplemented!();
        S::_size::<E>(senf,i)
    }
}
