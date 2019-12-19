use crate::core::ctx::aliases::*;
use crate::core::ctx::*;
use crate::core::widget::Widget;
use std::ops::DerefMut;
use std::ops::Deref;
use crate::core::ctx::*;

pub struct Link<'a,E> where E: Env {
    pub ctx: &'a mut E::Context,
    pub widget_id: E::WidgetID,
}

impl<'a,E> Link<'a,E> where E: Env {
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

    pub fn with_ctx<'b,F: Env<WidgetID=E::WidgetID>>(self, ctx: &'b mut F::Context) -> Link<'b,F> {
        Link{
            ctx,
            widget_id: self.widget_id,
        }
    }
    #[inline]
    pub fn enqueue<Q: Queue<E>>(&'a mut self, args: Q::Args, f: Q::Callback) -> Q::Return where E::Context: AccessQueue<Q,E> {
        self.ctx.queue_mut().add(args,f)
    }

    #[inline]
    pub fn is_hovered(&self) -> bool where ECHLink<E>: AsHandlerStateful<E,E::Context> + AsHandler<ECStateful<E>,E::Context> {
        self.ctx.state().is_hovered(&self.widget_id)
    }
    #[inline]
    pub fn is_selected(&self) -> bool where ECHLink<E>: AsHandlerStateful<E,E::Context> + AsHandler<ECStateful<E>,E::Context> {
        self.ctx.state().is_selected(&self.widget_id)
    }
}

impl<'a,E> Deref for Link<'a,E> where E: Env {
    type Target = E::Context;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
impl<'a,E> DerefMut for Link<'a,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

