use super::*;

//pub mod ctx_meta; TODO fix CtxMeta
//pub use ctx_meta::*;

#[allow(type_alias_bounds)]
pub mod aliases;
use aliases::*;

pub mod widgets;
pub use widgets::*;

pub mod queue;
pub use queue::*;

pub mod handler;
pub use handler::*;

pub mod resolved;
pub use resolved::*;

pub trait Context<E>: Sized + 'static where E: Env<Context=Self> {
    type Handler: Handler<E>;
    type Queue: Queue<E>;
    //type Meta: ContextMeta;

    #[inline] 
    fn handler_mut<H: Handler<E>>(&mut self) -> &mut H where Self::Handler: AsHandler<H,E> {
        Self::Handler::as_mut(self)
    }
    #[inline] 
    fn handler<H: Handler<E>>(&self) -> &H where Self::Handler: AsHandler<H,E> {
        Self::Handler::as_ref(self)
    }

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    fn _handler_mut(&mut self) -> &mut Self::Handler;
    fn _handler(&self) -> &Self::Handler;

    #[inline] 
    fn render(&mut self, w: Resolved<E>, r: (&mut ERenderer<E>,&Bounds)) {
        Self::Handler::_render(self.link(w),r)
    }
    #[inline] 
    fn event(&mut self, w: Resolved<E>, e: (EEvent<E>,&Bounds)) {
        Self::Handler::_event(self.link(w),e)
    }
    #[inline] 
    fn size(&mut self, w: Resolved<E>) -> ESize<E> {
        Self::Handler::_size(self.link(w))
    }

    #[inline] fn link<'a>(&'a mut self, w: Resolved<'a,E>) -> Link<'a,E> {
        Link{
            ctx: self,
            widget: w,
        }
    }

    #[inline] fn state(&self) -> &ECStateful<E> where Self::Handler: AsHandlerStateful<E> {
        Self::Handler::stateful(self)
    }
}