use crate::core::util::bounds::Bounds;
use crate::core::*;
use ctx::*;
use lazout::size::Size;
use super::*;

impl<S,C> Handler<C> for StandardCtx<S,C> where S: Handler<C>, C: Context, C::Link: AsHandler<Self,C> + AsHandler<S,C> + 'static {
    #[inline] 
    fn _render<E: Env>(senf: &mut C, i: &E::WidgetID, r: (&mut E::Renderer,&Bounds)) {
        S::_render::<E>(senf,i,r);
        unimplemented!()
    }
    #[inline] 
    fn _event<E: Env>(senf: &mut C, i: &E::WidgetID, e: E::Event) {
        S::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root<E: Env>(senf: &mut C, i: &E::WidgetID, e: E::Event) {
        Self::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _size<E: Env>(senf: &mut C, i: &E::WidgetID) -> Size {
        unimplemented!();
        S::_size::<E>(senf,i)
    }
}
