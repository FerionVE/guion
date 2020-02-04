use super::*;

/// The Queue accesible from the Context, used to enqueue events or actions from any thread
pub trait Queue<E>: Sized + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn wake(&self);
    fn enqueue_render(&self, force: bool);
    fn enqueue_event(&self, e: EEvent<E>);
    fn enqueue_widget_mut(&self, path: WPSlice<E>, f: impl FnOnce(&mut E::DynWidget));
    fn enqueue_widget(&self, path: WPSlice<E>, f: impl FnOnce(&E::DynWidget));
}

/// queue enqueue support trait
pub trait Enqueue<E,I>: Queue<E> + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn enqueue(&self, i: I);
}

#[allow(type_alias_bounds)]
type DynWidgetMut<E: Env> = Box<dyn FnOnce(&mut E::DynWidget)>;
