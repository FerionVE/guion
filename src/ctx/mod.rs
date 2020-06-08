//! The Context trait housing handlers, queue and other side stuff
use super::*;

pub mod queue;
pub mod clipboard;

/// The Context contains the Handlers, the queue and other side data and is also the entry point for most actions.
/// A Context is regularly referenced in parallel with the widget tree
pub trait Context<E>: Sized + 'static where E: Env<Context=Self> {
    type Handler: Handler<E>;
    type Queue: Queue<StdEnqueueable<E>,StdOrder>;

    #[inline] 
    fn handler_mut<H: Handler<E>>(&mut self) -> &mut H where Self: AsRefMut<H> {
        Self::as_mut(self)
    }
    #[inline] 
    fn handler<H: Handler<E>>(&self) -> &H where Self: AsRefMut<H> {
        Self::as_ref(self)
    }

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    fn _handler_mut(&mut self) -> &mut Self::Handler;
    fn _handler(&self) -> &Self::Handler;

    #[inline] 
    fn render(&mut self, w: Resolved<E>, r: &mut RenderLink<E>) {
        Self::Handler::_render(self.link(w),r)
    }
    #[inline] 
    fn event(&mut self, w: Resolved<E>, e: (EEvent<E>,&Bounds,u64)) {
        Self::Handler::_event(self.link(w),e)
    }
    #[inline] 
    fn size(&mut self, w: Resolved<E>) -> ESize<E> {
        Self::Handler::_size(self.link(w))
    }
    #[inline] 
    fn _event_root(&mut self, w: Resolved<E>, e: (EEvent<E>,&Bounds,u64)) {
        Self::Handler::_event_root(self.link(w),e)
    }

    #[inline] fn link<'l: 's,'s>(&'s mut self, w: Resolved<'l,E>) -> Link<'s,E> {
        Link{
            ctx: self,
            widget: w.short_lt(),
        }
    }

    #[inline] fn state_mut(&mut self) -> &mut ECStateful<E> where Self: AsHandlerStateful<E> {
        Self::stateful_mut(self)
    }
    #[inline] fn state(&self) -> &ECStateful<E> where Self: AsHandlerStateful<E> {
        Self::stateful(self)
    }

    fn default_style(&self) -> &EStyle<E>;
    fn default_border(&self) -> &Border;
}