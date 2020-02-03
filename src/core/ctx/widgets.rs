use super::*;
use std::rc::Rc;
/// Implemented on the root of the widget tree
pub trait Widgets<E>: Sized + 'static where E: Env {
    fn widget<'a>(&'a self, i: WPSlice<E>) -> Result<Resolved<'a,E>,()>;
    fn widget_mut<'a>(&'a mut self, i: WPSlice<E>) -> Result<Resolved<'a,E>,()>;

    #[inline]
    fn has_widget(&self, i: WPSlice<E>) -> bool {
        self.widget(i).is_ok()
    }

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}

    #[inline]
    fn with_env<F: Env<Storage=Self>>(&self) -> &F::Storage where Self: Widgets<F> {
        &self
    }
}

pub fn resolve_in_root<'a,E: Env>(w: &'a E::DynWidget, p: WPSlice<E>) -> Option<(Rc<WidgetRef<'a,E>>,EWPRc<E>)> {
    let r = w.resolve(p);
    let r = r.ok();

    //TODO macro for this
    if r.is_none() {return None;}
    let r = r.unwrap();
    
    match r {
        Resolvable::Widget(w) => 
            Some(
                (w,From::from(E::WidgetPath::from_slice(p)))
            ),
        Resolvable::Path(p) => resolve_in_root(w,p.slice()),
    }
}

pub fn resolve_in_root_mut<'a,E: Env>(w: &'a mut E::DynWidget, p: WPSlice<E>) -> Option<WidgetRefMut<'a,E>> {
    let path = resolve_in_root(w,p).map(|e| e.1 );

    if path.is_none() {return None;}
    let path = path.unwrap();

    Some(w.resolve_mut(path.slice()).unwrap())
}