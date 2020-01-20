use std::any::Any;

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

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(&mut self, s: &E::Storage, i: WPSlice<E>, r: (&mut ERenderer<E>,&Bounds)) {
        Self::Handler::_render(self.link(s,i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(&mut self, s: &E::Storage, i: WPSlice<E>, e: (EEvent<E>,&Bounds)) {
        Self::Handler::_event(self.link(s,i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(&mut self, s: &E::Storage, i: WPSlice<E>) -> Size {
        Self::Handler::_size(self.link(s,i))
    }

    #[inline] fn link<'a>(&'a mut self, s: &'a E::Storage, i: WPSlice<'a,E>) -> Link<'a,E> {
        Link{
            stor: s,
            ctx: self,
            path: i,
        }
    }

    #[inline] fn state(&self) -> &ECStateful<E> where Self::Handler: AsHandlerStateful<E> {
        Self::Handler::stateful(self)
    }
}