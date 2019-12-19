use crate::core::ctx::*;
use crate::core::style::Style;

pub type EPPText<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EPPChar<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EFont<E: Env> = <E::Style as Style>::Font;
pub type ECHLink<E: Env> = <E::Context as Context>::Link;
pub type ECStateful<E: Env> = <<E::Context as Context>::Link as AsHandlerStateful<E>>::T;
