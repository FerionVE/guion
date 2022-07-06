//! The [`Context`] trait housing handlers, queue and other side stuff
use super::*;

pub mod queue;
pub mod clipboard;

/// The Context contains the [`Handlers`](Handler), the [`Queue`] and other side data and is also the entry point for most actions.  
/// A Context is regularly referenced in parallel with the [widget tree](Env::Storage)
pub trait Context<'cc,E>: Sized + 'cc where E: Env {
    type Handler: HandlerBuilder<E>;
    type Queue: Queue<StdEnqueueable<E>,StdOrder>;

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    fn lt_mut(&mut self) -> &mut E::Context<'cc> where Self: 'cc;

    fn build_handler(&mut self) -> <Self::Handler as HandlerBuilder<E>>::Built where Self: Sized;

    // #[inline]
    // fn link<'o,'s:'o,'t:'o>(&'s mut self, w: Resolved<'t,E>) -> Link<'o,'cc,E> where 'cc: 'o, Self: 'cc {
    //     Link{
    //         ctx: self.lt_mut(),
    //         widget: w.lt(),
    //     }
    // }
}
