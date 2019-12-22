use crate::core::ctx::aliases::EStyle;
use crate::core::ctx::Context;
use crate::core::ctx::Env;
use crate::core::*;
use util::border::Border;

pub mod font;
pub use font::*;

pub mod variant;
pub use variant::*;

pub trait Style<C>: Clone + PartialEq where C: Context<Style=Self> {
    type Font;
    type Cursor;
    type PreprocessedText: PreprocessedText<Self,C>;
    type PreprocessedChar: PreprocessedChar;

    #[inline]
    fn with(&self, verbs: impl IntoIterator<Item=StyleVerb>) -> Self {
        let mut s = self.clone();
        for v in verbs {
            s._with(v);
        }
        s
    }
    #[doc(hidden)]
    fn _with(&mut self, v: StyleVerb);

    fn font(&self) -> Option<&Self::Font>;
    fn cursor(&self) -> Self::Cursor;

    fn default() -> &'static Self;
    #[inline]
    fn default_border() -> &'static Border;
    
    fn preprocess_text(&self, s: &str, c: &mut C) -> Self::PreprocessedText;
    #[inline]
    fn is_cached_valid(&self, s: &Self::PreprocessedText, c: &mut C) -> bool {
        s.style() == self
    }

    //fn xx_color(&self) -> Color;
    //fn set_xx_color(&mut self) -> Color;
}
