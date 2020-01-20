use super::*;

pub trait Queue<E>: Sized where E: Env, E::Context: Context<E,Queue=Self> {
    fn wake(&self);
    fn enqueue_render(&self, force: bool);
    fn enqueue_event(&self, e: EEvent<E>);
    fn euqueue_widget_mut(&self, f: impl FnOnce(&mut E::DynWidget));
}

pub trait Enqueue<E,I>: Queue<E> where E: Env, E::Context: Context<E,Queue=Self> {
    fn enqueue(&self, i: I);
}

type DynWidgetMut<E: Env> = Box<dyn FnOnce(&mut E::DynWidget)>;