//! The Env type defines a compound over any generic types
use super::*;
use std::fmt::Debug;

/// Type compound
/// Note the Trait bounds Clone, Default, PartialEq are not used and just for simplifying derives
pub trait Env: Sized + Clone + PartialEq + Debug + Send + Sync + 'static {
    type Backend: Backend<Self>;
    type Context: Context<Self>;
    type Storage: Widgets<Self>;
    type WidgetID: WidgetID;
    type WidgetPath: WidgetPath<Self>;
    type ValidState: ValidState;
    type Message: 'static;
    //type Commit: Eq + Ord;
}

pub trait EnvFlexStyleVariant: Env {
    type StyleVariant: StyleVariant;
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
macro_rules! impl_env_stds {
    ($e:ty) => {
        impl<'w> $crate::widget::as_widget::AsWidget<'w,$e> for <$e as $crate::env::Env>::WidgetPath {
            #[inline]
            fn as_ref<'s>(&'s self) -> $crate::widget::resolvable::Resolvable<'s,$e> where 'w: 's {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
            #[inline]
            fn into_ref(self) -> $crate::widget::resolvable::Resolvable<'w,$e> {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
        }
        impl<'w> $crate::widget::as_widget::AsWidgetMut<'w,$e> for <$e as $crate::env::Env>::WidgetPath {
            #[inline]
            fn as_mut<'s>(&'s mut self) -> $crate::widget::resolvable::ResolvableMut<'s,$e> where 'w: 's {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
            #[inline]
            fn into_mut(self) -> $crate::widget::resolvable::ResolvableMut<'w,$e> {
                $crate::widget::resolvable::ResolvableMut::Path(self.clone().into())
            }
        }
    };
}
