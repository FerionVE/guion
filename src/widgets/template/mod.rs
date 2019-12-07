pub mod imp;
pub mod o;
pub mod as_template;

use crate::core::widget::Widget;
use crate::core::ctx::Context;
use crate::core::widget::link::Link;
pub use imp::*;
pub use o::*;
pub use as_template::*;

pub trait ITemplate<E>: Widget<E> where E: Context {
    fn id(&self) -> E::WidgetID;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}