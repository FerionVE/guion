pub mod font;
use crate::core::util::border::Border;
pub use font::*;

pub mod cursor;
pub use cursor::*;

pub mod variant;
pub use variant::*;

pub trait Style: Clone + PartialEq {
    type Font: Font<Self>;
    type Cursor: Cursor;
    type PreprocessedText: PreprocessedText<Self>;
    type PreprocessedChar: PreprocessedChar;

    #[inline]
    fn with(&self, verbs: impl IntoIterator<Item=StyleVerb>) -> Self {
        let mut s = self.clone();
        for v in verbs {
            s._with(v);
        }
        s
    }

    fn _with(&mut self, v: StyleVerb);

    fn font(&self) -> Option<&Self::Font>;
    fn cursor(&self) -> Self::Cursor;

    fn default() -> &'static Self;
    #[inline]
    fn default_border() -> &'static Border;
    
    fn preprocess_text(&self, s: &str) -> Self::PreprocessedText;
    #[inline]
    fn is_cached_valid(&self, s: &Self::PreprocessedText) -> bool {
        s.style() == self
    }

    //fn xx_color(&self) -> Color;
    //fn set_xx_color(&mut self) -> Color;
}