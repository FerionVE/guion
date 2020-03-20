//! Queue trait and util fns for implementors
use super::*;

/// The Queue accesible from the Context, used to enqueue events or actions from any thread
/// invalidations are always done right before rendering
/// validations are always done right after rendering
pub trait Queue<I> { //TODO probably remove mandantory StdEnqueueable bound
    fn push(&mut self, v: I);
    fn send(&self, v: I);
}

pub enum StdEnqueueable<E> where E: Env {
    Render{force: bool},
    Event{event: EEvent<E>, ts: u64},
    MutateWidget{path: E::WidgetPath, f: fn(WidgetRefMut<E>,&mut E::Context), invalidate: bool},
    MutateWidgetClosure{path: E::WidgetPath, f: Box<dyn FnOnce(WidgetRefMut<E>,&mut E::Context)+'static>, invalidate: bool},
    MutateRoot{f: fn(&mut E::Storage,&mut E::Context)},
    MutateRootClosure{f: Box<dyn FnOnce(&mut E::Storage,&mut E::Context)+'static>},
    AccessWidget{path: E::WidgetPath, f: fn(WidgetRef<E>,&mut E::Context)},
    AccessWidgetClosure{path: E::WidgetPath, f: Box<dyn FnOnce(WidgetRef<E>,&mut E::Context)+'static>},
    AccessRoot{f: fn(&E::Storage,&mut E::Context)},
    AccessRootClosure{f: Box<dyn FnOnce(&E::Storage,&mut E::Context)+'static>},
    InvalidateWidget{path: E::WidgetPath},
    ValidateWidgetRender{path: E::WidgetPath},
    ValidateWidgetSize{path: E::WidgetPath, size: ESize<E>},
}

/// to be executed by the queue impl, always DIRECTLY before rendering
pub fn invalidate<E: Env>(stor: &mut E::Storage, i: E::WidgetPath) -> Result<(),()> {
    stor._widget_mut(i,true)?;
    Ok(())
}
/// to be executed by the queue impl, always DIRECTLY after rendering
pub fn validate<E: Env>(stor: &mut E::Storage, i: E::WidgetPath) -> Result<(),()> {
    let mut w = stor._widget_mut(i,false)?;
    w.widget().set_invalid(false);
    Ok(())
}
