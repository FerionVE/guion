use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub trait Env: Sized + Clone {
    type Renderer: Render;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: Clone;
    type Commit: Eq + Ord;
    type Ctx: Context<Self>;
}

pub trait Context<E> where E: Env {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;

    fn tune_id(&self, i: &mut E::WidgetID) {}
    fn tune_id_mut(&mut self, i: &mut E::WidgetID) {}
}