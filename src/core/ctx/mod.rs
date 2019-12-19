use std::borrow::BorrowMut;
use crate::core::widget::handlez::fns::WidgetFns;
use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;
use std::any::Any;

//pub mod ctx_meta; TODO fix CtxMeta
//pub use ctx_meta::*;

pub mod id;
pub use id::*;

pub mod aliases;
use aliases::*;

pub mod widgets;
pub use widgets::*;

pub mod queue;
pub use queue::*;

pub mod handler;
pub use handler::*;

pub mod stateful;
pub use stateful::*;

pub trait Env: Sized + 'static {
    type Context: Context + Widgets<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type Commit: Eq + Ord;
    type Style: Style;
}

pub trait Context: Sized + 'static {
    type Link: AsHandler<Self::Handler,Self>;
    type Handler: Handler<Self>;
    //type Meta: ContextMeta;

    #[inline] 
    fn handler<H: Handler<Self>>(&mut self) -> &mut H where Self::Link: AsHandler<H,Self> {
        Self::Link::as_mut(self)
    }

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, r: E::Renderer) where Self: Widgets<E> {
        Self::Handler::_render::<E>(self,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, e: E::Event) where Self: Widgets<E> {
        Self::Handler::_event::<E>(self,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env<Context=Self>>(&mut self, i: &E::WidgetID) -> Size where Self: Widgets<E> {
        Self::Handler::_size::<E>(self,i)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns<E: Env<Context=Self>>(&self, i: &E::WidgetID) -> WidgetFns<E> where Self: Widgets<E> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a,E: Env<Context=Self>>(&'a mut self, i: &E::WidgetID) -> Link<'a,E> where Self: Widgets<E> {
        Link{
            ctx: self,
            widget_id: i.clone(),
        }
    }

    #[inline] fn state<E: Env<Context=Self>>(&self) -> &<Self::Link as AsHandlerStateful<E,Self>>::T where Self: Widgets<E>, Self::Link: AsHandlerStateful<E,Self> + AsHandler<ECStateful<E>,E::Context> {
        Self::Link::stateful(self)
    }
}