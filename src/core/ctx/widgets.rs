use super::*;
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

pub fn resolve_in_root<'a,E: Env>(w: &'a E::DynWidget, p: WPSlice<E>) -> Option<&'a E::DynWidget> {
    todo!()
}

pub fn resolve_in_root_mut<'a,E: Env>(w: &'a mut E::DynWidget, p: WPSlice<E>) -> Option<&'a mut E::DynWidget> {
    todo!()
}