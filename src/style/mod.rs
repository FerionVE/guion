//! [Style handler](Style), [Style specifier and selectors](StyleSelector)
use self::selector::StyleSelectors;
use self::selector::std::AppendStdSelector;
use self::selected::SelectedStyle;
use self::selected::std::StdAttributes;
use self::standard::cursor::StdCursor;

use super::*;

pub mod color;
pub mod font;
pub mod standard;

pub mod selected;
pub mod selector;

pub trait Style<E>: Clone + Default where E: Env {
    // TODO pending [implied_bounds](https://github.com/rust-lang/rfcs/pull/2089) feature so that the messy StdSelectag deps can be moved into one trait
    type Selectors: StyleSelectors<E> + AppendStdSelector<E>;
    type Selected: SelectedStyle<E> + StdAttributes<E>;
    type Font;
    type Cursor: From<StdCursor>+Clone+Default;
    type Color: Color+Clone;
    
    fn and(&self, s: &Self) -> Self; //TODO clone efficiency

    fn font(&self, selector: &Self::Selector, c: &mut E::Context) -> Option<&Self::Font>;
    fn color(&self, selector: &Self::Selector, c: &mut E::Context) -> Self::Color;
    fn border(&self, selector: &Self::Selector, c: &mut E::Context) -> Border;
    fn cursor(&self, selector: &Self::Selector, c: &mut E::Context) -> Self::Cursor; //TODO std specific trait
}
