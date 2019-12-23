use crate::core::util::border::Border;
use crate::core::*;
use ctx::*;
use style::Style;

pub type ESPPText<E: Env> = <E::Style as Style<E>>::PreprocessedText;
pub type ESPPChar<E: Env> = <E::Style as Style<E>>::PreprocessedText;
pub type ESFont<E: Env> = <E::Style as Style<E>>::Font;
pub type ESColor<E: Env> = <E::Style as Style<E>>::Color;
pub type ESCursor<E: Env> = <E::Style as Style<E>>::Cursor;
pub type ECHandler<E: Env> = <E::Context as Context>::Handler;
pub type ECStateful<E: Env> = <ECHandler<E> as AsHandlerStateful<E>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E>>::K;

#[inline]
pub fn e_default_style<E: Env>() -> &'static E::Style {
    <E::Style as Style<E>>::default()
}
#[inline]
pub fn e_default_border<E: Env>() -> &'static Border {
    <E::Style as Style<E>>::default_border()
}