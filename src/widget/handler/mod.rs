use crate::widget::Widget;
use std::any::Any;
use crate::widget::env::*;
use crate::util::as_any::AsAny;

//pub mod ext;
pub mod dyne;

pub trait WidgetHandler<E>: Any where E: Env {
    /*fn me<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a E::DynWidget> {
        c.widgets().get(me)
    }
    fn me_mut<'a>(&self, c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut E::DynWidget> {
        c.widgets_mut().get_mut(me)
    }*/

    fn render(c: &mut E::Ctx, me: &E::WidgetID, r: E::Renderer);
    fn event(c: &mut E::Ctx, me: &E::WidgetID, e: E::Event);
}

