use crate::core::event::variants::*;
use super::*;

impl<E> Clone for Box<dyn Variant<E>> where E: Env {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
#[doc(hidden)]
pub trait VariantDerive<E>: 'static where E: Env {
    fn _as_any(&self) -> &dyn Any;
    fn _as_any_mut(&mut self) -> &mut dyn Any;
    //don't trigger ambiguousity for now
    fn clon(&self) -> Self where Self: Sized;
    fn clone_dyn(&self) -> Box<dyn Variant<E>>;
}

impl<T,E> VariantDerive<E> for T where T: Variant<E> + Clone, E: Env {
    #[inline]
    fn _as_any(&self) -> &dyn Any {self}
    #[inline]
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}

    #[inline]
    fn clon(&self) -> Self where Self: Sized {
        Clone::clone(self)
    }
    #[inline]
    fn clone_dyn(&self) -> Box<dyn Variant<E>> {
        Box::new( Clone::clone(self) )
    }
}

pub trait StdVarSup<E>:
    VariantSupport<KbdDown<E>,E> +
    VariantSupport<KbdUp<E>,E> +
    VariantSupport<MouseDown<E>,E> +
    VariantSupport<MouseUp<E>,E> +
    VariantSupport<MouseMove,E> +
    VariantSupport<MouseEnter,E> +
    VariantSupport<MouseLeave,E> +
    VariantSupport<GainedFocus,E> +
    VariantSupport<LostFocus,E> +
    VariantSupport<RootEvent<E>,E>
where E: Env, E::Backend: Backend<E,Event=Self> {
    fn is_kbd_down(&self) -> Option<KbdDown<E>> {
        self.is::<KbdDown<E>>()
    }
    fn is_kbd_up(&self) -> Option<KbdUp<E>> {
        self.is::<KbdUp<E>>()
    }
    fn is_mouse_down(&self) -> Option<MouseDown<E>> {
        self.is::<MouseDown<E>>()
    }
    fn is_mouse_up(&self) -> Option<MouseUp<E>> {
        self.is::<MouseUp<E>>()
    }
    fn is_mouse_move(&self) -> Option<MouseMove> {
        self.is::<MouseMove>()
    }
    fn is_mouse_enter(&self) -> Option<MouseEnter> {
        self.is::<MouseEnter>()
    }
    fn is_mouse_leave(&self) -> Option<MouseLeave> {
        self.is::<MouseLeave>()
    }
    fn is_gained_focus(&self) -> Option<GainedFocus> {
        self.is::<GainedFocus>()
    }
    fn is_lost_focus(&self) -> Option<LostFocus> {
        self.is::<LostFocus>()
    }
    /*fn _is_root_event(&self) -> Option<RootEvent<E>> {
        self.is::<RootEvent<E>>()
    }*/
}

impl<E,T> StdVarSup<E> for T where T: 
    VariantSupport<KbdDown<E>,E> +
    VariantSupport<KbdUp<E>,E> +
    VariantSupport<MouseDown<E>,E> +
    VariantSupport<MouseUp<E>,E> +
    VariantSupport<MouseMove,E> +
    VariantSupport<MouseEnter,E> +
    VariantSupport<MouseLeave,E> +
    VariantSupport<GainedFocus,E> +
    VariantSupport<LostFocus,E> +
    VariantSupport<RootEvent<E>,E>
, E: Env, E::Backend: Backend<E,Event=T> {

}

/*impl<E> Event<E> for () where E: Env, E::Backend: Backend<E,Event=Self> {

}*/