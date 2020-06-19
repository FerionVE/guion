//! Aliases for deep/nested types inside Env
#![allow(type_alias_bounds)]
use super::*;

pub type ERenderer<E: Env> = <E::Backend as Backend<E>>::Renderer;
pub type EEvent<E: Env> = <E::Backend as Backend<E>>::Event;
pub type EEDest<E: Env> = <EEvent<E> as Event<E>>::Dest;
pub type EEKey<E: Env> = <EEvent<E> as Event<E>>::Key;
pub type EEFilter<E: Env> = <E::Backend as Backend<E>>::EventFilter;
pub type EStyle<E: Env> = <E::Backend as Backend<E>>::Style;
pub type ESize<E: Env> = <E::Backend as Backend<E>>::Size;

pub type ESPPText<E: Env> = <EStyle<E> as Style<E>>::PreprocessedText;
pub type ESFont<E: Env> = <EStyle<E> as Style<E>>::Font;
pub type ESColor<E: Env> = <EStyle<E> as Style<E>>::Color;
pub type ESCursor<E: Env> = <EStyle<E> as Style<E>>::Cursor;
pub type ESVariant<E: Env> = <EStyle<E> as Style<E>>::Variant;
pub type ECHandler<E: Env> = <E::Context as Context<E>>::Handler;
pub type ECQueue<E: Env> = <E::Context as Context<E>>::Queue;

pub type ECStateful<E: Env> = <E::Context as AsHandlerStateful<E>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E>>::K;

pub type EWPSub<E: Env> = <E::WidgetPath as WidgetPath<E>>::SubPath;

pub type CtxRef<'a,E: Env> = (&'a E::Storage,&'a mut E::Context);
pub type CtxRefR<'a,E: Env> = (&'a E::Storage,&'a E::Context);
pub type CtxRefM<'a,E: Env> = (&'a mut E::Storage,&'a mut E::Context);

/// Reference to a widget or immediate widget
pub type WidgetRef<'a,E: Env> = Box<dyn Widget<'a,E>+'a>;
/// Reference to a widget or immediate widget
pub type WidgetRefMut<'a,E: Env> = Box<dyn WidgetMut<'a,E>+'a>;