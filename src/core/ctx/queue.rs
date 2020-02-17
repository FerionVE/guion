use super::*;

/// The Queue accesible from the Context, used to enqueue events or actions from any thread
/// invalidations are always done right before rendering
/// validations are always done right after rendering
pub trait Queue<E>: Sized + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn wake(&self);
    fn enqueue_render(&self, force: bool);
    fn enqueue_event(&self, e: EEvent<E>);
    fn enqueue_widget_mut(&self, path: WPSlice<E>, f: fn(&mut E::DynWidget), invalidate: bool);
    fn enqueue_widget(&self, path: WPSlice<E>, f: fn(&E::DynWidget));
    fn enqueue_widget_mut_closure(&self, path: WPSlice<E>, f: impl FnOnce(&mut E::DynWidget), invalidate: bool);
    fn enqueue_widget_closure(&self, path: WPSlice<E>, f: impl FnOnce(&E::DynWidget));
    fn enqueue_widget_invalidate(&self, path: WPSlice<E>);
    fn enqueue_widget_validate(&self, path: WPSlice<E>);
}

/// queue enqueue support trait
pub trait Enqueue<E,I>: Queue<E> + Sync where E: Env, E::Context: Context<E,Queue=Self> {
    fn enqueue(&self, i: I);
}

#[allow(type_alias_bounds)]
type DynWidgetMut<E: Env> = Box<dyn FnOnce(&mut E::DynWidget)>;

/// to be executed by the queue impl, always DIRECTLY before rendering
pub fn invalidate<E: Env>(stor: &mut E::Storage, i: WPSlice<E>) -> Result<(),()> {
    stor.widget_mut(i,true)?;
    Ok(())
}
/// to be executed by the queue impl, always DIRECTLY after rendering
pub fn validate<E: Env>(stor: &mut E::Storage, i: WPSlice<E>) -> Result<(),()> {
    let mut w = stor.widget_mut(i,false)?;
    w.set_invalid(false);
    Ok(())
}