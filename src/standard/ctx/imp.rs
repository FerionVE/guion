use crate::core::ctx::ctx_meta::ContextMeta;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::Context;
use crate::core::lazout::size::Size;
use crate::core::widget::link::Link;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<E> Context for StandardCtx<E> where E: Context, E::Meta: ContextMeta<Self>, E::Renderer: Render<Self>, E::DynWidget: Widget<Self> {
    type Meta = E::Meta;
    type Renderer = E::Renderer;
    type Event = E::Event;
    ///regularly just dyn Widget
    type DynWidget = E::DynWidget;
    type WidgetID = E::WidgetID;
    type Commit = E::Commit;
    type Style = E::Style;

    #[inline]
    fn widget(&self, i: &Self::WidgetID) -> Option<&Self::DynWidget> {
        self.sup.widget(i)
    }
    #[inline]
    fn widget_mut(&mut self, i: &Self::WidgetID) -> Option<&mut Self::DynWidget> {
        self.sup.widget_mut(i)
    }
    #[inline]
    fn tune_id(&self, i: &mut Self::WidgetID) {
        self.sup.tune_id(i)
    }
    #[inline]
    fn tune_id_mut(&mut self, i: &mut Self::WidgetID) {
        self.sup.tune_id_mut(i)
    }

    #[inline]
    fn pre_render(&mut self, i: &Self::WidgetID, e: &mut Self::Renderer) {
        unimplemented!();
    }
    #[inline]
    fn post_render(&mut self, i: &Self::WidgetID, e: &mut Self::Renderer) {
        unimplemented!();
    }
    
    #[inline]
    fn pre_event(&mut self, i: &Self::WidgetID, e: Self::Event) -> Self::Event {
        unimplemented!();
        e
    }
    #[inline]
    fn post_event(&mut self, i: &Self::WidgetID, e: Self::Event) {
        unimplemented!();
    }
    
    #[inline]
    fn pre_size(&mut self, i: &Self::WidgetID) {
        unimplemented!();
    }
    #[inline]
    fn post_size(&mut self, i: &Self::WidgetID, s: Size) -> Size {
        unimplemented!();
        s
    }

    #[inline]
    fn link<'a>(&'a mut self, i: Self::WidgetID) -> Link<'a,Self> {
        Link{
            ctx: self,
            widget_id: i
        }
    }

    #[inline]
    fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline]
    fn selected(&self) -> Option<E::WidgetID> {
        None
    }
}