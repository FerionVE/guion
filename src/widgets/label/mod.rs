use crate::core::*;
use widget::Widget;
use ctx::*;
use widget::link::Link;

pub mod imp;
pub mod o;
pub mod as_label;

#[doc(inline)]
pub use imp::*;
#[doc(inline)]
pub use o::*;

/*pub mod as_label {
    crate::create_widget_as_widget_module!(ILabel,AsLabel);
}*/

//pub use as_label::*;

/// implement a view as Label over a type
/// 
/// Then put the reference or owned type inside a AsLabel to use as widget
/// If your type should only be viewed as one widget, you can use impl_label! to implement Widget directly
pub trait ILabel<E>: Widget<E> where E: Env {
    fn id(&self) -> E::WidgetID;

    fn style(&self) -> &E::Style;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}