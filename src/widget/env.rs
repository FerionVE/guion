use crate::util::as_any::AsAny;
use crate::event::Event;
use crate::render::Render;
use crate::widget::Widget;

pub trait Env: Sized + Clone {
    type Renderer: Render;
    type Event: Event;
    type DynWidget: AsAny + Widget<Self> + ?Sized;
    type WidgetID: Eq + Clone;
    type Commit: Eq + Ord;
    type Stor: WidgetStore<Self> + 'static;
    type Ctx: Context<Self>;
}

pub trait WidgetStore<E> where E: Env {
    fn get(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn get_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;
    
    fn add(&mut self, w: Box<E::DynWidget>) -> E::WidgetID;
    fn pop(&mut self, i: &E::WidgetID) -> Option<Box<E::DynWidget>>;
}

pub trait Context<E> where E: Env {
    fn widgets(&self) -> &E::Stor;
    fn widgets_mut(&mut self) -> &mut E::Stor;
}