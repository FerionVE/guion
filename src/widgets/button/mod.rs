pub mod imp;
pub mod o;

use crate::core::widget::Widget;
use crate::core::ctx::Context;
use crate::core::widget::link::Link;
pub use imp::*;
pub use o::*;

pub trait IButton<E>: Widget<E> where E: Context {
    fn id(&self) -> E::WidgetID;

    fn action(&self) -> fn(Link<E>);
    fn caption(&self) -> &str;

    fn style(&self) -> E::Style;

    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}