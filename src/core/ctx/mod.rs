use crate::core::widget::handler::fns::WidgetFns;
use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub mod ctx_meta;
pub use ctx_meta::*;

pub mod id;
pub use id::*;

pub mod aliases;

pub trait Context: Sized + 'static {
    type Meta: ContextMeta<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: WidgetID<Self>;
    type Commit: Eq + Ord;
    type Style: Style;

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
        (self.widget_fns(i).render)(self.link(i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(&mut self, i: &Self::WidgetID, e: Self::Event) {
        (self.widget_fns(i).event)(self.link(i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(&mut self, i: &Self::WidgetID) -> Size {
        (self.widget_fns(i).size)(self.link(i))
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
}

pub trait ContextStateful: Context {
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

