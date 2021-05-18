//! Entry point of the widget tree
use super::*;

/// Implemented on the root of the widget tree  
/// Represents the root of a widget tree and being a starting point for widget resolving
pub trait Widgets<E>: Sized where E: Env {
    /// Resolve Widget by [path](WidgetPath)
    /// 
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) Implementations often can just call [`resolve_in_root`](resolve_in_root)
    fn widget(&self, i: E::WidgetPath) -> Result<Resolved<E>,E::Error>;
    /// Resolve Widget by [path](WidgetPath)
    /// 
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square) Implementations often can just call [`resolve_in_root_mut`](resolve_in_root_mut)
    fn widget_mut(&mut self, i: E::WidgetPath) -> Result<ResolvedMut<E>,E::Error>;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath) -> bool {
        self.widget(i).is_ok()
    }

    #[deprecated] 
    fn trace_bounds(&self, ctx: &mut E::Context, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error>;

    fn lt_ref<'l,'r,'s>(&'r self) -> &'r E::Storage<'s> where 's: 'r, 'l: 'r, 'l: 's, Self: 'l;
    fn lt_mut<'l,'r,'s>(&'r mut self) -> &'r mut E::Storage<'s> where 's: 'r, 'l: 'r, 'l: 's, Self: 'l;

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}
}
//#[doc(hidden)]
/// Used by [`Widgets::widget`] implementations
pub fn resolve_in_root<'l,'s,E>(root: &'s dyn Widget<E>, sub: E::WidgetPath, abs_path: E::WidgetPath, stor: &'l E::Storage<'_>) -> Result<Resolved<'s,E>,E::Error> where E: Env, 'l: 's {
    let r = root.resolve(sub.refc())?;
    
    match r {
        Resolvable::Widget(w) => 
            Ok(Resolved {
                wref: w,
                path: abs_path.clone(),
                direct_path: abs_path,
                stor: stor.lt_ref(),
            }),
        Resolvable::Path(p) => {
            let mut r = stor.widget(p)?;
            r.path = abs_path;
            Ok(r)
        },
    }
}
//#[doc(hidden)]
/// Used by [`Widgets::widget_mut`](Widgets::widget_mut) implementations
pub fn resolve_in_root_mut<'l:'s,'s,E: Env>(
    stor: &'l mut E::Storage<'l>,
    mut root_in_stor_mut: impl for<'a> FnMut(&'a mut E::Storage<'l>) -> &'a mut dyn WidgetMut<E>,
    sub: E::WidgetPath, abs_path: E::WidgetPath,
) -> Result<ResolvedMut<'s,E>,E::Error> {

    let final_path;

    match root_in_stor_mut(stor).resolve_mut(sub.clone())? {
        ResolvableMut::Widget(w) => 
            final_path = Ok(abs_path.clone()),
        ResolvableMut::Path(p) => 
            final_path = Err(p),
    }

    match final_path {
        Ok(p) => {
            let w = root_in_stor_mut(stor).resolve_mut(sub)?
                .as_widget().unwrap();
            Ok(ResolvedMut {
                wref: w,
                path: p.clone(),
                direct_path: p,
            })
        },
        Err(p) => 
            stor.widget_mut(p)
                .map(|mut r| {
                    r.path = abs_path;
                    r
                }),
    }
}
