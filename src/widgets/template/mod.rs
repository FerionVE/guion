pub mod imp;
pub mod o;
pub mod as_template;

use crate::core::widget::Widget;
use crate::core::ctx::*;
use crate::core::widget::link::Link;
pub use imp::*;
pub use o::*;

/*pub mod as_template {
    crate::create_widget_as_widget_module!(ITemplate,AsTemplate);
}*/

pub use as_template::*;

/// implement a view as Template over a type
/// 
/// Then put the reference or owned type inside a AsTemplate to use as widget
/// If your type should only be viewed as one widget, you can use impl_template! to implement Widget directly
pub trait ITemplate<E>: Widget<E> where E: Env {
    fn id(&self) -> E::WidgetID;

    fn style(&self) -> &E::Style;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}