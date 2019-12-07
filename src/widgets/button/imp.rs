use crate::core::lazout::size::Size;
use super::*;

#[macro_export]
macro_rules! impl_button {
    ($t:ty) => {
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::IButton<E>, E: $crate::macro_prelude::Context + 'static {
            $crate::impl_button_inner!($t,E);
        }
    };
}

#[macro_export]
macro_rules! impl_button_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::IButton::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$c> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::button::_render::<$s,$c>,
                event: $crate::widgets::button::_event::<$s,$c>,
                size: $crate::widgets::button::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::IButton::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::IButton::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::IButton::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::IButton::set_parent(self,v)
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
            true
        }
        #[inline]
        fn has_childs(&self) -> bool {
            false
        }
        #[inline]
        fn style(&self) -> &E::Style {
            $crate::macro_prelude::IButton::style(self)
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>, r: &mut E::Renderer) {
    unimplemented!()
}

pub fn _event<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}