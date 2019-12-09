pub mod font;
use crate::core::util::border::Border;
pub use font::*;

pub mod cursor;
pub use cursor::*;

pub trait Style: Clone + PartialEq {
    type Font: Font<Self>;
    type Cursor: Cursor;
    type PreprocessedText: PreprocessedText<Self>;
    type PreprocessedChar: PreprocessedChar;

    fn hovered(&self) -> Self;
    fn font(&self) -> Option<&Self::Font>;
    fn hover_cursor(&self) -> Self::Cursor;

    fn default() -> &'static Self;
    #[inline]
    fn default_border() -> &'static Border;
    
    fn preprocess_text(&self, s: &str) -> Self::PreprocessedText;
    #[inline]
    fn is_cached_valid(&self, s: &Self::PreprocessedText) {
        //eq style
    }

    //fn xx_color(&self) -> Color;
    //fn set_xx_color(&mut self) -> Color;
}