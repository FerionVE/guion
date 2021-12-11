//! query some StdState from any Handler/Context tracking some StdState
use super::*;

use crate::event::key_combo::{KeyCombo, MatchKey, Matches};

pub mod handler;
pub mod dyn_state;
pub mod standard;

//TODO move to StdState trait and AsStdState for deref

//move StdState trait to standard StdState trait. StdState is not a core feature!

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
    fn is_pressed(&self, c: impl KeyCombo<E>) -> Option<&Self::K> {
        let getion = |e: MatchKey<'_,E>| {
            self.pressed().iter().enumerate()
                .find(|(_,v)| e.matches(&v.key()) )
                .map(|(i,_)| (true,Some(Matches(vec![i]))) )
                .unwrap_or((false,None))
        };
        if let (true,Some(k)) = c.match_in(getion) {
            Some(&self.pressed()[k.max()])
        } else {
            None
        }
    }
    #[inline]
    fn is_pressed_and_id(&self, c: impl KeyCombo<E>, id: E::WidgetID) -> Option<&Self::K> {
        if let Some(v) = self.is_pressed(c) {
            if v.widget().is(id.clone()) {
                return Some(v);
            }
        }
        None
    }

    fn cursor_pos(&self) -> Option<Offset>;
}
