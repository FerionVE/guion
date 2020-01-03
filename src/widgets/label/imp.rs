use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use lazout::size::Size;
use super::*;

/// implement Widget for a ILabel implementor
/// 
/// INFO: using AsWidget is recommended over this, because you can only implement one widget view for one type
#[macro_export]
macro_rules! impl_label {
    ($t:ty) => {
        #[doc(hidden)]
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::ILabel<E>, E: $crate::macro_prelude::Env + 'static {
            $crate::impl_label_inner!($t,E);
        }
    };
}
/// impl<E> Widget<E> for T where T: ILabel<E>, E: Env + 'static {
///     crate::impl_label_inner!(T,E);
/// }
#[macro_export]
macro_rules! impl_label_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::ILabel::id(self)
        }
        #[inline]
        fn _fns(&self) -> $crate::macro_prelude::WidgetFns<$c> {
            $crate::macro_prelude::WidgetFns{
                render: $crate::widgets::label::_render::<$s,$c>,
                event: $crate::widgets::label::_event::<$s,$c>,
                size: $crate::widgets::label::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::ILabel::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::ILabel::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::ILabel::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::ILabel::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=$crate::macro_prelude::WPSlice<$c>> + 'a> {
            Box::new(
                std::iter::empty()
            )
        }

        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<$crate::macro_prelude::WPSlice<$c>> {
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
        fn style(&self) -> &$crate::macro_prelude::EStyle<E> {
            $crate::macro_prelude::ILabel::style(self)
        }
    };
}

pub fn _render<W: ILabel<E> + 'static, E: Env + 'static>(mut l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
    unimplemented!()
}

pub fn _event<W: ILabel<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds)) {
    unimplemented!()
}

pub fn _size<W: ILabel<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}