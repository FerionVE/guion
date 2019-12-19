use super::*;

pub trait AsHandler<H,C>: Sized where C: Context<Link=Self>, H: Handler<C>, C::Link: AsHandler<C::Handler,C> {
    fn as_mut(c: &mut C) -> &mut H;
    fn as_ref(c: &C) -> &H;
}