//! Style handler, Style specifier and selectors
use self::standard::cursor::StdCursor;

use super::*;

pub mod font;
pub mod selector;
pub mod color;
pub mod standard;

use selector::standard::StdSelector;
use std::ops::Deref;

pub trait Style<E>: Clone + Default where E: Env {
    type Font;
    type Cursor: From<StdCursor>;
    type Color: Color;
    type Glyphs: Glyphs<E>;
    
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::Glyphs;
    //TODO fix partial eq impl
    fn is_cached_valid(&self, s: &Self::Glyphs, _c: &mut E::Context) -> bool;

    fn and(&self, s: &Self) -> Self; //TODO clone efficiency
}

pub trait StyleQuery<S,E>: Style<E> where E: Env {
    fn font(&self, selector: S, c: &mut E::Context) -> Option<&Self::Font>;
    fn color(&self, selector: S, c: &mut E::Context) -> Self::Color;
    fn border(&self, selector: S, c: &mut E::Context) -> Border;
    fn cursor(&self, selector: S, c: &mut E::Context) -> Self::Cursor; //TODO std specific trait
}
