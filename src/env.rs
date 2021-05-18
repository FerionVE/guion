//! The Env type defines a compound over any generic types
use super::*;
use std::fmt::Debug;

/// Type compound
/// 
/// Note the Trait bounds Clone, Default, PartialEq are not used and just for simplifying derives
pub trait Env: Sized + Clone + Default + PartialEq + Debug + Send + Sync + 'static {
    type Backend: Backend<Self>;
    type Context: Context<Self>;
    type Storage<'a>: Widgets<Self>+'a;
    type WidgetID: WidgetID;
    /// Implementation of path to resolve [`Widget`]
    type WidgetPath: WidgetPath<Self>;
    type ValidState: ValidState;
    type Message;
    type Error: std::error::Error + From<GuionError<Self>> + From<()>;
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
macro_rules! impl_env_stds {
    ($e:ty) => {
        impl $crate::widget::as_widget::AsWidget<$e> for <$e as $crate::env::Env>::WidgetPath {
            #[inline]
            fn as_ref(&self) -> $crate::widget::resolvable::Resolvable<$e> {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
            #[inline]
            fn into_ref<'w>(self) -> $crate::widget::resolvable::Resolvable<'w,$e> where Self: 'w {
                $crate::widget::resolvable::Resolvable::Path(self.clone().into())
            }
        }
        impl $crate::widget::as_widget::AsWidgetMut<$e> for <$e as $crate::env::Env>::WidgetPath {
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

trait Bar<T> where T: Foo {

}

trait Foo: Sized {
    type Baz<'a>: Bar<Self>+'a;
}
