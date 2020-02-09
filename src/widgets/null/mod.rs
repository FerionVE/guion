use crate::core::ctx::aliases::*;
use crate::core::*;
use widget::Widget;
use ctx::*;
use widget::link::Link;

pub mod imp;
pub mod o;
pub mod as_null;

#[doc(inline)]
pub use imp::*;
#[doc(inline)]
pub use o::*;

/*pub mod as_null {
    crate::create_widget_as_widget_module!(INull,AsNull);
}*/

pub use as_null::*;

/// implement a view as Null over a type
/// 
/// Then put the reference or owned type inside a AsNull to use as widget
/// If your type should only be viewed as one widget, you can use impl_null! to implement Widget directly
pub trait INull<E>: Widget<E> where E: Env {
    fn id(&self) -> E::WidgetID;

    fn style(&self, s: &mut EStyle<E>);
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);

    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
}