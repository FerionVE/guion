use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub trait Env: Sized + Clone {
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: Clone;
    type Commit: Eq + Ord;
    type Ctx: Context<Self>;
}

pub trait Context<E>: Sized where E: Env<Ctx=Self> {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;

    fn tune_id(&self, i: &mut E::WidgetID) {}
    fn tune_id_mut(&mut self, i: &mut E::WidgetID) {}
    
    #[inline] fn render_widget(&mut self, r: E::Renderer, i: &E::WidgetID, f: fn(Link<E>, E::Renderer)) {
        f(self.link(i.clone()), r);
    }
    #[inline] fn event_widget(&mut self, e: E::Event, i: &E::WidgetID, f: fn(Link<E>, E::Event)) {
        f(self.link(i.clone()), e);
    }
    #[inline] fn size_widget(&mut self, i: &E::WidgetID, f: fn(Link<E>)->Size) -> Size {
        f(self.link(i.clone()))
    }

    #[inline] fn link<'a>(&'a mut self, i: E::WidgetID) -> Link<'a,E> {
        Link{
            ctx: self,
            widget_id: i
        }
    }
}