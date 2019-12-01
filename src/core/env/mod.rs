use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub trait Env: Sized + Clone {
    type Renderer: Render;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID + Clone;
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

    fn next_id(&mut self) -> E::WidgetID;
}

pub trait WidgetID {
    fn set_idx(&mut self, _i: Option<usize>) {}

    fn get_idx(&self) -> Option<usize> {None}
    
    ///Do NOT eq the inserted idx
    fn eq(&self, o: &Self) -> bool;
}