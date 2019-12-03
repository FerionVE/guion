use crate::core::lazout::Size;
use crate::core::util::bounded_widget::BoundedWidget;
use crate::core::widget::handler::HandlerFns;
use crate::core::widget::handler::Handler;
use std::any::Any;
use crate::core::env::Env;

pub mod link;
pub mod handler;

pub trait Widget<E>: Any where E: Env {
    fn id(&self) -> E::WidgetID;

    fn handler<'a>(&self) -> Handler<E> {
        Handler {
            id: self.id(),
            fns: self._handler(),
        }
    }
    
    fn _handler(&self) -> HandlerFns<E>;

    ///commit accessors may moved to Handler
    fn render_invalid(&self) -> bool;
    fn set_render_invalid(&mut self, v: bool);

    fn layout_invalid(&self) -> bool;
    fn set_layout_invalid(&mut self, v: bool);

    fn size(&self) -> Size;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=BoundedWidget<E>> + 'a>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}