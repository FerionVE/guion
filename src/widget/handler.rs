use crate::widget::link::Link;
use crate::widget::env::Env;

pub struct Handler<E> where E: Env {
    pub(crate) own_id: E::WidgetID,
    pub(crate) fns: HandlerFns<E>,
}

pub struct HandlerFns<E> where E: Env {
    pub render: fn(Link<E>, E::Renderer),
    pub event: fn(Link<E>, E::Event),
}

impl<E> Handler<E> where E: Env {
    pub fn render(&mut self, c: &mut E::Ctx, r: E::Renderer) {
        (self.fns.render)(self.link(c),r)
    }

    pub fn event(&mut self, c: &mut E::Ctx, e: E::Event) {
        (self.fns.event)(self.link(c),e)
    }

    fn link<'a>(&self, c: &'a mut E::Ctx) -> Link<'a,E> {
        Link{
            ctx: c,
            widget_id: self.own_id.clone()
        }
    }
}