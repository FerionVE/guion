use crate::core::util::border::Border;
use crate::core::*;
use ctx::*;
use style::Style;

pub type EStyle<E: Env> = <E::Context as Context>::Style;
pub type ESPPText<E: Env> = <EStyle<E> as Style<E::Context>>::PreprocessedText;
pub type ESPPChar<E: Env> = <EStyle<E> as Style<E::Context>>::PreprocessedText;
pub type ESFont<E: Env> = <EStyle<E> as Style<E::Context>>::Font;
pub type ESCursor<E: Env> = <EStyle<E> as Style<E::Context>>::Cursor;
pub type ECHLink<E: Env> = <E::Context as Context>::Link;
pub type ECStateful<E: Env> = <<E::Context as Context>::Link as AsHandlerStateful<E,E::Context>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E,E::Context>>::K;

#[inline]
pub fn e_default_style<E: Env>() -> &'static EStyle<E> {
    <EStyle<E> as Style<E::Context>>::default()
}
#[inline]
pub fn e_default_border<E: Env>() -> &'static Border {
    <EStyle<E> as Style<E::Context>>::default_border()
}