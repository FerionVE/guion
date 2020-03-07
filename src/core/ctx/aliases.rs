//! Aliases resolving to deep/nested types inside Env
use super::*;

pub type ERenderer<E: Env> = <E::Backend as Backend<E>>::Renderer;
pub type EEvent<E: Env> = <E::Backend as Backend<E>>::Event;
pub type EEDest<E: Env> = <EEvent<E> as Event<E>>::Dest;
pub type EEKey<E: Env> = <EEvent<E> as Event<E>>::Key;
pub type EStyle<E: Env> = <E::Backend as Backend<E>>::Style;
pub type ESize<E: Env> = <E::Backend as Backend<E>>::Size;

pub type ESPPText<E: Env> = <EStyle<E> as Style<E>>::PreprocessedText;
pub type ESPPChar<E: Env> = <EStyle<E> as Style<E>>::PreprocessedChar;
pub type ESFont<E: Env> = <EStyle<E> as Style<E>>::Font;
pub type ESColor<E: Env> = <EStyle<E> as Style<E>>::Color;
pub type ESCursor<E: Env> = <EStyle<E> as Style<E>>::Cursor;
pub type ESVariant<E: Env> = <EStyle<E> as Style<E>>::Variant;
pub type ECHandler<E: Env> = <E::Context as Context<E>>::Handler;
pub type ECQueue<E: Env> = <E::Context as Context<E>>::Queue;

pub type ECStateful<E: Env> = <E::Context as AsHandlerStateful<E>>::T;
pub type EPressedKey<E: Env> = <ECStateful<E> as HandlerStateful<E>>::K;

//pub type ESubWidgetID<E: Env> = <E::WidgetID as WidgetID>::SubWidgetID;
//pub type EWPSlice<'a,E: Env> = <&'a E as EnvLt<'a>>::PathSlice;
pub type EWPSub<E: Env> = <E::WidgetPath as WidgetPath<E>>::SubPath;

pub type CtxRef<'a,E: Env> = (&'a E::Storage,&'a mut E::Context);
pub type CtxRefR<'a,E: Env> = (&'a E::Storage,&'a E::Context);
pub type CtxRefM<'a,E: Env> = (&'a mut E::Storage,&'a mut E::Context);
