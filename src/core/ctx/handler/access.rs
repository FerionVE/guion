use super::*;

pub trait AsHandler<H,E>: Sized where E: Env, H: Handler<E> {
    fn as_mut(c: &mut E::Context) -> &mut H;
    fn as_ref(c: &E::Context) -> &H;
}