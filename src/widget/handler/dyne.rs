use crate::widget::handler::WidgetHandler;
use crate::widget::env::Env;
use std::any::TypeId;

#[derive(Clone)]
pub struct DynHandler<E> where E: Env + 'static {
    t: TypeId,
    render: fn(&mut E::Ctx, &E::WidgetID, E::Renderer),
    event: fn(&mut E::Ctx, &E::WidgetID, E::Event),
}

impl<E> DynHandler<E> where E: Env {
    pub fn render(&self, c: &mut E::Ctx, me: &E::WidgetID, r: E::Renderer) {
        (self.render)(c,me,r)
    }
    pub fn event(&self, c: &mut E::Ctx, me: &E::WidgetID, e: E::Event) {
        (self.event)(c,me,e)
    }

    pub fn is<T: WidgetHandler<E>>(&self) -> bool {
        self.t == TypeId::of::<T>()
    }

    pub fn of<T: WidgetHandler<E>>() -> Self {
        Self{
            t: TypeId::of::<T>(),
            render: T::render,
            event: T::event,
        }
    }
}