use crate::core::widget::Widget;
use std::ops::DerefMut;
use std::ops::Deref;
use crate::core::env::Env;
use crate::core::env::Context;
use crate::core::env::WidgetStore;

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

