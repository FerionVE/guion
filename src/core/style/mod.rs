pub mod font;
use crate::core::util::border::Border;
pub use font::*;

pub mod cursor;
pub use cursor::*;

pub trait Style: Clone {
    type F: Font;
    type C: Cursor;

    fn hovered(&self) -> Self;
    fn font(&self) -> Option<&Self::F>;
    fn hover_cursor(&self) -> Self::C;

    fn default() -> &'static Self;
    #[inline]
    fn default_border() -> &'static Border;

    //fn xx_color(&self) -> Color;
    //fn set_xx_color(&mut self) -> Color;
}