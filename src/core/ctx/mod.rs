use crate::core::util::bounds::Bounds;
use crate::core::event::key::Key;
use crate::core::event::Destination;
use widget::dyn_widget::DynWidget;
use std::any::Any;

use crate::core::*;
use widget::fns::WidgetFns;
use style::Style;
use lazout::size::Size;
use widget::link::Link;
use event::Event;
use render::Render;
use widget::Widget;
use state::handler::*;

//pub mod ctx_meta; TODO fix CtxMeta
//pub use ctx_meta::*;

pub mod id;
pub use id::*;

#[allow(type_alias_bounds)]
pub mod aliases;
use aliases::*;

pub mod widgets;
pub use widgets::*;

pub mod queue;
pub use queue::*;

pub mod handler;
pub use handler::*;

pub trait Env: Sized + 'static {
    type Context: Context + Widgets<Self>;
    type Renderer: Render<Self>;
    type Event: Event<Self>;
    ///regularly just dyn Widget
    type DynWidget: DynWidget<Self> + ?Sized;
    type WidgetID: WidgetID;
    type Commit: Eq + Ord;
    type EventDest: Destination;
    type EventKey: Key;
    type EventConsuming;
    type Style: Style<Self>;
}

pub trait Context: Sized + 'static {
    type Handler: Handler<Self>;
    //type Meta: ContextMeta;

    #[inline] 
    fn handler_mut<H: Handler<Self>>(&mut self) -> &mut H where Self::Handler: AsHandler<H,Self> {
        Self::Handler::as_mut(self)
    }
    #[inline] 
    fn handler<H: Handler<Self>>(&self) -> &H where Self::Handler: AsHandler<H,Self> {
        Self::Handler::as_ref(self)
    }

    fn _handler_mut(&mut self) -> &mut Self::Handler;
    fn _handler(&self) -> &Self::Handler;

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env<Context=Self>>(&mut self, i: &E::WidgetID, r: (&mut E::Renderer,&Bounds)) where Self: Widgets<E> {
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

    #[inline] fn link<'a,E: Env<Context=Self>>(&'a mut self, i: &E::WidgetID, b: Bounds) -> Link<'a,E> where Self: Widgets<E> {
        Link{
            ctx: self,
            id: i.clone(),
            bounds: b,
        }
    }

    #[inline] fn state<E: Env<Context=Self>>(&self) -> &ECStateful<E> where Self: Widgets<E>, Self::Handler: AsHandlerStateful<E> {
        Self::Handler::stateful(self)
    }
}