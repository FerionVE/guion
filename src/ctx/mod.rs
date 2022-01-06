//! The [`Context`] trait housing handlers, queue and other side stuff
use std::ops::DerefMut;

use super::*;

pub mod queue;
pub mod clipboard;

/// The Context contains the [`Handlers`](Handler), the [`Queue`] and other side data and is also the entry point for most actions.  
/// A Context is regularly referenced in parallel with the [widget tree](Env::Storage)
pub trait Context<E>: Sized where E: Env {
    type Handler: Handler<E>;
    type Queue: Queue<StdEnqueueable<E>,StdOrder>;

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    //TODO this can't be done by Context impls without violating variance unless being 'static
    fn lt_mut<'a>(&mut self) -> &mut E::Context<'a> where Self: 'a;

    #[inline] 
    fn render(&mut self, w: Resolved<E>, r: &mut ERenderer<'_,E>) {
        Self::Handler::_render(self.link(w),r)
    }
    #[inline] 
    fn event_direct(&mut self, w: Resolved<E>, e: &EventCompound<E>) -> EventResp {
        Self::Handler::_event_direct(self.link(w),e)
    }
    #[inline]
    fn send_event(&mut self, w: Resolved<E>, e: &EventCompound<E>, child: E::WidgetPath) -> Result<EventResp,E::Error> {
        Self::Handler::_send_event(self.link(w),e,child)
    }
    #[inline] 
    fn size(&mut self, w: Resolved<E>, e: &EStyle<E>) -> ESize<E> {
        Self::Handler::_size(self.link(w),e)
    }
    #[inline] 
    fn _event_root(&mut self, w: Resolved<E>, e: &EventCompound<E>) -> EventResp {
        Self::Handler::_event_root(self.link(w),e)
    }

    #[inline]
    fn link<'s,'l: 's,'t: 's,'cc: 's>(&'s mut self, w: Resolved<'t,E>) -> Link<'s,'cc,E> where Self: 'cc {
        Link{
            ctx: self.lt_mut(),
            widget: w.lt(),
        }
    }
}
