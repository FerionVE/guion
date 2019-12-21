use crate::core::*;
use widget::Widget;
use ctx::*;
use widget::link::Link;

pub mod imp;
pub mod o;
#[doc(inline)]
pub use imp::*;
#[doc(inline)]
pub use o::*;

pub trait IEmpty<E>: Widget<E> where E: Env {
    fn id(&self) -> E::WidgetID;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}