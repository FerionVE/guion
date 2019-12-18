use crate::core::ctx::*;

pub trait Queue<E> where E: Env {
    type Callback;
    type Args;
    type Return;

    fn add(&mut self, a: Self::Args, f: Self::Callback) -> Self::Return;
}

pub trait AccessQueue<Q,E>: Context<E::HDeref> where Q: Queue<E>, E: Env<Context=Self>, Self: Widgets<E>, E::HDeref: for<'a> From<&'a mut Self> {
    fn queue_mut(&mut self) -> &mut Q;
}