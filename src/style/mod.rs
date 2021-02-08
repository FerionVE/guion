//! Style handler, Style specifier and selectors
use self::selectag::standard::StdSelectag;
use self::selector::{StyleSelector, StyleSelectorAppend};
use self::standard::cursor::StdCursor;

use super::*;

pub mod font;
pub mod selector;
pub mod selectag;
pub mod color;
pub mod standard;

use std::ops::Deref;

pub trait Style<E>: Clone + Default where E: Env {
    // TODO pending [implied_bounds](https://github.com/rust-lang/rfcs/pull/2089) feature so that the messy StdSelectag deps can be moved into one trait
    type Selector:
        StyleSelector<E> +
        StyleSelectorAppend<StdSelectag<E>,E> +
        for<'a> StyleSelectorAppend<&'a StdSelectag<E>,E> +
        for<'a> StyleSelectorAppend<&'a [StdSelectag<E>],E> +
        for<'a,'b> StyleSelectorAppend<&'a [&'b StdSelectag<E>],E>;
    type Font;
    type Cursor: From<StdCursor>+Clone;
    type Color: Color+Clone;
    type Glyphs: Glyphs<E>;
    
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::Glyphs;
    //TODO fix partial eq impl
    fn is_cached_valid(&self, s: &Self::Glyphs, _c: &mut E::Context) -> bool;

    fn and(&self, s: &Self) -> Self; //TODO clone efficiency

    fn font(&self, selector: &Self::Selector, c: &mut E::Context) -> Option<&Self::Font>;
    fn color(&self, selector: &Self::Selector, c: &mut E::Context) -> Self::Color;
    fn border(&self, selector: &Self::Selector, c: &mut E::Context) -> Border;
    fn cursor(&self, selector: &Self::Selector, c: &mut E::Context) -> Self::Cursor; //TODO std specific trait
}
