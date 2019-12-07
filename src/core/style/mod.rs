pub mod font;
pub use font::*;

pub trait Style: Clone {
    type F: Font;

    fn hovered(&self) -> Self;
    fn font(&self) -> Option<&Self::F>;

    fn default() -> Self;
}