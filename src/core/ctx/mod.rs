use crate::core::style::Style;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use crate::core::event::Event;
use crate::core::render::Render;
use crate::core::widget::Widget;

pub mod ctx_meta;
pub use ctx_meta::*;

pub trait Context: Sized + 'static {
    type Meta: ContextMeta<Self>;
    type Renderer: Render<Self>;
    type Event: Event;
    ///regularly just dyn Widget
    type DynWidget: Widget<Self> + ?Sized;
    type WidgetID: PartialEq + Clone;
    type Commit: Eq + Ord;
    type Style: Style;

    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget>;
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget>;

    #[inline] fn tune_id(&self, _i: &mut Self::WidgetID) {}
    #[inline] fn tune_id_mut(&mut self, _i: &mut Self::WidgetID) {}

    #[inline] fn pre_render(&mut self, _i: &Self::WidgetID, _e: &mut Self::Renderer) {}
    #[inline] fn post_render(&mut self, _i: &Self::WidgetID, _e: &mut Self::Renderer) {}
    
    #[inline] fn pre_event(&mut self, _i: &Self::WidgetID, e: Self::Event) -> Self::Event {e}
    #[inline] fn post_event(&mut self, _i: &Self::WidgetID, _e: Self::Event) {}
    
    #[inline] fn pre_size(&mut self, _i: &Self::WidgetID) {}
    #[inline] fn post_size(&mut self, _i: &Self::WidgetID, s: Size) -> Size {s}

    #[inline] fn link<'a>(&'a mut self, i: Self::WidgetID) -> Link<'a,Self> {
        Link{
            ctx: self,
            widget_id: i
        }
    }

    #[inline] fn hovered(&self) -> Option<Self::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<Self::WidgetID> {
        None
    }
}