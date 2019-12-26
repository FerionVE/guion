use crate::core::ctx::aliases::*;
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
    
    fn clone(&self) -> Self where Self: Sized;
    fn clone_dyn(&self) -> Box<dyn Variant<E>>;
}

impl<T,E> VariantDerive<E> for T where T: Variant<E> + Clone, E: Env {
    #[inline]
    fn _as_any(&self) -> &dyn Any {self}
    #[inline]
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}

    #[inline]
    fn clone(&self) -> Self where Self: Sized {
        Clone::clone(self)
    }
    #[inline]
    fn clone_dyn(&self) -> Box<dyn Variant<E>> {
        Box::new( Clone::clone(self) )
    }
}

pub trait StdVarSup<E>:
    VariantSupport<KbdDown<EEventKey<E>>,E> +
    VariantSupport<KbdUp<EEventKey<E>>,E> +
    VariantSupport<MouseDown<EEventKey<E>>,E> +
    VariantSupport<MouseUp<EEventKey<E>>,E> +
    VariantSupport<MouseMove,E> +
    VariantSupport<MouseEnter,E> +
    VariantSupport<MouseLeave,E>
where E: Env, E::Backend: Backend<E,Event=Self> {

}

impl<E,T> StdVarSup<E> for T where T: 
    VariantSupport<KbdDown<EEventKey<E>>,E> +
    VariantSupport<KbdUp<EEventKey<E>>,E> +
    VariantSupport<MouseDown<EEventKey<E>>,E> +
    VariantSupport<MouseUp<EEventKey<E>>,E> +
    VariantSupport<MouseMove,E> +
    VariantSupport<MouseEnter,E> +
    VariantSupport<MouseLeave,E>
, E: Env, E::Backend: Backend<E,Event=T> {

}