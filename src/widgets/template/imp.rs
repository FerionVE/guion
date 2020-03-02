use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use lazout::size::Size;
use super::*;

/// implement Widget for a ITemplate implementor
/// 
/// INFO: using AsWidget is recommended over this, because you can only implement one widget view for one type
#[doc(hidden)] //remove this
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
#[doc(hidden)] //remove this
#[macro_export]
macro_rules! impl_template_inner {
    ($s:ty,$c:ty) => {
        #[inline]
        fn id(&self) -> <$c>::WidgetID {
            $crate::macro_prelude::ITemplate::id(self)
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
        fn childs_ref(&self) -> Vec<$crate::macro_prelude::Resolvable<$c>> {
            std::vec![]
        }
        #[inline]
        fn childs_mut(&mut self) -> Vec<$crate::macro_prelude::ResolvableMut<$c>> {
            std::vec![]
        }
        #[inline]
        fn focusable(&self) -> bool {
            false
        }
        #[inline]
        fn childs(&self) -> usize {
            0
        }
        #[inline]
        fn style(&self, s: &mut $crate::macro_prelude::ESVariant<$c>) {
            $crate::macro_prelude::ITemplate::style(self,s)
        }
        #[inline]
        fn render(&self, l: $crate::macro_prelude::Link<$c>, r: &mut $crate::macro_prelude::RenderLink<$c>) -> bool {
            $crate::widgets::template::_render::<Self,$c>(l,r)
        }
        #[inline]
        fn event(&self, l: $crate::macro_prelude::Link<$c>, e: ($crate::macro_prelude::EEvent<$c>,&$crate::macro_prelude::Bounds,u64)) {
            $crate::widgets::template::_event::<Self,$c>(l,e)
        }
        #[inline]
        fn size(&self, l: $crate::macro_prelude::Link<$c>) -> $crate::macro_prelude::ESize<$c> {
            $crate::widgets::template::_size::<Self,$c>(l)
        }
        #[inline]
        fn _trace_bounds(&self, l: $crate::macro_prelude::Link<E>, i: usize, b: &$crate::macro_prelude::Bounds, force: bool) -> Result<Bounds,()> {
            Err(())
        }
    };
}

pub fn _render<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>, r: &mut RenderLink<E>) -> bool {
    todo!();
    true
}

pub fn _event<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
    todo!()
}

pub fn _size<W: ITemplate<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> ESize<E> {
    todo!()
}