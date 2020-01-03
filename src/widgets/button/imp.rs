use crate::core::util::bounds::Bounds;
use crate::core::*;
use render::widgets::RenderStdWidgets;
use lazout::size::Size;
use ctx::aliases::*;
use event::key::Key;
use event::variants::*;
use event::*;
use state::handler::*;
use super::*;

#[macro_export]
macro_rules! impl_button {
    ($t:ty) => {
        #[doc(hidden)]
        impl<E> $crate::macro_prelude::Widget<E> for $t where 
            $t: $crate::macro_prelude::IButton<E>,
            E: $crate::macro_prelude::Env + 'static,
            ERenderer<E>: $crate::macro_prelude::RenderStdWidgets<E>,
            $crate::macro_prelude::ECHandler<E>: $crate::macro_prelude::AsHandlerStateful<E>, 
            EEvent<E>: $crate::macro_prelude::VariantSupport<$crate::macro_prelude::KbdDown<EEKey<E>>,E>,
        {
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
        fn _fns(&self) -> $crate::macro_prelude::WidgetFns<$c> {
            $crate::macro_prelude::IButton::_fns(self)
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
        fn style(&self) -> &$crate::macro_prelude::EStyle<E> {
            $crate::macro_prelude::IButton::style(self)
        }
    };
}

pub fn _render<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: (&mut ERenderer<E>,&Bounds)) where ERenderer<E>: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E>, EEvent<E>: VariantSupport<KbdDown<EEKey<E>>,E> {
    let senf = l.me::<W>();
    let down = 
        l.is_hovered() && l.state().is_pressed_and_id(&[<EEKey<E> as Key>::MOUSE_LEFT], &l.id) ||
        l.is_selected() && l.state().is_pressed_and_id(&[<EEKey<E> as Key>::ENTER], &l.id);
        
    r.0.draw_text_button(r.1,down,senf.caption(),IButton::style(senf));
}

pub fn _event<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: (EEvent<E>,&Bounds)) where ERenderer<E>: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E>, EEvent<E>: VariantSupport<KbdDown<EEKey<E>>,E> {
    let senf = l.me::<W>();
    
    if let Some(e) = e.0.is::<KbdDown<_>>() {
        if e.key == senf.kbd_trigger() {
            (senf.action())(l)
        }
    }
}

pub fn _size<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size where ERenderer<E>: RenderStdWidgets<E>, ECHandler<E>: AsHandlerStateful<E>, EEvent<E>: VariantSupport<KbdDown<EEKey<E>>,E> {
    unimplemented!()
}