//! [Style handler](Style), [Style specifier and selectors](StyleSelector)

use crate::env::Env;
use crate::util::border::Border;

use self::color::Color;
use self::selectag::standard::StdSelectag;
use self::selector::{StyleSelector, StyleSelectorAppend};
use self::standard::cursor::StdCursor;

pub mod selector;
pub mod selectag;
pub mod color;
pub mod font;
pub mod standard;

pub trait Style<E>: Clone + Default where E: Env {
    // TODO pending [implied_bounds](https://github.com/rust-lang/rfcs/pull/2089) feature so that the messy StdSelectag deps can be moved into one trait
    type Selector:
        StyleSelector<E> +
        StyleSelectorAppend<StdSelectag<E>,E> +
        for<'a> StyleSelectorAppend<&'a StdSelectag<E>,E> +
        for<'a> StyleSelectorAppend<&'a [StdSelectag<E>],E> +
        for<'a,'b> StyleSelectorAppend<&'a [&'b StdSelectag<E>],E>;
    type Font;
    type Cursor: From<StdCursor> + Clone + Default;
    type Color: Color + Clone + PartialEq; //TODO special eq for cachor instead of general PartialEq
    
    fn and(&self, s: &Self) -> Self; //TODO clone efficiency

    fn font(&self, selector: &Self::Selector, c: &mut E::Context<'_>) -> Option<&Self::Font>;
    fn color(&self, selector: &Self::Selector, c: &mut E::Context<'_>) -> Self::Color;
    fn border(&self, selector: &Self::Selector, c: &mut E::Context<'_>) -> Border;
    fn cursor(&self, selector: &Self::Selector, c: &mut E::Context<'_>) -> Self::Cursor; //TODO std specific trait
}
