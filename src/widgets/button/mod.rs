pub mod imp;
pub mod o;

pub use imp::*;
pub use o::*;

pub trait IButton<E> where E: Env {
    fn id(&self) -> E::WidgetID;

    fn press(&self) -> fn(Link<E>);

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<&E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}