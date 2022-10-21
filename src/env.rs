//! The Env type defines a compound over any generic types

use std::fmt::Debug;

use crate::backend::Backend;
use crate::ctx::Context;
use crate::event_new::downcast_map::EventDowncastMap;
use crate::handler::Handler;
use crate::root::{RootRef, RootMut};
use crate::util::error::GuionError;

/// Type compound
///
/// Note the Trait bounds Clone, Default, PartialEq are not used and just for simplifying derives
pub trait Env: Sized + Clone + Copy + Default + PartialEq + Debug + Send + Sync + 'static {
    type Backend: Backend<Self>;
    type Context<'a>: Context<'a, Self> + 'a;
    type RootRef<'a>: RootRef<Self> + 'a;
    type RootMut<'a>: RootMut<Self> + 'a;
    type ValidState: ValidState;
    type Message;
    type Error: std::error::Error + From<GuionError<Self>> + From<()>;
    type Phantom: InfallibleType;
    type EventDowncastMap: EventDowncastMap<Self>;
    //type Commit: Eq + Ord;
}

pub trait EnvFlexCtxHandler: Env {
    type CtxHandler: Handler<Self>;
}

pub trait ValidState {
    fn valid() -> Self;
    fn rerender(&self) -> bool;
    fn relayout(&self) -> bool;
}

#[macro_export]
#[deprecated]
macro_rules! impl_env_stds {
    ($e:ty) => {
        impl $crate::widget::as_widget::AsWidget<$e> for <$e as $crate::env::Env>::WidgetPath {
            #[inline]
            fn as_ref(&self) -> $crate::widget::resolvable::Resolvable<$e> {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
            #[inline]
            fn into_ref<'w>(self) -> $crate::widget::resolvable::Resolvable<'w, $e>
            where
                Self: 'w,
            {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
        }
        impl $crate::widget::as_widget::AsWidgetMut<$e> for <$e as $crate::env::Env>::WidgetPath {
            #[inline]
            fn as_mut(&mut self) -> $crate::widget::resolvable::ResolvableMut<$e> {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
            #[inline]
            fn into_mut<'w>(self) -> $crate::widget::resolvable::ResolvableMut<'w, $e>
            where
                Self: 'w,
            {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
        }
    };
}

/// Implement [`AsWidget`] for a [`Path`](WidgetPath) type for a specific [`Env`]. Required as the blanket impl of [`AsWidget`] for [`Widget`] makes it impossible to generic-implement [`AsWidget`] for [`Paths`](WidgetPath).
///
/// Syntax: `impl_as_widget_for_path!(EnvType; Type[<...>] [where ...])`
///
/// Example: `impl_as_widget_for_path!(MyEnv; StandardPath);`
///
/// Example: `impl_as_widget_for_path!(MyEnv; MyPath<V> where V: Clone);`
#[macro_export]
macro_rules! impl_as_widget_for_path {
    (
        $e:ty;
        $typ:ident
        $( < $($args:ident),* $(,)* > )?
        $(where $($preds:tt)+)?
    ) => {
        impl<$( $($args),* )?> $crate::widget::as_widget::AsWidget<$e> for $typ <$( $($args),* )?>
            $(where $($preds)*)?
        {
            #[inline]
            fn as_ref(&self) -> $crate::widget::resolvable::Resolvable<$e> {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
            #[inline]
            fn into_ref<'w>(self) -> $crate::widget::resolvable::Resolvable<'w,$e> where Self: 'w {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
        }
        impl<$( $($args),* )?> $crate::widget::as_widget::AsWidgetMut<$e> for $typ <$( $($args),* )?>
            $(where $($preds)*)?
        {
            #[inline]
            fn as_mut(&mut self) -> $crate::widget::resolvable::ResolvableMut<$e> {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
            #[inline]
            fn into_mut<'w>(self) -> $crate::widget::resolvable::ResolvableMut<'w,$e> where Self: 'w {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
        }
    };
}

/// A type that can never the instantiatad. Implemented on [`std::convert::Infallible`]
/// 
/// # Safety
/// 
/// This is only ever implemented on types that can't be instantiated
/// 
/// # Example
/// 
/// ```rust
/// enum Foo<E> where E: Env {
///     Value,
///     PhantomData(E::Phantom) //E::Phantom implements Infallible
/// }
/// 
/// fn bar<E>(value: Foo<E>) where E: Env {
///     match value {
///         Value => {},
///         PhantomData(_) => unsafe { unreachable_unchecked!() },
///     }
/// }
/// ```
pub unsafe trait InfallibleType: Sized + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Debug + Send + Sync + 'static {}
pub enum EnvPhantom {}
unsafe impl InfallibleType for std::convert::Infallible {}
