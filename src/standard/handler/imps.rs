use super::*;
use crate::standard::state::kbd::KbdPressedKey;

impl<S,E> HandlerStateful<E> for StdHandler<S,E> where S: Handler<E>, E: Env, E::Context: AsRefMut<Self> + 'static, EEvent<E>: StdVarSup<E> {
    type K = KbdPressedKey<E>;
    fn hovered(&self) -> Option<E::WidgetID> {
        self.s.mouse.hovered.as_ref().map(|p| p.id().clone() )
    }
    fn selected(&self) -> Option<E::WidgetID> {
        self.s.kbd.focused.as_ref().map(|p| p.id().clone() )
    }
    fn pressed(&self) -> &[Self::K] {
        &self.s.kbd.pressed[..]
    }
}