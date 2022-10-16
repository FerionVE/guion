//! [`Queue`] trait and util fns for implementors
use std::sync::Arc;

use crate::newpath::PathResolvusDyn;
use crate::widget::dyn_tunnel::WidgetDyn;

use super::*;

/// The Queue, accessible from [`E::Context`](Context), used to enqueue [events](Event) or actions from any thread
/// 
/// Invalidations are always done right before rendering  
/// Validations are always done right after rendering  
pub trait Queue<I,O> { //TODO probably remove mandatory StdEnqueueable bound
    fn push(&mut self, v: I, order: O, prio: i64);
    fn send(&self, v: I, order: O, prio: i64);
}

pub enum StdEnqueueable<E> where E: Env {
    Render{force: bool},
    Event{event: EEvent<E>, ts: u64},
    MutateRoot{f: PtrMutEvent<E>},
    MutateRootClosure{f: BoxMutEvent<E>},
    AccessWidget{path: Arc<dyn PathResolvusDyn<E>>, f: PtrAccessWidget<E>},
    AccessWidgetClosure{path: Arc<dyn PathResolvusDyn<E>>, f: BoxAccessWidget<E>},
    AccessRoot{f: PtrAccessRoot<E>},
    AccessRootClosure{f: BoxAccessRoot<E>},
    MutMessage{path: Arc<dyn PathResolvusDyn<E>>, msg: E::Message},
    InvalidateWidget{path: Arc<dyn PathResolvusDyn<E>>},
    ValidateWidgetRender{path: Arc<dyn PathResolvusDyn<E>>},
    ValidateWidgetSize{path: Arc<dyn PathResolvusDyn<E>>, size: ESize<E>},
}

pub type BoxMutEvent<E> = Box<dyn for<'r> FnOnce(<E as Env>::RootMut<'r>,&'r (),&mut <E as Env>::Context<'_>) + Send + Sync + 'static>;
pub type PtrMutEvent<E> = for<'r> fn(<E as Env>::RootMut<'r>,&'r (),&mut <E as Env>::Context<'_>);
pub type BoxAccessWidget<E> = Box<dyn for<'w,'ww,'r> FnOnce(&'w (dyn WidgetDyn<E>+'ww),<E as Env>::RootRef<'r>,&'r (),&mut <E as Env>::Context<'_>) + Send + Sync + 'static>;
pub type PtrAccessWidget<E> = for<'w,'ww,'r> fn(&'w (dyn WidgetDyn<E>+'ww),<E as Env>::RootRef<'r>,&'r (),&mut <E as Env>::Context<'_>);
pub type BoxAccessRoot<E> = Box<dyn for<'r> FnOnce(<E as Env>::RootRef<'r>,&'r (),&mut <E as Env>::Context<'_>) + Send + Sync + 'static>;
pub type PtrAccessRoot<E> = for<'r> fn(<E as Env>::RootRef<'r>,&'r (),&mut <E as Env>::Context<'_>);


/// event ordering in a standard loop
///
/// loop {
///     PreEvents()
///     for event in events {
///         PreEvent()
///         process(event)
///         PostCurrent()
///         PostEvent()
///     }
///     PostEvents()
///     PreRender()
///     render()
///     RenderValidation()
///     PostCurrent()
///     PostRender()
/// }
#[derive(Copy,Clone,Hash,Eq,PartialEq)]
pub enum StdOrder {
    /// before processing events
    PreEvents,
    /// before processing a single event
    PreEvent,
    /// after processing a single event
    PostEvent,
    /// after processing events
    PostEvents,
    /// before rendering
    PreRender,
    /// directly after rendering
    RenderValidation,
    /// after rendering and RenderValidation
    PostRender,

    // after the current pass
    PostCurrent,
}

// /// to be executed by the queue impl, always DIRECTLY before rendering
// #[deprecated]
// pub fn invalidate<E: Env>(stor: &mut E::Storage<'_>, i: E::WidgetPath) -> Result<(),E::Error> {
//     stor.widget_mut(i)
//         .map(#[inline] |mut w| w._set_invalid(true) )
// }
// #[deprecated]
// /// to be executed by the queue impl, always DIRECTLY after rendering
// pub fn validate<E: Env>(stor: &mut E::Storage<'_>, i: E::WidgetPath) -> Result<(),E::Error> {
//     stor.widget_mut(i)
//         .map(#[inline] |mut w| w._set_invalid(false) )
// }
