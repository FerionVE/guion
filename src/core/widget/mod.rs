use crate::core::util::border::Border;
use crate::core::widget::handler::WidgetFns;
use crate::core::widget::handler::Handler;
use std::any::Any;
use crate::core::ctx::Context;
use crate::core::style::Style;

pub mod link;
pub mod handler;
//pub mod imp;

pub trait Widget<E>: Any where E: Context + 'static {
    fn id(&self) -> E::WidgetID;
    #[inline]
    fn handler<'a>(&self, c: &'a mut E) -> Handler<'a,E> { //TODO deprecate in future
        Handler {
            id: self.id(),
            ctx: c,
        }
    }
    
    fn _fns(&self) -> WidgetFns<E>;

    ///commit accessors may moved to Handler
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a>;

    fn childs_vec<'a>(&'a self) -> Vec<E::WidgetID>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn selectable(&self) -> bool;

    fn has_childs(&self) -> bool;
    #[inline]
    fn style(&self) -> &E::Style {
        E::Style::default()
    }
    #[inline]
    fn border(&self) -> &Border {
        E::Style::default_border()
    }
}