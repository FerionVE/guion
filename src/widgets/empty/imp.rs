use crate::core::lazout::size::Size;
use super::*;

#[macro_export]
macro_rules! impl_empty {
    ($t:ty) => {
        impl<E> $crate::macro_prelude::Widget<E> for $t where $t: $crate::macro_prelude::IEmpty<E>, E: $crate::macro_prelude::Env + 'static {
            $crate::impl_empty_inner!($t,E);
        }
    };
}

#[macro_export]
macro_rules! impl_empty_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::IEmpty::id(self)
        }
        #[inline]
        fn _fns(&self) -> $crate::macro_prelude::WidgetFns<$c> {
            $crate::macro_prelude::WidgetFns{
                render: $crate::widgets::empty::_render::<$s,$c>,
                event: $crate::widgets::empty::_event::<$s,$c>,
                size: $crate::widgets::empty::_size::<$s,$c>,
            }
        }
        #[inline]
        fn invalid(&self) -> bool {
            $crate::macro_prelude::IEmpty::invalid(self)
        }
        #[inline]
        fn set_invalid(&mut self, v: bool) {
            $crate::macro_prelude::IEmpty::set_invalid(self,v)
        }
        #[inline]
        fn parent(&self) -> Option<<$c>::WidgetID> {
            $crate::macro_prelude::IEmpty::parent(self)
        }
        #[inline]
        fn set_parent(&mut self, v: Option<<$c>::WidgetID>) {
            $crate::macro_prelude::IEmpty::set_parent(self,v)
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
            <E::Style as $crate::core::style::Style>::default()
        }
        
        #[inline] fn as_any(&self) -> &dyn std::any::Any {self}
        #[inline] fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
    };
}

pub fn _render<W: IEmpty<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) {
    
}

pub fn _event<W: IEmpty<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) {
    
}

pub fn _size<W: IEmpty<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size {
    Size::empty()
}