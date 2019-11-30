use crate::widget::handler::HandlerFns;
use crate::widget::handler::Handler;
use std::any::Any;
use crate::widget::env::Env;


pub mod env;
pub mod link;
pub mod handler;

pub trait Widget<E>: Any where E: Env {
    fn id(&self) -> E::WidgetID;

    fn handler<'a>(&self) -> Handler<E> {
        Handler {
            own_id: self.id(),
            fns: self._handler(),
        }
    }
    
    fn _handler(&self) -> HandlerFns<E>;

    ///commit accessors may moved to Handler
    fn commit(&self) -> &E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn parent_mut(&mut self) -> &mut Option<E::WidgetID>;

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)> + 'a>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}