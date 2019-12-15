use crate::core::ctx::*;
use crate::core::style::Style;

pub type EPPText<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EPPChar<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EFont<E: Env> = <E::Style as Style>::Font;