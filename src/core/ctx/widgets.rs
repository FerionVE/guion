use super::*;

pub trait Widgets<E>: 'static where E: Env {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;

    #[inline]
    fn has_widget(&self, i: &E::WidgetID) -> bool {
        self.widget(i).is_some()
    }

    #[inline] fn tune_id(&self, _i: &mut E::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut E::WidgetID) {}
}