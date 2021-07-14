use std::ops::Range;

use crate::env::Env;
use crate::util::bounds::*;

use super::stor::{TextStor, ToTextLayout};

pub trait TxtLayout<E>: TxtLayoutFromStor<str,E>+TxtLayoutFromStor<String,E>+for<'a> TxtLayoutFromStor<&'a str,E> where E: Env {
    fn remove_chars(&mut self, range: Range<usize>);
    /// off in char units
    fn push_chars(&mut self, off: usize, chars: &str);
    
    // fns from https://docs.rs/piet/0.3.0/piet/trait.TextLayout.html

    fn size(&self) -> Dims;

    fn char_at_display(&self, p: Offset) -> usize;

    fn display_of_char(&self, c: usize) -> Bounds;

    fn selection_bounds(&self, s: Range<usize>) -> Vec<Bounds>;

    fn coord_of(&self, i: u32) -> Option<(u32,u32)>;
    fn at_coord(&self, xy: (u32,u32)) -> Option<u32>;
    fn cursor_pos_reverse_line_centric(&self, line: u32, x: i32) -> Option<u32>;
    fn line_count(&self) -> u32;
    fn chars(&self) -> usize;

    fn len(&self) -> usize;

    fn move_cursor(&self, dir: Direction, off: usize) -> usize; //TODO re-implement cursor_stick

    /// How many bytes are n chars to the left from off?
    fn char_len_l(&self, off: usize, chars: usize) -> usize;

    fn fix_boundary(&self, off: usize) -> usize;
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub trait TxtLayoutFromStor<S,E> where E: Env, S: TextStor<E>+?Sized {
    fn from(s: &S, c: &mut E::Context) -> Self;
    fn update(&mut self, s: &S, c: &mut E::Context);
}

impl<T,S,E> TxtLayoutFromStor<S,E> for T where T: TxtLayout<E>, S: ToTextLayout<T,E>, E: Env {
    #[inline]
    fn from(s: &S, c: &mut E::Context) -> Self {
        s.to_text_layout(c)
    }
    #[inline]
    fn update(&mut self, s: &S, c: &mut E::Context) {
        s.update_text_layout(self, c)
    }
}
