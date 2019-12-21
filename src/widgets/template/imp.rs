use crate::core::*;
use lazout::size::Size;
use super::*;

/// implement Widget for a ITemplate implementor
/// 
/// INFO: using AsWidget is recommended over this, because you can only implement one widget view for one type
#[macro_export]
macro_rules! impl_template {
    ($t:ty) => {
        #[doc(hidden)]
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::ITemplate<E>, E: $crate::macro_prelude::Env + 'static {
            $crate::impl_template_inner!($t,E);
        }
    };
}
/// impl<E> Widget<E> for T where T: ITemplate<E>, E: Env + 'static {
///     crate::impl_template_inner!(T,E);
/// }
#[macro_export]
macro_rules! impl_template_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::ITemplate::id(self)
        }
        #[inline]
        fn _fns(&self) -> $crate::macro_prelude::WidgetFns<$c> {
            $crate::macro_prelude::WidgetFns{
                render: $crate::widgets::template::_render::<$s,$c>,
                event: $crate::widgets::template::_event::<$s,$c>,
                size: $crate::widgets::template::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::ITemplate::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::ITemplate::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::ITemplate::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::ITemplate::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$c>::WidgetID> + 'a> {
            Box::new(
                std::iter::empty()
            )
        }

        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<<$c>::WidgetID> {
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
        fn style(&self) -> &E::Style {
            $crate::macro_prelude::ITemplate::style(self)
        }
    };
}

pub fn _render<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>, r: E::Renderer) {
    unimplemented!()
}

pub fn _event<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}