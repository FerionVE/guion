use super::*;
use state::standard::key::StdPressedKey;

impl<S,E> HandlerStateful<E> for StdHandler<S,E> where
    S: Handler<E>,
    E: Env,
    E::Context: AsRefMut<Self> + AsHandlerStateful<E> + 'static,
    EEvent<E>: StdVarSup<E>
{
    type K = StdPressedKey<E>;
    fn hovered(&self) -> Option<E::WidgetID> { //TODO eventually WidgetIdent return in trait
        self.s.mouse.hovered.as_ref().map(|p| p.id.clone() )
    }
    fn selected(&self) -> Option<E::WidgetID> {
        self.s.kbd.focused.as_ref().map(|p| p.id.clone() )
    }
    fn pressed(&self) -> &[Self::K] {
        &self.s.key.pressed[..]
    }
    fn cursor_pos(&self) -> Option<Offset> {
        self.s.mouse.pos
    }
    fn remote_states(&mut self) -> &mut std::collections::HashMap<(E::WidgetID,std::any::TypeId),Box<dyn Any>> {
        &mut self.s.remote_states
    }
}