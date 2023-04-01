//! query some StdState from any Handler/Context tracking some StdState

use crate::ctx::Context;
use crate::env::Env;
use crate::event::key::PressedKey;
use crate::event::key_combo::{KeyCombo, MatchKey, Matches};
use crate::newpath::{PathResolvusDyn, FwdCompareStat, PathStackDyn, PathStack};
use crate::util::bounds::Offset;
use crate::widget::id::WidgetID;

pub mod handler;
pub mod dyn_state;
pub mod standard;

//TODO move to StdState trait and AsStdState for deref

//move StdState trait to standard StdState trait. StdState is not a core feature!

pub trait CtxStdState<'cc,E>: Context<'cc,E> + Sized + 'cc where E: Env {
    type T: StdState<E> + 'cc;
    
    fn state_mut(&mut self) -> &mut Self::T;
    fn state(&self) -> &Self::T;
} 
pub trait StdState<E> where E: Env {
    type K: PressedKey<E> + 'static;
    
    fn hovered(&self) -> Option<(&(dyn PathResolvusDyn<E>+'_),WidgetID)>;
    fn selected(&self) -> Option<(&(dyn PathResolvusDyn<E>+'_),WidgetID)>;

    #[inline]
    fn is_hovered_path(&self, i: &(dyn PathStackDyn<E>+'_)) -> bool {
        self.hovered().map_or(false, #[inline] |w| i.fwd_compare(w.0) == FwdCompareStat::Equal )
    }
    #[inline]
    fn is_focused_path(&self, i: &(dyn PathStackDyn<E>+'_)) -> bool {
        self.selected().map_or(false, #[inline] |w| i.fwd_compare(w.0) == FwdCompareStat::Equal )
    }

    #[inline]
    fn is_hovered(&self, i: WidgetID) -> bool {
        self.hovered().map_or(false, #[inline] |w| i == w.1 )
    }
    #[inline]
    fn is_focused(&self, i: WidgetID) -> bool {
        self.selected().map_or(false, #[inline] |w| i == w.1 )
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
    fn is_pressed_and_path(&self, c: impl KeyCombo<E>, id: &(dyn PathStackDyn<E>+'_)) -> Option<&Self::K> {
        if let Some(v) = self.is_pressed(c) {
            if id.fwd_compare(v.widget().0) == FwdCompareStat::Equal {
                return Some(v);
            }
        }
        None
    }
    #[inline]
    fn is_pressed_and_id(&self, c: impl KeyCombo<E>, id: WidgetID) -> Option<&Self::K> {
        if let Some(v) = self.is_pressed(c) {
            if v.widget().1 == id {
                return Some(v);
            }
        }
        None
    }

    fn cursor_pos(&self) -> Option<Offset>;
}
