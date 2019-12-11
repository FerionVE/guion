use crate::core::ctx::Context;
use crate::core::style::Style;

pub type EPPText<E: Context> = <E::Style as Style>::PreprocessedText;
pub type EPPChar<E: Context> = <E::Style as Style>::PreprocessedText;
pub type EFont<E: Context> = <E::Style as Style>::Font;