use std::ops::Range;

use crate::env::Env;
use crate::util::bounds::*;

use super::cursel::{TxtCurSel, Direction};
use super::stor::{TextStor, ToTextLayout};

pub trait TxtLayout<E>: TxtLayoutFromStor<str,E>+TxtLayoutFromStor<String,E>+for<'a> TxtLayoutFromStor<&'a str,E> /*+Send+Sync*/ where E: Env {
    type CurSel: TxtCurSel<E> +Send+Sync;
    // fns from https://docs.rs/piet/0.3.0/piet/trait.TextLayout.html

    fn display_size(&self) -> Dims;

    fn bytepos_at_display(&self, p: Offset) -> usize;

    fn display_of_bytepos(&self, c: usize) -> Bounds;

    fn cursor_bounds(&self, s: Self::CurSel) -> Bounds;
    fn selection_bounds(&self, s: Self::CurSel) -> Vec<Bounds>;

    // fn coord_of(&self, i: u32) -> Option<(u32,u32)>;
    // fn at_coord(&self, xy: (u32,u32)) -> Option<u32>;
    // fn cursor_pos_reverse_line_centric(&self, line: u32, x: i32) -> Option<u32>;
    fn line_count(&self) -> u32;

    fn len_bytes(&self) -> usize;

    // move cursor into direction. this resets selection (unselect)
    fn move_cursor_direction(&self, old: Self::CurSel, dir: Direction, extend_selection: bool) -> Self::CurSel; //TODO re-implement cursor_stick
    // move cursor to display pos. this resets selection (unselect)
    fn move_cursor_display(&self, old: Self::CurSel, disp_pos: Offset, extend_selection: bool) -> Self::CurSel; //TODO re-implement cursor_stick

    /// How many bytes are n chars to the left from off?
    fn char_len_l(&self, off: usize, chars: usize) -> usize;

    fn fix_boundary(&self, off: usize) -> usize;
    fn fix_cursor_boundaries(&self, s: &mut Self::CurSel);
    fn fixed_cursor_boundaries(&self, mut s: Self::CurSel) -> Self::CurSel {
        self.fix_cursor_boundaries(&mut s);
        s
    }

    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn sync_replace(&mut self, replace_range: Range<usize>, insert: &str);
}

pub trait TxtLayoutFromStor<S,E> where E: Env, S: TextStor<E>+?Sized {
    fn from(s: &S, c: &mut E::Context<'_>) -> Self;
    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    fn update(&mut self, s: &S, c: &mut E::Context<'_>);
}

impl<T,S,E> TxtLayoutFromStor<S,E> for T where T: TxtLayout<E>, S: ToTextLayout<T,E> + ?Sized, E: Env {
    #[inline]
    fn from(s: &S, c: &mut E::Context<'_>) -> Self {
        s.to_text_layout(c)
    }
    /// Apply identical change applied to TextStor.
    /// 
    /// A change should always be applied to TextStor, and depending on the sync update method, synced/updated to TextLayout and TextCurSel
    #[inline]
    fn update(&mut self, s: &S, c: &mut E::Context<'_>) {
        s.update_text_layout(self, c)
    }
}
