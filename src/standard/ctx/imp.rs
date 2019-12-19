use crate::core::ctx::id::WidgetID;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::*;
use crate::core::lazout::size::Size;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<S,C> Handler<C> for StandardCtx<S,C> where S: Handler<C>, C: Context, for<'a> C::Link: AsHandler<'a,Self> + AsHandler<'a,S> + 'static {
    #[inline] 
    fn _render<E: Env>(senf: C::Link, i: &E::WidgetID, r: E::Renderer) {
        //Self::Child::_render::<E>(senf,i,r)
        unimplemented!()
    }
    #[inline] 
    fn _event<E: Env>(senf: C::Link, i: &E::WidgetID, e: E::Event) {
        //Self::Child::_event::<E>(senf,i,e)
        unimplemented!()
    }
    #[inline] 
    fn _size<E: Env>(senf: C::Link, i: &E::WidgetID) -> Size {
        //Self::Child::_size::<E>(senf,i)
        unimplemented!()
    }
}

/*impl<S> AsHandler<Self> for StandardCtx<S> where S: Handler {
    fn handler_mut(&mut self) -> &mut Self {
        self
    }
}

/*impl<S,E> HandlerStateful<E> for StandardCtx<S> where S: Handler, E: Env {

}*/

struct E<F> {
    i: F,
}

use std::ops::*;

impl<F> Add<u32> for E<F> where F: Sub<u32,Output=u32> {
    type Output=u32;

    fn add(self, r: u32) -> u32 {
        self.i - r
    }
}*/