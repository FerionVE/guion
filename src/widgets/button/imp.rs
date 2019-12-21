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
            E::Renderer: $crate::macro_prelude::RenderStdWidgets<E>,
            $crate::macro_prelude::ECHLink<E>: $crate::macro_prelude::AsHandlerStateful<E,E::Context>, 
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
        fn style(&self) -> &E::Style {
            $crate::macro_prelude::IButton::style(self)
        }
    };
}

pub fn _render<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, mut r: E::Renderer) where E::Renderer: RenderStdWidgets<E>, ECHLink<E>: AsHandlerStateful<E,E::Context> {
    let senf = l.me::<W>();
    let down = 
        l.is_hovered() && l.state().is_pressed_and_id(&[EKey::<E>::MOUSE_LEFT], &l.widget_id) ||
        l.is_selected() && l.state().is_pressed_and_id(&[EKey::<E>::ENTER], &l.widget_id);
        
    r.draw_text_button(down,senf.caption(),IButton::style(senf));
}

pub fn _event<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>, e: E::Event) where E::Renderer: RenderStdWidgets<E>, ECHLink<E>: AsHandlerStateful<E,E::Context> {
    let senf = l.me::<W>();
    
    if let Some(e) = e.is::<KbdDown<_>>() {
        if e.key == senf.kbd_trigger() {
            (senf.action())(l)
        }
    }
}

pub fn _size<W: IButton<E> + 'static, E: Env + 'static>(mut l: Link<E>) -> Size where E::Renderer: RenderStdWidgets<E>, ECHLink<E>: AsHandlerStateful<E,E::Context> {
    unimplemented!()
}