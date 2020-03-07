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
        impl<E> $crate::macro_prelude::Widget<E> for $t where
            $t: $crate::macro_prelude::INull<E>,
            E: $crate::macro_prelude::Env + 'static,
            $crate::macro_prelude::ERenderer<E>: $crate::macro_prelude::RenderStdWidgets<E>,
            E::Context: $crate::macro_prelude::AsHandlerStateful<E>,
            ESVariant<E>: StyleVariantSupport<StdVerb>
        {
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
            $crate::macro_prelude::INull::style(self,s)
        }
        #[inline]
        fn render(&self, l: $crate::macro_prelude::Link<$c>, r: &mut $crate::macro_prelude::RenderLink<$c>) -> bool {
            $crate::widgets::null::_render::<Self,$c>(l,r)
        }
        #[inline]
        fn event(&self, l: $crate::macro_prelude::Link<$c>, e: ($crate::macro_prelude::EEvent<$c>,&$crate::macro_prelude::Bounds,u64)) {
            $crate::widgets::null::_event::<Self,$c>(l,e)
        }
        #[inline]
        fn size(&self, l: $crate::macro_prelude::Link<$c>) -> $crate::macro_prelude::ESize<$c> {
            $crate::widgets::null::_size::<Self,$c>(l)
        }
        #[inline]
        fn _trace_bounds(&self, l: $crate::macro_prelude::Link<E>, i: usize, b: &$crate::macro_prelude::Bounds, force: bool) -> Result<Bounds,()> {
            Err(())
        }
    };
}

pub fn _render<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>, r: &mut RenderLink<E>) -> bool where ERenderer<E>: RenderStdWidgets<E>, E::Context: AsHandlerStateful<E>, ESVariant<E>: StyleVariantSupport<StdVerb> {
    let mut r = r.with(&[StdVerb::Hovered(l.is_hovered())]);
    r.fill_rect();
    true
}

pub fn _event<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
    
}

pub fn _size<W: INull<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> ESize<E> {
    From::from(Size::empty())
}