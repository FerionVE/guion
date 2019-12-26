use crate::core::ctx::widgets::Widgets;
use super::*;

pub trait AsHandlerStateful<E>: Handler<E::Context> + Sized where E: Env, E::Context: Context<Handler=Self> + Widgets<E> {
    type T: HandlerStateful<E>;
    
    fn stateful_mut(e: &mut E::Context) -> &mut Self::T;
    fn stateful(e: &E::Context) -> &Self::T;
} 

pub trait HandlerStateful<E>: Handler<E::Context> + 'static where E: Env, E::Context: Widgets<E> {
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
    fn is_pressed(&self, c: &[EEventKey<E>]) -> Option<&Self::K> {
        unimplemented!()
    }
    #[inline]
    fn is_pressed_and_id(&self, c: &[EEventKey<E>], id: &E::WidgetID) -> bool {
        unimplemented!()
    }
}

