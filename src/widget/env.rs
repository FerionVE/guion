use crate::util::as_any::AsAny;
use crate::event::Event;
use crate::render::Render;
use crate::widget::Widget;

pub trait Env: Sized + Clone {
    type Renderer: Render;
    type Event: Event;
    ///regularly just dyn Widget
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

    fn me<'a,S: Widget<E> + 'static>(&'a self, me: &E::WidgetID) -> Option<&'a S> {
        self.widgets().get(me)
        .map(|d|
            d.as_any().downcast_ref::<S>().expect("Invalid Widget Downcast Type")
        )
    }

    fn me_mut<'a,S: Widget<E> + 'static>(&'a mut self, me: &E::WidgetID) -> Option<&'a mut S> {
        self.widgets_mut().get_mut(me)
        .map(|d|
            d.as_any_mut().downcast_mut::<S>().expect("Invalid Widget Downcast Type")
        )
    }
}