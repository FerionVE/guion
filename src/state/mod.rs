//! query some StdState from any Handler/Context tracking some StdState
use super::*;

pub mod handler;
pub mod standard;
pub mod dyn_state;

//TODO move to StdState trait and AsStdState for deref

//move StdState trait to sstandard StdState trait. StdState is not a core feature!

pub trait CtxStdState<E>: Context<E> + Sized where E: Env<Context=Self> {
    type T: StdState<E>;
    
    fn state_mut(&mut self) -> &mut Self::T;
    fn state(&self) -> &Self::T;
} 
pub trait StdState<E>: 'static where E: Env {
    type K: PressedKey<E>;
    
    fn hovered(&self) -> Option<E::WidgetID>;
    fn selected(&self) -> Option<E::WidgetID>;

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, #[inline] |w| w == *i )
    }
    #[inline]
    fn is_focused(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, #[inline] |w| w == *i )
    }

    /*fn pressed(&self) -> &[Self::K];
    ///ordered: combo only pressed if the last N pressed keys (timestamp) are the one passed in k in exact order
    fn is_pressed(&self, k: &[ComboPart]) -> bool {
        todo!()
    }*/

    fn pressed(&self) -> &[Self::K];
    #[inline]
    fn is_pressed(&self, c: &[EEKey<E>]) -> Option<&Self::K> {
        //todo!() implement all c handling
        self.pressed().iter()
            .find(#[inline] |p| p.key() == c[0] )
    }
    #[inline]
    fn is_pressed_and_id(&self, c: &[EEKey<E>], id: E::WidgetID) -> Option<&Self::K> {
        //todo!() implement all c handling
        self.pressed().iter()
            .find(#[inline] |p| p.key() == c[0] && p.widget().is(id.clone()) )
    }

    fn cursor_pos(&self) -> Option<Offset>;
}
