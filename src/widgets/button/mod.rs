pub mod imp;
pub mod o;

use crate::core::event::VariantSupport;
use crate::core::event::variants::KbdDown;
use render::widgets::RenderStdWidgets;
use ctx::aliases::*;
use state::handler::*;
use widget::fns::WidgetFns;
use crate::core::*;
use widget::Widget;
use ctx::*;
use widget::link::Link;
use event::key::Key;

#[doc(inline)]
pub use imp::*;
#[doc(inline)]
pub use o::*;

pub trait IButton<E>: Widget<E> + Sized where E: Env, ECHandler<E>: AsHandlerStateful<E>, ERenderer<E>: RenderStdWidgets<E>, EEvent<E>: VariantSupport<KbdDown<EEventKey<E>>,E> {
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
    
    fn style(&self) -> &EStyle<E>;
    
    fn invalid(&self) -> bool;
    fn set_invalid(&mut self, v: bool);
    
    fn parent(&self) -> Option<E::WidgetID>;
    fn set_parent(&mut self, v: Option<E::WidgetID>);
    #[inline]
    fn kbd_trigger(&self) -> EEventKey<E> {
        <EEventKey<E> as Key>::ENTER
    }
}