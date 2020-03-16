use super::*;

/// The Queue accesible from the Context, used to enqueue events or actions from any thread
/// invalidations are always done right before rendering
/// validations are always done right after rendering
pub trait Queue<E>: Sized + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn wake(&self);
    fn enqueue_render(&mut self, force: bool);
    fn enqueue_event(&mut self, e: (EEvent<E>,&Bounds,u64));
    fn enqueue_widget_mut(&mut self, path: E::WidgetPath, f: fn(&mut dyn WidgetMut<E>), invalidate: bool);
    fn enqueue_widget(&mut self, path: E::WidgetPath, f: fn(&dyn Widget<E>));
    fn enqueue_widget_mut_closure(&mut self, path: E::WidgetPath, f: impl FnOnce(&mut dyn WidgetMut<E>)+Sync+'static, invalidate: bool);
    fn enqueue_widget_closure(&mut self, path: E::WidgetPath, f: impl FnOnce(&dyn Widget<E>)+Sync+'static);
    fn enqueue_widget_invalidate(&mut self, path: E::WidgetPath);
    fn enqueue_widget_validate(&mut self, path: E::WidgetPath);
}

/// queue enqueue support trait
pub trait Enqueue<E,I>: Queue<E> + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn enqueue(&mut self, i: I);
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
