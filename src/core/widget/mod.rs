use crate::core::widget::handler::HandlerFns;
use crate::core::widget::handler::Handler;
use std::any::Any;
use crate::core::ctx::Context;

pub mod link;
pub mod handler;
pub mod imp;

pub trait Widget<'a,E>: Any where E: Context {
    fn id(&'a self) -> E::WidgetID;
    #[inline]
    fn handler(&'a self) -> Handler<E> {
        Handler {
            id: self.id(),
            fns: self._handler(),
        }
    }
    
    fn _handler(&'a self) -> HandlerFns<E>;

    ///commit accessors may moved to Handler
    fn invalid(&'a self) -> bool;
    fn set_invalid(&'a mut self, v: bool);

    fn parent(&'a self) -> Option<E::WidgetID>;
    fn set_parent(&'a mut self, v: Option<E::WidgetID>);

    fn childs(&'a self) -> Box<dyn Iterator<Item=E::WidgetID> + 'a>;

    fn childs_vec(&'a self) -> Vec<E::WidgetID>;

    fn as_any(&'a self) -> &dyn Any;
    fn as_any_mut(&'a mut self) -> &mut dyn Any;

    fn selectable(&'a self) -> bool;

    fn has_childs(&'a self) -> bool;

    fn style(&'a self) -> &'a E::Style;
}