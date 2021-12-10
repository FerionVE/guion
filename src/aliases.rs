//! Aliases for deep/nested types inside Env
#![allow(type_alias_bounds)]
use crate::widget::imp::{AWidgetMut, AWidget};

use super::*;

pub type ERenderer<'a,E: Env> = <E::Backend as Backend<E>>::Renderer<'a>;
pub type ETextLayout<E: Env> = <E::Backend as Backend<E>>::TextLayout;
pub type EEvent<E: Env> = <E::Backend as Backend<E>>::Event;
pub type EEDest<E: Env> = <EEvent<E> as Event<E>>::Dest;
pub type EEKey<E: Env> = <EEvent<E> as Event<E>>::Key;
pub type EEFilter<E: Env> = <E::Backend as Backend<E>>::EventFilter;
pub type EStyle<E: Env> = <E::Backend as Backend<E>>::Style;
pub type ESize<E: Env> = <E::Backend as Backend<E>>::Size;

pub type ESSelector<E: Env> = <EStyle<E> as Style<E>>::Selector;
pub type ESFont<E: Env> = <EStyle<E> as Style<E>>::Font;
pub type ESColor<E: Env> = <EStyle<E> as Style<E>>::Color;
pub type ESCursor<E: Env> = <EStyle<E> as Style<E>>::Cursor;
pub type ECHandler<'cc,E: Env> = <E::Context<'cc> as Context<E>>::Handler;
pub type ECQueue<'cc,E: Env> = <E::Context<'cc> as Context<E>>::Queue;

pub type ECStdState<'cc,E: Env> = <E::Context<'cc> as CtxStdState<E>>::T;
pub type EPressedKey<'cc,E: Env> = <ECStdState<'cc,E> as StdState<E>>::K;

pub type CtxRef<'a,'s:'a,'cc:'a,E: Env> = (&'a E::Storage<'s>,&'a mut E::Context<'cc>);
pub type CtxRefR<'a,'s:'a,'cc:'a,E: Env> = (&'a E::Storage<'s>,&'a E::Context<'cc>);
pub type CtxRefM<'a,'s:'a,'cc:'a,E: Env> = (&'a mut E::Storage<'s>,&'a mut E::Context<'cc>);

/// Reference to a [`Widget`](Widget) or [immediate widget](AsWidget)
pub type WidgetRef<'a,E: Env> = AWidget<'a,E>;
/// Reference to a [`Widget`](WidgetMut) or [immediate widget](AsWidgetMut)
pub type WidgetRefMut<'a,E: Env> = AWidgetMut<'a,E>;
