use crate::core::lazout::size::Size;
use super::*;

#[macro_export]
macro_rules! impl_button {
    ($t:ty,$Context:ty) => {
        #[inline]
        fn id(&self) -> <$Context>::WidgetID {
            $crate::macro_prelude::IButton::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$Context> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::button::_render::<$t,$Context>,
                event: $crate::widgets::button::_event::<$t,$Context>,
                size: $crate::widgets::button::_size::<$t,$Context>,
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
        fn parent(&self) -> Option<&<$Context>::WidgetID> {
            $crate::macro_prelude::IButton::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$Context>::WidgetID>) {
            $crate::macro_prelude::IButton::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$Context>::WidgetID> + 'a> {
            Box::new(
                std::iter::empty()
            )
        }

        #[inline]
        fn childs_vec<'a>(&'a self) -> Vec<<$Context>::WidgetID> {
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
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>, mut r: &mut E::Renderer) {
    unimplemented!()
}

pub fn _event<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: IButton<E> + 'static, E: Context + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}