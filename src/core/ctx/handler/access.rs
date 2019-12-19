use super::*;

pub trait HandlerAccess<'a,C>: AsMut<C> + AsHandler<'a,C::Handler> where C: Context {
    fn from_ctx(c: &'a mut C) -> Self;
}

pub trait AsHandler<'a,H> {
    fn as_mut(&'a mut self) -> &'a mut H;
    fn into_mut(self) -> &'a mut H;
}