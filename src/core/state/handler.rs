use super::*;

pub trait AsHandlerStateful<E>: Context<E> + Sized where E: Env<Context=Self> {
    type T: HandlerStateful<E>;
    
    fn stateful_mut(&mut self) -> &mut Self::T;
    fn stateful(&self) -> &Self::T;
} 
pub trait HandlerStateful<E>: Handler<E> + 'static where E: Env {
    type K: PressedKey<E>;
    
    fn hovered(&self) -> Option<E::WidgetID>;
    fn selected(&self) -> Option<E::WidgetID>;

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_focused(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }

    /*fn pressed(&self) -> &[Self::K];
    ///ordered: combo only pressed if the last N pressed keys (timestamp) are the one passed in k in exact order
    fn is_pressed(&self, k: &[ComboPart]) -> bool {
        todo!()
    }*/

    fn pressed(&self) -> &[Self::K];
    #[inline]
    fn is_pressed(&self, c: &[EEKey<E>]) -> Option<&Self::K> {
        todo!()
    }
    #[inline]
    fn is_pressed_and_id(&self, c: &[EEKey<E>], id: &E::WidgetID) -> bool {
        todo!()
    }
}

