use std::borrow::BorrowMut;
use crate::core::widget::handler::fns::WidgetFns;
use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;
use std::any::Any;

pub mod ctx_meta;
pub use ctx_meta::*;

pub mod id;
pub use id::*;

pub mod aliases;

pub mod queue;
pub use queue::*;

mod imp;

pub trait Context: Sized + 'static {
    type Handler: ContextLayer<Self>;
    type Meta: ContextMeta<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID<Self>;
    type Commit: Eq + Ord;
    type Style: Style;

    fn handler_mut(&mut self) -> &mut Self::Handler;

    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget>;
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget>;

    #[inline]
    fn has_widget(&self, i: &Self::WidgetID) -> bool {
        self.widget(i).is_some()
    }

    #[inline] fn tune_id(&self, _i: &mut Self::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut Self::WidgetID) {}

    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(&mut self, i: &Self::WidgetID, r: Self::Renderer) {
        Self::Handler::_render(self,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(&mut self, i: &Self::WidgetID, e: Self::Event) {
        Self::Handler::_event(self,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(&mut self, i: &Self::WidgetID) -> Size {
        Self::Handler::_size(self,i)
    }
    /// PANICKS if widget doesn't exists
    #[inline]
    fn widget_fns(&self, i: &Self::WidgetID) -> WidgetFns<Self> {
        Widget::_fns(self.widget(i).expect("Lost Widget"))
    }

    #[inline] fn link<'a>(&'a mut self, i: &Self::WidgetID) -> Link<'a,Self> {
        Link{
            ctx: self,
            widget_id: i.clone(),
        }
    }

    #[inline]
    fn get_handler<L: ContextLayer<Self>>(&mut self) -> Option<&mut L> {
        self.handler_mut()._ref_of()
    }
}

pub trait ContextLayer<E>: Sized + 'static where E: Context {
    //
    type Child: ContextLayer<E> + Sized + 'static;
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(senf: &mut E, i: &E::WidgetID, r: E::Renderer) {
        Self::Child::_render(senf,i,r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(senf: &mut E, i: &E::WidgetID, e: E::Event) {
        Self::Child::_event(senf,i,e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(senf: &mut E, i: &E::WidgetID) -> Size {
        Self::Child::_size(senf,i)
    }

    #[deprecated = "Should not be called as it will panic on the impl of ()"]
    fn _child_mut(&mut self) -> &mut Self::Child;
    #[allow(deprecated)]
    #[inline]
    fn child_mut(&mut self) -> Option<&mut Self::Child> {
        Some(self._child_mut())
    }

    #[inline]
    fn _ref_of<L: ContextLayer<E>>(&mut self) -> Option<&mut L> {
        if Any::is::<L>(self) {
            Any::downcast_mut::<L>(self)
        }else{
            self.child_mut().and_then(<Self::Child as ContextLayer<E>>::_ref_of)
        }
    }

    #[inline]
    fn get_self(senf: &mut E) -> &mut Self {
        senf.get_handler().expect("ContextLayer<E> must be or be a child of E::Handler")
    }
}

pub trait ContextLayerStateful<E>: Sized where E: Context {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}

pub trait ContextStateful: Context where Self::Handler: ContextLayerStateful<Self> {
    #[inline] fn hovered(&self) -> Option<Self::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<Self::WidgetID> {
        None
    }

    #[inline]
    fn is_hovered(&self, i: &Self::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &Self::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }
}