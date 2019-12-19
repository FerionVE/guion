pub mod imp;
pub mod o;

use render::widgets::RenderStdWidgets;
use ctx::aliases::*;
use state::handler::*;
use widget::handlez::fns::WidgetFns;
use ctx::aliases::EKey;
use crate::core::*;
use widget::Widget;
use ctx::*;
use widget::link::Link;
use event::key::Key;
pub use imp::*;
pub use o::*;

pub trait IButton<E>: Widget<E> + Sized where E: Env, ECHLink<E>: AsHandlerStateful<E,E::Context>, E::Renderer: RenderStdWidgets<E> {
    fn id(&self) -> E::WidgetID;
    
    #[inline]
    fn _fns(&self) -> WidgetFns<E> {
        WidgetFns{
            render: imp::_render::<Self,E>,
            event: imp::_event::<Self,E>,
            size: imp::_size::<Self,E>,
        }
    }
    
    fn action(&self) -> fn(Link<E>);
    fn caption(&self) -> &str;
    
    fn style(&self) -> &E::Style;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);
    
    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
    #[inline]
    fn kbd_trigger(&self) -> EKey<E> {
        EKey::<E>::ENTER
    }
}