use crate::core::ctx::*;
use crate::core::style::Style;
use crate::core::event::key::PressedKey;

pub type EPPText<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EPPChar<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EFont<E: Env> = <E::Style as Style>::Font;
pub type ECHLink<E: Env> = <E::Context as Context>::Link;
pub type ECStateful<E: Env> = <<E::Context as Context>::Link as AsHandlerStateful<E,E::Context>>::T;
pub type ECStateKey<E: Env> = <ECStateful<E> as HandlerStateful<E,E::Context>>::K;
pub type ECStateKCode<E: Env> = <ECStateKey<E> as PressedKey<E::WidgetID>>::K;