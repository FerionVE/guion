use super::*;

pub trait DynState<E>: 'static where E: Env {
    fn remote_state_or_default<T>(&self, id: E::WidgetID) -> T where T: Default + Clone + 'static;
    fn push_remote_state<T>(&mut self, id: E::WidgetID, v: T) where T: 'static;
}
