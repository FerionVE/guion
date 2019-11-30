use crate::widget::Widget;
use std::any::Any;
use crate::widget::env::*;
use crate::util::as_any::AsAny;

pub trait WidgetHandler<E>: Any where E: Env {
    /*fn me<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a E::DynWidget> {
        c.widgets().get(me)
    }
    fn me_mut<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut E::DynWidget> {
        c.widgets_mut().get_mut(me)
    }*/

    fn render(&self, c: &mut E::Ctx, me: &E::WidgetID, r: E::Renderer);
    fn event(&self, c: &mut E::Ctx, me: &E::WidgetID, e: E::Event);
}

pub trait WidgetHandlerExt<E>: Any where E: Env {
    fn me<'a,S: Widget<E> + 'static>(&self, c: &'a E::Ctx, me: &E::WidgetID) -> Option<&'a S>;
    fn me_mut<'a,S: Widget<E> + 'static>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut S>;
}

impl<E,T> WidgetHandlerExt<E> for T where T: WidgetHandler<E>, E: Env + 'static {
    fn me<'a,S: Widget<E> + 'static>(&self, c: &'a E::Ctx, me: &E::WidgetID) -> Option<&'a S> {
        c.widgets().get(me)
        .map(|d|
            d.as_any().downcast_ref::<S>().expect("Invalid Widget Downcast Type")
        )
    }

    fn me_mut<'a,S: Widget<E> + 'static>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut S> {
        c.widgets_mut().get_mut(me)
        .map(|d|
            d.as_any_mut().downcast_mut::<S>().expect("Invalid Widget Downcast Type")
        )
    }
}

impl<E,T> WidgetHandler<E> for Box<T> where T: WidgetHandler<E>, E: Env {
    fn render(&self, c: &mut E::Ctx, me: &E::WidgetID, r: E::Renderer) {
        (**self).render(c,me,r)
    }
    fn event(&self, c: &mut E::Ctx, me: &E::WidgetID, e: E::Event) {
        (**self).event(c,me,e)
    }
}