use crate::core::ctx::Context;

pub trait Queue<E> where E: Context {
    type Callback;
    type Args;
    type Return;

    fn add(&mut self, a: Self::Args, f: Self::Callback) -> Self::Return;
}

pub trait AccessQueue<Q>: Context where Q: Queue<Self> {
    fn queue_mut(&mut self) -> &mut Q;
}