pub mod font;
pub use font::*;

pub trait Style: Clone {
    type F: Font;

    fn hovered() -> Self;
    fn font() -> Option<Self::F>;
}