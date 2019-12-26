use super::*;

pub trait Queue<E> where E: Env {
    type Callback;
    type Args;
    type Return;

    fn add(&mut self, a: Self::Args, f: Self::Callback) -> Self::Return;
}

pub trait AccessQueue<Q,E>: Context where Q: Queue<E>, E: Env<Context=Self>, Self: Widgets<E> {
    fn queue_mut(&mut self) -> &mut Q;
}