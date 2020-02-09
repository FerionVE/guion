use super::*;

pub mod font;
pub use font::*;

pub mod variant;
pub use variant::*;

pub mod color;
pub use color::*;
use std::ops::Deref;

pub trait Style<E>: Clone + PartialEq where E: Env, E::Backend: Backend<E,Style=Self> {
    type Font;
    type Cursor;
    type Color: Color;
    type PreprocessedText: PreprocessedText<E>;
    type PreprocessedChar: PreprocessedChar;

    #[inline]
    fn with(&self, verbs: impl IntoIterator<Item=impl Deref<Target=StyleVerb>>) -> Self {
        let mut s = self.clone();
        s.attach(verbs);
        s
    }
    #[inline]
    fn attach(&mut self, verbs: impl IntoIterator<Item=impl Deref<Target=StyleVerb>>) {
        for v in verbs {
            self._with(*v.deref());
        }
    }
    #[doc(hidden)]
    fn _with(&mut self, v: StyleVerb);

    fn font(&self) -> Option<&Self::Font>;
    fn cursor(&self) -> Self::Cursor;
    
    fn preprocess_text(&self, s: &str, c: &mut E::Context) -> Self::PreprocessedText;
    #[inline]
    fn is_cached_valid(&self, s: &Self::PreprocessedText, _c: &mut E::Context) -> bool {
        s.style() == self
    }

    //fn xx_color(&self) -> Color;
    //fn set_xx_color(&mut self) -> Color;
}
