//! The [`Context`] trait housing handlers, queue and other side stuff
use std::ops::DerefMut;

use super::*;

pub mod queue;
pub mod clipboard;

/// The Context contains the [`Handlers`](Handler), the [`Queue`] and other side data and is also the entry point for most actions.  
/// A Context is regularly referenced in parallel with the [widget tree](Env::Storage)
pub trait Context<E>: Sized where E: Env {
    type Handler: HandlerBuilder<E>;
    type Queue: Queue<StdEnqueueable<E>,StdOrder>;

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    //TODO this can't be done by Context impls without violating variance unless being 'static
    fn lt_mut<'a>(&mut self) -> &mut E::Context<'a> where Self: 'a;

    fn build_handler(&mut self) -> <Self::Handler as HandlerBuilder<E>>::Built where Self: Sized;

    #[inline]
    fn link<'s,'l: 's,'t: 's,'cc: 's>(&'s mut self, w: Resolved<'t,E>) -> Link<'s,'cc,E> where Self: 'cc {
        Link{
            ctx: self.lt_mut(),
            widget: w.lt(),
        }
    }
}
