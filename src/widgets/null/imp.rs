use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use lazout::size::Size;
use super::*;

/// implement Widget for a INull implementor
/// 
/// INFO: using AsWidget is recommended over this, because you can only implement one widget view for one type
#[doc(hidden)] //remove this
#[macro_export]
macro_rules! impl_null {
    ($t:ty) => {
        #[doc(hidden)]
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::INull<E>, E: $crate::macro_prelude::Env + 'static {
            $crate::impl_null_inner!($t,E);
        }
    };
}
/// impl<E> Widget<E> for T where T: INull<E>, E: Env + 'static {
///     crate::impl_null_inner!(T,E);
/// }
#[doc(hidden)] //remove this
#[macro_export]
macro_rules! impl_null_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::INull::id(self)
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::INull::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::INull::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::INull::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::INull::set_parent(self,v)
        }
        #[inline]
        fn _childs(&self) -> Vec<$crate::macro_prelude::WidgetRef<$c>> {
            std::vec![]
        }
        #[inline]
        fn _childs_mut(&mut self) -> Vec<$crate::macro_prelude::WidgetRefMut<$c>> {
            std::vec![]
        }
        #[inline]
        fn child_paths(&self, own_path: $crate::macro_prelude::WPSlice<$c>) -> Vec<<$c>::WidgetPath> {
            std::vec![]
        }
        #[inline]
        fn selectable(&self) -> bool {
            false
        }
        #[inline]
        fn has_childs(&self) -> bool {
            false
        }
        #[inline]
        fn style(&self) -> &$crate::macro_prelude::EStyle<$c> {
            $crate::macro_prelude::INull::style(self)
        }
        #[inline]
        fn render(&self, l: $crate::macro_prelude::Link<$c>, r: (&mut $crate::macro_prelude::ERenderer<$c>,&$crate::macro_prelude::Bounds)) {
            $crate::widgets::null::_render::<Self,$c>(l,r)
        }
        #[inline]
        fn event(&self, l: $crate::macro_prelude::Link<$c>, e: ($crate::macro_prelude::EEvent<$c>,&$crate::macro_prelude::Bounds)) {
            $crate::widgets::null::_event::<Self,$c>(l,e)
        }
        #[inline]
        fn size(&self, l: $crate::macro_prelude::Link<$c>) -> $crate::macro_prelude::ESize<$c> {
            $crate::widgets::null::_size::<Self,$c>(l)
        }
    };
}

pub fn _render<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
    
}

pub fn _event<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds)) {
    
}

pub fn _size<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> ESize<E> {
    From::from(Size::empty())
}