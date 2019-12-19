use crate::core::*;
use ctx::*;
use style::Style;
use event::key::PressedKey;

pub type EPPText<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EPPChar<E: Env> = <E::Style as Style>::PreprocessedText;
pub type EFont<E: Env> = <E::Style as Style>::Font;
pub type ECHLink<E: Env> = <E::Context as Context>::Link;
pub type ECStateful<E: Env> = <<E::Context as Context>::Link as AsHandlerStateful<E,E::Context>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E,E::Context>>::K;
pub type EKey<E: Env> = <E::Event as Event<E>>::K;