use super::*;

pub trait Widgets<E>: 'static where E: Env {
    fn widget(&self, i: WPSlice<E>) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: WPSlice<E>) -> Option<&mut E::DynWidget>;

    #[inline]
    fn has_widget(&self, i: WPSlice<E>) -> bool {
        self.widget(i).is_some()
    }

    #[deprecated] #[inline] fn tune_path(&self, _i: &mut E::WidgetPath) {}
    #[deprecated] #[inline] fn tune_path_mut(&mut self, _i: &mut E::WidgetPath) {}
}