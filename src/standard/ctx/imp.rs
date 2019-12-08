use crate::core::ctx::id::WidgetID;
use crate::core::ctx::ctx_meta::ContextMeta;
use crate::core::widget::Widget;
use crate::core::render::Render;
use crate::core::ctx::Context;
use crate::core::lazout::size::Size;
use super::*;
///NOTE that E is not the current Context but the underlying
impl<E> Context for StandardCtx<E> where E: Context, E::Meta: ContextMeta<Self>, E::Renderer: Render<Self>, E::DynWidget: Widget<Self>, E::WidgetID: WidgetID<Self> {
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
    fn _render(&mut self, i: &Self::WidgetID, r: Self::Renderer) {
        unimplemented!();
        //set the cursor from widget's style
        self.sup._render(i,r);
        //draw selected if current widget is in SelectedState
    }
    
    #[inline]
    fn _event(&mut self, i: &Self::WidgetID, e: Self::Event) {
        unimplemented!();
        //Wkeydown: add to respective keystate
        //Wkeyup: remove from respective keystate
        //Wmousemove: after sending mousemove, send mouseenter/leave if HoverState changed
        //mousemove: set HoverState to Some(current)
        //Wmouseleave: set HoverState to None
        self.sup._event(i,e);
    }
    
    #[inline]
    fn _size(&mut self, i: &Self::WidgetID) -> Size {
        unimplemented!();
        self.sup._size(i)
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