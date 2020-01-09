pub use super::*;

pub trait WPProvider<E> where E: Env {
    fn path(&self, parent: WPSlice<E>) -> E::WidgetPath;
    fn id_eq(&self, id: &E::WidgetID) -> bool;
    fn widget_if_id_eq<'a>(&'a self, id: &EWPSub<E>) -> Option<&'a E::DynWidget>;
    fn widget_if_id_eq_mut<'a>(&'a mut self, id: &EWPSub<E>) -> Option<&'a mut E::DynWidget>;
}

impl<E,T> WPProvider<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn path(&self, parent: WPSlice<E>) -> E::WidgetPath {
        self.self_in_parent(parent)
    }
    #[inline]
    fn id_eq(&self, id: &E::WidgetID) -> bool {
        &self.id() == id
    }
    #[inline]
    fn widget_if_id_eq<'a>(&'a self, id: &EWPSub<E>) -> Option<&'a E::DynWidget> {
        self.is_subpath(id).map(#[inline] move || DynWidget::erase(self) )
    }
    #[inline]
    fn widget_if_id_eq_mut<'a>(&'a mut self, id: &EWPSub<E>) -> Option<&'a mut E::DynWidget> {
        self.is_subpath(id).map(#[inline] move || DynWidget::erase_mut(self) )
    }
}