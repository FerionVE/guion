use super::*;

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

    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}