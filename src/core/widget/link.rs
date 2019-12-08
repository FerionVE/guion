use crate::core::ctx::id::WidgetID;
use crate::core::widget::Widget;
use std::ops::DerefMut;
use std::ops::Deref;
use crate::core::ctx::Context;

pub struct Link<'a,E> where E: Context {
    pub ctx: &'a mut E,
    pub widget_id: E::WidgetID,
}

impl<'a,E> Link<'a,E> where E: Context {
    #[inline]
    pub fn me<S: Widget<E> + 'static>(&'a self) -> &'a S {
        self.ctx.widget(&self.widget_id)
            .expect("Link: Widget Gone")
            .as_any()
            .downcast_ref::<S>().expect("Link: Wrong Widget Type")
    }
    #[inline] 
    pub fn me_mut<S: Widget<E> + 'static>(&'a mut self) -> &'a mut S {
        self.ctx.widget_mut(&self.widget_id)
            .expect("Link: Widget Gone")
            .as_any_mut()
            .downcast_mut::<S>().expect("Link: Wrong Widget Type")
    }

    pub fn with_ctx<'b,F: Context<WidgetID=E::WidgetID>>(self, ctx: &'b mut F) -> Link<'b,F> where E::WidgetID: WidgetID<F> {
        Link{
            ctx,
            widget_id: self.widget_id,
        }
    }
}

impl<'a,E> Deref for Link<'a,E> where E: Context {
    type Target = E;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,E> DerefMut for Link<'a,E> where E: Context {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

