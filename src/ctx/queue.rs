//! Queue trait and util fns for implementors
use super::*;

/// The Queue accesible from the Context, used to enqueue events or actions from any thread
/// invalidations are always done right before rendering
/// validations are always done right after rendering
pub trait Queue<I,O> { //TODO probably remove mandantory StdEnqueueable bound
    fn push(&mut self, v: I, order: O, prio: i64);
    fn send(&self, v: I, order: O, prio: i64);
}

pub enum StdEnqueueable<E> where E: Env {
    Render{force: bool},
    Event{event: EEvent<E>, ts: u64},
    MutateWidget{path: E::WidgetPath, f: fn(WidgetRef<E>,&mut E::Context,E::WidgetPath)},
    MutateWidgetClosure{path: E::WidgetPath, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context,E::WidgetPath)+'static>},
    TryMutateWidget{path: E::WidgetPath, f: fn(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)},
    TryMutateWidgetClosure{path: E::WidgetPath, f: Box<dyn FnOnce(WidgetRefMut<E>,&mut E::Context,E::WidgetPath)+'static>},
    MutateRoot{f: fn(&mut E::Storage,&mut E::Context)},
    MutateRootClosure{f: Box<dyn FnOnce(&mut E::Storage,&mut E::Context)+'static>},
    AccessWidget{path: E::WidgetPath, f: fn(WidgetRef<E>,&mut E::Context)},
    AccessWidgetClosure{path: E::WidgetPath, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context)+'static>},
    AccessRoot{f: fn(&E::Storage,&mut E::Context)},
    AccessRootClosure{f: Box<dyn FnOnce(&E::Storage,&mut E::Context)+'static>},
    MutMessage{path: E::WidgetPath, msg: E::Message},
    InvalidateWidget{path: E::WidgetPath},
    ValidateWidgetRender{path: E::WidgetPath},
    ValidateWidgetSize{path: E::WidgetPath, size: ESize<E>},
}

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

/// to be executed by the queue impl, always DIRECTLY before rendering
#[deprecated]
pub fn invalidate<E: Env>(stor: &mut E::Storage, i: E::WidgetPath) -> Result<(),GuionError<E>> {
    stor.widget_mut(i)?.mutate()?._set_invalid(true);
    Ok(())
}
#[deprecated]
/// to be executed by the queue impl, always DIRECTLY after rendering
pub fn validate<E: Env>(stor: &mut E::Storage, i: E::WidgetPath) -> Result<(),GuionError<E>> {
    stor.widget_mut(i)?.mutate()?._set_invalid(false);
    Ok(())
}
