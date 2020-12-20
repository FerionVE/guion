//! Entry point of the widget tree
use super::*;

/// Implemented on the root of the widget tree
pub trait Widgets<E>: Sized + 'static where E: Env {
    fn widget<'a>(&'a self, i: E::WidgetPath) -> Result<Resolved<'a,E>,()>;
    fn widget_mut<'a>(&'a mut self, i: E::WidgetPath) -> Result<ResolvedMut<'a,E>,()>;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath) -> bool {
        self.widget(i).is_ok()
    }

    fn trace_bounds(&self, ctx: &mut E::Context, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()>;

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}

    #[inline]
    fn with_env<F: Env<Storage=Self>>(&self) -> &F::Storage where Self: Widgets<F> {
        &self
    }
}
#[doc(hidden)]
pub fn resolve_in_root<E: Env>(w: &dyn Widget<E>, p: E::WidgetPath) -> Result<(WidgetRef<'_,E>,E::WidgetPath),()> {
    let r = w.resolve(p.refc())?;
    
    match r {
        Resolvable::Widget(w) => 
            Ok(
                (w,From::from(p))
            ),
        Resolvable::Path(p) => resolve_in_root(w,p),
    }
}
#[doc(hidden)]
pub fn resolve_in_root_mut<E: Env>(w: &mut dyn WidgetMut<E>, p: E::WidgetPath) -> Result<(WidgetRefMut<'_,E>,E::WidgetPath),()> {
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
