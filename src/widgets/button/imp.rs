use std::iter::empty;
use std::any::Any;
use crate::core::lazout::size::Size;
use crate::core::lazout::calc::calc_bounds;
use crate::core::widget::Widget;
use crate::core::widget::handler::fns::HandlerFns;
use super::*;

#[macro_export]
macro_rules! impl_button {
    ($t:ty,$env:ty) => {
        #[inline]
        fn id(&self) -> <$env>::WidgetID {
            $crate::macro_prelude::IButton::id(self)
        }
        #[inline]
        fn _handler(&self) -> $crate::macro_prelude::HandlerFns<$env> {
            $crate::macro_prelude::HandlerFns{
                render: $crate::widgets::button::_render::<$t,$env>,
                event: $crate::widgets::button::_event::<$t,$env>,
                size: $crate::widgets::button::_size::<$t,$env>,
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
        fn parent(&self) -> Option<&<$env>::WidgetID> {
            $crate::macro_prelude::IButton::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$env>::WidgetID>) {
            $crate::macro_prelude::IButton::set_parent(self,v)
        }
        #[inline]
        fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=<$env>::WidgetID> + 'a> {
            Box::new(
                std::iter::empty()
            )
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    unimplemented!()
}

pub fn _event<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    unimplemented!()
}

pub fn _size<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    unimplemented!()
}