//! Aliases for deep/nested types inside Env
#![allow(type_alias_bounds)]
use crate::text::layout::TxtLayout;

use super::*;

pub type ERenderer<'a, E: Env> = <E::Backend as Backend<E>>::Renderer<'a>;
pub type ETextLayout<E: Env> = <E::Backend as Backend<E>>::TextLayout;
pub type ETCurSel<E: Env> = <ETextLayout<E> as TxtLayout<E>>::CurSel;
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
pub type ECHandler<'cc, E: Env> = <E::Context<'cc> as Context<'cc, E>>::Handler;
pub type ECQueue<'cc, E: Env> = <E::Context<'cc> as Context<'cc, E>>::Queue;

pub type ECStdState<'cc, E: Env> = <E::Context<'cc> as CtxStdState<'cc, E>>::T;
pub type EPressedKey<'cc, E: Env> = <ECStdState<'cc, E> as StdState<E>>::K;

pub type CtxRef<'a, 'rr, 'cc: 'a, E: Env> = (E::RootRef<'rr>, &'a mut E::Context<'cc>);
pub type CtxRefR<'a, 'rr, 'cc: 'a, E: Env> = (E::RootRef<'rr>, &'a E::Context<'cc>);
pub type CtxRefM<'a, 'rr, 'cc: 'a, E: Env> = (E::RootMut<'rr>, &'a mut E::Context<'cc>);

// Reference to a [`Widget`](Widget) or [immediate widget](AsWidget)
//pub type WidgetRef<'a, E: Env> = WCow<'a, dyn WidgetDyn<E> + 'a, Box<(dyn WidgetDyn<E> + 'a)>>;

// TODO this is HORRIBLE temp hack
// pub type RefDynAsWidget<'a, E: Env> =
//     &'a dyn AsWidget<E, Widget = dyn WidgetDyn<E> + 'a, WidgetOwned = Box<(dyn WidgetDyn<E> + 'a)>>;
