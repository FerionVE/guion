pub mod font;
use crate::core::util::border::Border;
pub use font::*;

pub trait Style: Clone {
    type F: Font;

    fn hovered(&self) -> Self;
    fn font(&self) -> Option<&Self::F>;

    fn default() -> &'static Self;
    #[inline]
    fn default_border() -> &'static Border;
}