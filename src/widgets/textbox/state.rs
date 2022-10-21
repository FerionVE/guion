use crate::aliases::{ETextLayout, ETCurSel};
use crate::env::Env;
use crate::text::layout::TxtLayout;
use crate::text::stor::{TextStorMut, TextStor};
use crate::util::bounds::{Offset, Bounds};
use crate::widgets::util::state::{AtomStateMut, AtomState};

use super::TextBoxUpdate;

pub fn max_off<E>(g: &ETextLayout<E>, b: &Bounds) -> Offset where E: Env {
    let size = g.display_size();
    Offset {
        x: size.w.saturating_sub( b.w() ) as i32,
        y: size.h.saturating_sub( b.h() ) as i32,
    }
}

// #[non_exhaustive]
// #[derive(Default)]
// pub struct TextBoxObj<E> where E: Env {
//     pub text: String,
//     pub scroll: (u32,u32),
//     pub selection: ETCurSel<E>,
// }


/// Conveniant container to store the other states beside of the text itself, like e.g. the selection/cursor and the scroll position
#[non_exhaustive]
#[derive(Default)]
pub struct TextBoxMeta<E> where E: Env {
    pub scroll: (u32,u32),
    pub selection: ETCurSel<E>,
}

// impl<E> TextBoxObj<E> where E: Env {
//     pub fn apply_tbupdate(&mut self, t: &TextBoxUpdate<E>) {
//         if let Some(tbupd) = &t.0 {
//             TextStorMut::<E>::replace(&mut self.text,tbupd.0.clone(),tbupd.1.as_ref());
//         }
//         if let Some(curs) = t.1.clone() {
//             self.selection = curs;
//         }
//     }
//     pub fn apply_scroll(&mut self, s: (u32,u32)) {
//         self.scroll = s;
//     }
// }

// impl<E> AtomState<E,ETCurSel<E>> for TextBoxMeta<E> where E: Env {
//     #[inline]
//     fn get_direct(&self) -> Result<ETCurSel<E>,()> {
//         Ok(self.selection.clone())
//     }
// }
// impl<E> AtomStateMut<E,ETCurSel<E>> for TextBoxMeta<E> where E: Env {
//     #[inline]
//     fn set_direct(&mut self, v: ETCurSel<E>) -> Result<(),()> {
//         self.selection = v;
//         Ok(())
//     }
// }

// impl<E> AtomState<E,(u32,u32)> for TextBoxMeta<E> where E: Env {
//     #[inline]
//     fn get_direct(&self) -> Result<(u32,u32),()> {
//         Ok(self.scroll)
//     }
// }
// impl<E> AtomStateMut<E,(u32,u32)> for TextBoxMeta<E> where E: Env {
//     #[inline]
//     fn set_direct(&mut self, v: (u32,u32)) -> Result<(),()> {
//         self.scroll = v;
//         Ok(())
//     }
// }
