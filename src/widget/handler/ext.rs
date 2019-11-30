use crate::widget::handler::WidgetHandler;
use std::any::Any;
use crate::widget::Widget;
use crate::widget::env::*;
use crate::util::as_any::AsAny;

pub trait WidgetHandlerExt<E>: Any where E: Env {
    fn me<'a,S: Widget<E> + 'static>(c: &'a E::Ctx, me: &E::WidgetID) -> Option<&'a S>;
    fn me_mut<'a,S: Widget<E> + 'static>(c: &'a mut E::Ctx, me: &E::WidgetID) -> Option<&'a mut S>;
}

impl<E,T> WidgetHandlerExt<E> for T where T: WidgetHandler<E>, E: Env + 'static {
    
}