use std::ops::DerefMut;
use std::ops::Deref;
use std::any::Any;
use crate::widget::env::Env;
use crate::widget::env::Context;
use crate::widget::env::WidgetStore;

pub mod env;
pub mod wref;

pub trait Widget<E>: Any where E: Env {

    ///fn render(&self, CE:)
    
    fn _render(&self) -> fn(Link<E>, E::Renderer);
    fn _event(&self) -> fn(Link<E>, E::Event);

    ///commit accessors may moved to Handler
    fn commit(&self) -> &E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn parent_mut(&mut self) -> &mut Option<E::WidgetID>;

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)> + 'a>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Link<'a,E> where E: Env {
    pub ctx: &'a mut E::Ctx,
    pub widget_id: E::WidgetID,
}

impl<'a,E> Link<'a,E> where E: Env {
    pub fn me<S: Widget<E> + 'static>(&'a self) -> &'a S {
        self.ctx.widgets().get(&self.widget_id)
            .expect("Link: Widget Gone")
            .as_any()
            .downcast_ref::<S>().expect("Link: Wrong Widget Type")
    }

    pub fn me_mut<S: Widget<E> + 'static>(&'a mut self) -> &'a mut S {
        self.ctx.widgets_mut().get_mut(&self.widget_id)
            .expect("Link: Widget Gone")
            .as_any_mut()
            .downcast_mut::<S>().expect("Link: Wrong Widget Type")
    }
}

impl<'a,E> Deref for Link<'a,E> where E: Env {
    type Target = E::Ctx;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,E> DerefMut for Link<'a,E> where E: Env {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}