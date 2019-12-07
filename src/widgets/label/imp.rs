use crate::core::lazout::size::Size;
use super::*;

/// implement Widget for a ILabel implementor
/// 
/// INFO: using AsWidget is recommended over this, because you can only implement one widget view for one type
#[macro_export]
macro_rules! impl_label {
    ($t:ty) => {
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::ILabel<E>, E: $crate::macro_prelude::Context + 'static {
            $crate::impl_label_inner!($t,E);
        }
    };
}
/// impl<E> Widget<E> for T where T: ILabel<E>, E: Context + 'static {
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
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$c> {
            $crate::macro_prelude::HandlerFns{
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
        fn style(&self) -> E::Style {
            $crate::macro_prelude::ILabel::style(self).clone()
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: ILabel<E> + 'static, E: Context + 'static>(mut l: Link<E>, r: &mut E::Renderer) {
    unimplemented!()
}

pub fn _event<W: ILabel<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: ILabel<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}