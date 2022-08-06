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
    //don't trigger ambiguity for now
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

/// Extension Trait for Events supporting all standard variants.  
/// Exists to simplify where clauses.
pub trait StdVarSup<E>:
    VariantSupport<KbdDown<E>,E> +
    VariantSupport<KbdUp<E>,E> +
    VariantSupport<KbdPress<E>,E> +
    VariantSupport<TextInput<E>,E> +
    VariantSupport<MouseDown<E>,E> +
    VariantSupport<MouseUp<E>,E> +
    VariantSupport<MouseScroll<E>,E> +
    VariantSupport<MouseMove<E>,E> +
    VariantSupport<MouseEnter<E>,E> +
    VariantSupport<MouseLeave<E>,E> +
    VariantSupport<WindowMove,E> +
    VariantSupport<WindowResize,E> +
    VariantSupport<Focus<E>,E> +
    VariantSupport<Unfocus<E>,E> +
    VariantSupport<RootEvent<E>,E>
where E: Env {
    #[inline]
    fn is_kbd_down(&self) -> Option<KbdDown<E>> {
        self.is::<KbdDown<E>>()
    }
    #[inline]
    fn is_kbd_press(&self) -> Option<KbdPress<E>> {
        self.is::<KbdPress<E>>()
    }
    #[inline]
    fn is_kbd_up(&self) -> Option<KbdUp<E>> {
        self.is::<KbdUp<E>>()
    }
    #[inline]
    fn is_kbd_down_or_press(&self) -> Option<KbdDown<E>> {
        if let Some(ee) = self.is_kbd_down() {
            Some(ee)
        }else if let Some(ee) = self.is_kbd_press() {
            Some(KbdDown{key: ee.key})
        }else{
            None
        }
    }
    #[inline]
    fn is_text_input(&self) -> Option<TextInput<E>> {
        self.is::<TextInput>()
    }
    #[inline]
    fn is_mouse_down(&self) -> Option<MouseDown<E>> {
        self.is::<MouseDown<E>>()
    }
    #[inline]
    fn is_mouse_up(&self) -> Option<MouseUp<E>> {
        self.is::<MouseUp<E>>()
    }
    #[inline]
    fn is_mouse_scroll(&self) -> Option<MouseScroll<E>> {
        self.is::<MouseScroll>()
    }
    #[inline]
    fn is_mouse_move(&self) -> Option<MouseMove<E>> {
        self.is::<MouseMove>()
    }
    #[inline]
    fn is_mouse_enter(&self) -> Option<MouseEnter<E>> {
        self.is::<MouseEnter>()
    }
    #[inline]
    fn is_mouse_leave(&self) -> Option<MouseLeave<E>> {
        self.is::<MouseLeave>()
    }
    #[inline]
    fn is_window_move(&self) -> Option<WindowMove> {
        self.is::<WindowMove>()
    }
    #[inline]
    fn is_window_reset(&self) -> Option<WindowResize> {
        self.is::<WindowResize>()
    }
    #[inline]
    fn is_focus(&self) -> Option<Focus<E>> {
        self.is::<Focus>()
    }
    #[inline]
    fn is_unfocus(&self) -> Option<Unfocus<E>> {
        self.is::<Unfocus>()
    }
    /*#[inline]
    fn _is_root_event(&self) -> Option<RootEvent<E>> {
        self.is::<RootEvent<E>>()
    }*/
    #[inline]
    fn is_hover_update(&self) -> bool {
        self.is_mouse_enter().is_some() || self.is_mouse_leave().is_some()
    }
}

impl<E,T> StdVarSup<E> for T where T: 
    VariantSupport<KbdDown<E>,E> +
    VariantSupport<KbdUp<E>,E> +
    VariantSupport<KbdPress<E>,E> +
    VariantSupport<TextInput<E>,E> +
    VariantSupport<MouseDown<E>,E> +
    VariantSupport<MouseUp<E>,E> +
    VariantSupport<MouseScroll<E>,E> +
    VariantSupport<MouseMove<E>,E> +
    VariantSupport<MouseEnter<E>,E> +
    VariantSupport<MouseLeave<E>,E> +
    VariantSupport<WindowMove,E> +
    VariantSupport<WindowResize,E> +
    VariantSupport<Focus<E>,E> +
    VariantSupport<Unfocus<E>,E> +
    VariantSupport<RootEvent<E>,E>
, E: Env, E::Backend: Backend<E,Event=T> {

}
