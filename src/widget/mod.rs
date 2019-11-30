use std::any::Any;
use crate::widget::handler::WidgetHandler;
use crate::widget::env::Env;

pub mod handler;
pub mod env;

pub trait Widget<E>: Any where E: Env {
    fn handler(&self) -> Box<dyn WidgetHandler<E>>;

    ///commit accessors may moved to Handler
    fn commit(&self) -> &E::Commit;
    fn commit_mut(&mut self) -> &mut E::Commit;

    fn parent(&self) -> Option<&E::WidgetID>;
    fn parent_mut(&mut self) -> &mut Option<E::WidgetID>;

    fn childs<'a>(&'a self) -> Box<dyn Iterator<Item=((u32,u32,u32,u32),E::WidgetID)> + 'a>;

    fn _as_any(&self) -> &dyn Any;
    fn _as_any_mut(&mut self) -> &mut dyn Any;
}

fn e<E: Env>(a: &dyn Widget<E>) {
    let h = a.handler();
}