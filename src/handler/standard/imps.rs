use super::*;
use state::standard::key::StdPressedKey;
use std::any::TypeId;
use std::hash::Hash;

impl<S,E> StdState<E> for StdHandler<S,E> where
    S: Handler<E>,
    E: Env,
    for<'a> E::Context<'a>: AsRefMut<Self> + CtxStdState<E>,
    EEvent<E>: StdVarSup<E>
{
    type K = StdPressedKey<E>;
    #[inline]
    fn hovered(&self) -> Option<E::WidgetID> { //TODO eventually WidgetIdent return in trait
        self.s.mouse.hovered.as_ref().map(#[inline] |p| p.id.clone() )
    }
    #[inline]
    fn selected(&self) -> Option<E::WidgetID> {
        self.s.kbd.focused.as_ref().map(#[inline] |p| p.id.clone() )
    }
    #[inline]
    fn pressed(&self) -> &[Self::K] {
        &self.s.key.pressed[..]
    }
    #[inline]
    fn cursor_pos(&self) -> Option<Offset> {
        self.s.mouse.pos
    }
    
}

impl<S,E> DynState<E> for StdHandler<S,E> where
    S: Handler<E>,
    E: Env,
    E::WidgetID: Eq + Hash,
    for<'a> E::Context<'a>: AsRefMut<Self> + CtxStdState<E>,
    EEvent<E>: StdVarSup<E>
{
    fn remote_state_or_default<T>(&self, i: E::WidgetID) -> T where T: Default + Clone + 'static {
        self.s.remote_states
            .get(&(i,TypeId::of::<T>()))
            .map_or_else(T::default,#[inline] |v|
                v
                .downcast_ref::<T>()
                .unwrap()
                .clone()
            )
    }
    fn push_remote_state<T>(&mut self, i: E::WidgetID, v: T) where T: 'static {
        self.s.remote_states
            .insert((i,TypeId::of::<T>()),Box::new(v)); //TODO do not realloc always
    }
}
