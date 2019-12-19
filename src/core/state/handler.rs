use crate::core::*;
use ctx::handler::Handler;
use ctx::handler::access::AsHandler;
use ctx::widgets::Widgets;
use ctx::*;
use ctx::aliases::*;
use event::key::PressedKey;
use event::key::Key;
use super::*;

pub trait AsHandlerStateful<E,C>: Sized where E: Env<Context=C>, C: Context<Link=Self> + Widgets<E>, C::Link: AsHandler<Self::T,C> + AsHandler<C::Handler,C> {
    type T: HandlerStateful<E,C>;
    #[inline]
    fn stateful_mut(e: &mut C) -> &mut Self::T {
        AsHandler::as_mut(e)
    }
    #[inline]
    fn stateful(e: &C) -> &Self::T {
        AsHandler::as_ref(e)
    }
} 

pub trait HandlerStateful<E,C>: Handler<C> + 'static where E: Env<Context=C>, C: Context + Widgets<E>, C::Link: AsHandler<Self,C> {
    type K: PressedKey<E>;
    
    fn hovered(&self) -> Option<E::WidgetID>;
    fn selected(&self) -> Option<E::WidgetID>;

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }

    /*fn pressed(&self) -> &[Self::K];
    ///ordered: combo only pressed if the last N pressed keys (timestamp) are the one passed in k in exact order
    fn is_pressed(&self, k: &[ComboPart]) -> bool {
        unimplemented!()
    }*/

    fn pressed(&self) -> &[Self::K];
    #[inline]
    fn is_pressed(&self, c: &[EKey<E>]) -> Option<&Self::K> {
        unimplemented!()
    }
    #[inline]
    fn is_pressed_and_id(&self, c: &[EKey<E>], id: &E::WidgetID) -> bool {
        unimplemented!()
    }
}

