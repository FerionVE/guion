//! Entry point of the widget tree
use super::*;

/// Implemented on the root of the widget tree  
/// Represents the root of a widget tree and being a starting point for widget resolving
pub trait Widgets<E>: Sized + 'static where E: Env {
    /// Resolve Widget by path  
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) Implementations often can just call [`resolve_in_root`](resolve_in_root)
    fn widget(&self, i: E::WidgetPath) -> Result<Resolved<E>,GuionError<E>>;
    /// Resolve Widget by path  
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) Implementations often can just call [`resolve_in_root_mut`](resolve_in_root_mut)
    fn widget_mut(&mut self, i: E::WidgetPath) -> Result<ResolvedMut<E>,GuionError<E>>;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath) -> bool {
        self.widget(i).is_ok()
    }

    fn trace_bounds(&self, ctx: &mut E::Context, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,GuionError<E>>;

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}

    #[inline]
    fn with_env<F: Env<Storage=Self>>(&self) -> &F::Storage where Self: Widgets<F> {
        &self
    }
}
//#[doc(hidden)]
/// Used by [`Widgets::widget`](Widgets::widget) implementations
pub fn resolve_in_root<E: Env>(w: &dyn Widget<E>, p: E::WidgetPath) -> Result<(WidgetRef<E>,E::WidgetPath),GuionError<E>> {
    let r = w.resolve(p.refc())?;
    
    match r {
        Resolvable::Widget(w) => 
            Ok(
                (w,From::from(p))
            ),
        Resolvable::Path(p) => resolve_in_root(w,p),
    }
}
//#[doc(hidden)]
/// Used by [`Widgets::widget_mut`](Widgets::widget_mut) implementations
pub fn resolve_in_root_mut<E: Env>(w: &mut dyn WidgetMut<E>, p: E::WidgetPath) -> Result<(WidgetRefMut<E>,E::WidgetPath),GuionError<E>> {
    let final_path = resolve_in_root::<E>(w.base(),p)
        .map(#[inline] |e| e.1 )?;

    Ok((
        w.resolve_mut(final_path.refc())
            .unwrap()
            .as_widget()
            .unwrap_nodebug(),
        final_path
    ))
}
