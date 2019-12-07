use crate::core::widget::handler::HandlerFns;
use crate::core::widget::handler::Handler;
use std::any::Any;
use crate::core::ctx::Context;

pub mod link;
pub mod handler;
//pub mod imp;

pub trait Widget<E>: Any where E: Context {
    fn id(&self) -> E::WidgetID;
    #[inline]
    fn handler<'a>(&self) -> Handler<E> {
        Handler {
            id: self.id(),
            fns: self._handler(),
        }
    }
    
    fn _handler(&self) -> HandlerFns<E>;

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

    fn style(&self) -> E::Style;
}