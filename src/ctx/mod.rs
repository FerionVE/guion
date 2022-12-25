//! The [`Context`] trait housing handlers, queue and other side stuff

use crate::env::Env;
use crate::intercept::InterceptBuilder;
use crate::widget::id::WidgetID;

use self::queue::{BoxMutEvent, StdEnqueueable, StdOrder, Queue};

pub mod queue;
pub mod clipboard;

/// The Context contains the [`Handlers`](Handler), the [`Queue`] and other side data and is also the entry point for most actions.  
/// A Context is regularly referenced in parallel with the [widget tree](Env::Storage)
pub trait Context<'cc,E>: Sized + 'cc where E: Env {
    type Intercept: InterceptBuilder<E>;
    type Queue: Queue<StdEnqueueable<E>,StdOrder>;

    fn queue_mut(&mut self) -> &mut Self::Queue;
    fn queue(&self) -> &Self::Queue;

    fn lt_mut(&mut self) -> &mut E::Context<'cc> where Self: 'cc;

    fn build_intercept(&mut self) -> <Self::Intercept as InterceptBuilder<E>>::Built where Self: Sized;

    // #[inline]
    // fn link<'o,'s:'o,'t:'o>(&'s mut self, w: Resolved<'t,E>) -> Link<'o,'cc,E> where 'cc: 'o, Self: 'cc {
    //     Link{
    //         ctx: self.lt_mut(),
    //         widget: w.lt(),
    //     }
    // }

    #[deprecated="TODO better queue shorthands"]
    fn mutate_closure(&mut self, closure: BoxMutEvent<E>) {
        self.queue_mut().push(
            StdEnqueueable::MutateRootClosure { f: closure },
            StdOrder::PostCurrent,
            0,
        );
    }

    fn retained_id(&mut self) -> WidgetID;

    #[cfg(feature = "qcell")]
    fn tcell_owner(&self) -> &qcell::TCellOwner<E::CtxTCellOwner>;

    #[cfg(feature = "qcell")]
    fn tcell_owner_mut(&mut self) -> &mut qcell::TCellOwner<E::CtxTCellOwner>;
}
