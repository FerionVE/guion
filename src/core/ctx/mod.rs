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

pub mod queue;
pub use queue::*;

mod imp;

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

pub trait Widgets<E>: 'static where E: Env {
    fn widget(&self, i: &E::WidgetID) -> Option<&E::DynWidget>;
    fn widget_mut(&mut self, i: &E::WidgetID) -> Option<&mut E::DynWidget>;

    #[inline]
    fn has_widget(&self, i: &E::WidgetID) -> bool {
        self.widget(i).is_some()
    }

    #[inline] fn tune_id(&self, _i: &mut E::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut E::WidgetID) {}
}

pub trait Context: Sized + 'static {
    type Handler: Handler;
    //type Meta: ContextMeta;

    fn handler_mut(&mut self) -> &mut Self::Handler;

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

    #[inline]
    fn get_handler<L: Handler>(&mut self) -> Option<&mut L> {
        self.handler_mut()._ref_of()
    }
}

pub trait Handler: Sized + 'static {
    //
    type Child: Handler + 'static;
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: &mut E::Context, i: &E::WidgetID, r: E::Renderer) {
        Self::Child::_render::<E>(senf,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: &mut E::Context, i: &E::WidgetID, e: E::Event) {
        Self::Child::_event::<E>(senf,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: &mut E::Context, i: &E::WidgetID) -> Size {
        Self::Child::_size::<E>(senf,i)
    }

    #[deprecated = "Should not be called as it will panic on the impl of ()"]
    fn _child_mut(&mut self) -> &mut Self::Child;
    #[allow(deprecated)]
    #[inline]
    fn child_mut(&mut self) -> Option<&mut Self::Child> {
        Some(self._child_mut())
    }

    #[deprecated = "Should not be called as it will panic on the impl of ()"]
    fn _child(&self) -> &Self::Child;
    #[allow(deprecated)]
    #[inline]
    fn child(&self) -> Option<&Self::Child> {
        Some(self._child())
    }

    #[inline]
    fn _ref_of<L: Handler>(&mut self) -> Option<&mut L> {
        if Any::is::<L>(self) {
            Any::downcast_mut::<L>(self)
        }else{
            self.child_mut().and_then(<Self::Child as Handler>::_ref_of)
        }
    }

    #[inline]
    fn get_self<C: Context>(senf: &mut C) -> &mut Self {
        senf.get_handler().expect("This Handler must be or be a child of E::Context::Handler")
    }
}

pub trait HandlerWithChild: Handler + Sized + 'static {
    #[allow(deprecated)]
    #[inline]
    fn hwc_child_mut(&mut self) -> &mut Self::Child {
        self._child_mut()
    }
    #[allow(deprecated)]
    #[inline]
    fn hwc_child(&self) -> &Self::Child {
        self._child()
    }
}
//TODO remove this again
pub trait AsHandler<C> where C: Handler {
    fn handler_mut(&mut self) -> &mut C;
}

pub trait HandlerStateful<E>: 'static where E: Env {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}

pub trait ContextStateful<E>: Context where Self::Handler: HandlerStateful<E>, E: Env {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }
}