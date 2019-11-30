use crate::widget::handler::WidgetHandler;
use crate::widget::env::Env;

pub mod handler;
pub mod env;

pub trait Widget<E> where E: Env {
    type H: WidgetHandler<E>;

    fn handler(&self) -> Self::H;

    ///commit accessors may moved to Handler
    fn commit(&self) -> E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;

    fn parent(&self) -> Option<&E::WidgetID>;

    fn childs(&self) -> Box<dyn Iterator<Item=(u32,u32,E::WidgetID)>>;
}