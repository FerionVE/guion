use super::*;
use std::rc::Rc;
/// Implemented on the root of the widget tree
pub trait Widgets<E>: Sized + 'static where E: Env {
    fn widget<'a>(&'a self, i: E::WidgetPath) -> Result<Resolved<'a,E>,()>;
    #[inline]
    fn widget_mut<'a>(&'a mut self, i: E::WidgetPath) -> Result<ResolvedMut<'a,E>,()> {
        self._widget_mut(i,true)
    }
    fn _widget_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvedMut<'a,E>,()>;

    #[inline]
    fn has_widget(&self, i: E::WidgetPath) -> bool {
        self.widget(i).is_ok()
    }

    fn trace_bounds(&self, ctx: &mut E::Context, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()>;

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}

    #[inline]
    fn with_env<F: Env<Storage=Self>>(&self) -> &F::Storage where Self: Widgets<F> {
        &self
    }
}

pub fn resolve_in_root<'a,E: Env>(w: &'a E::DynWidget, p: E::WidgetPath) -> Option<(Rc<WidgetRef<'a,E>>,E::WidgetPath)> {
    let r = w.resolve(p.refc());
    let r = r.ok();

    //TODO macro for this
    if r.is_none() {return None;}
    let r = r.unwrap();
    
    match r {
        Resolvable::Widget(w) => 
            Some(
                (w,From::from(p))
            ),
        Resolvable::Path(p) => resolve_in_root(w,p),
    }
}

pub fn resolve_in_root_mut<'a,E: Env>(w: &'a mut E::DynWidget, p: E::WidgetPath, invalidate: bool) -> Option<(WidgetRefMut<'a,E>,E::WidgetPath)> {
    let path = resolve_in_root::<E>(w,p).map(|e| e.1 );

    if path.is_none() {return None;}
    let path = path.unwrap();

    Some((
        w.resolve_mut(path.refc(),invalidate)
            .unwrap()
            .as_widget()
            .unwrap_nodebug(),
        path
    ))
}

/*pub fn resolve_in_root_mutt<'a,E: Env>(w: &'a mut E::DynWidget, p: E::WidgetPath, invalidate: bool) -> Option<(WidgetRefMut<'a,E>,E::WidgetPath)> {
    fn dummy<'b,E: Env>(w: &'b mut E::DynWidget, p: E::WidgetPath) -> ResolvableMut<'b,E> {
        todo!()
    }

    /*match dummy(w,p) {
        ResolvableMut::Widget(w) => return Some((w,p.unslice().into())),
        ResolvableMut::Path(p) => resolve_in_root_mutt(w,p,invalidate),
    }*/

    let path = {
        match dummy(w,p) {
            ResolvableMut::Widget(w) => None,
            ResolvableMut::Path(p) => Some(p),
        }
    };
    let path = path
        .as_ref()
        .map(|path| path )
        .unwrap_or(p);
    resolve_in_root_mutt(w,path,invalidate)
}*/