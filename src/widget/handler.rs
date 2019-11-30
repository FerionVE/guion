use std::any::Any;
use crate::widget::env::*;

pub trait WidgetHandler<E>/*: Any*/ where E: Env {
    fn me<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a E::DynWidget> {
        c.widgets().get(me)
    }
    fn me_mut<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut E::DynWidget> {
        c.widgets_mut().get_mut(me)
    }

    fn render(&self, c: &mut E::Ctx, me: &E::WidgetID, r: E::Renderer);
    fn event(&self, c: &mut E::Ctx, me: &E::WidgetID, r: E::Event);
}