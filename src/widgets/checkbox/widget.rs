use crate::style::standard::cursor::StdCursor;

use super::*;
use imp::ICheckBox;

impl<'w,E,State,Text,TrMut> Widget<E> for CheckBox<'w,E,State,Text,TrMut> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<'a,E>,
    State: AtomState<E,bool>,
    Text: AsWidget<E>,
    TrMut: TriggerMut<E>,
{
    fn child_paths(&self, _: E::WidgetPath, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        if l.state().is_hovered(&self.id) {
            r.set_cursor_specific(&StdCursor::Hand.into(),l.ctx);
        }
        let size = r.bounds().size.h;
        {
            let rect = Bounds::from_wh(size,size);
            let mut r = r.slice(&rect);
            r.with(&[
                    StdSelectag::ObjForeground,
                ][..])
                .fill_rect(l.ctx);
            r.inside_border_by_mul(StdSelectag::BorderVisual,3,l.ctx)
                .with(&[
                    StdSelectag::ObjForeground,
                    StdSelectag::Hovered(l.is_hovered()),
                    StdSelectag::Focused(l.is_focused()),
                    StdSelectag::Locked(self.locked),
                    StdSelectag::Pressed(self.state.get(l.ctx))
                ][..])
                .fill_rect(l.ctx);
            r.with(&[
                    StdSelectag::ObjBorder,
                    StdSelectag::Hovered(l.is_hovered()),
                    StdSelectag::Focused(l.is_focused()),
                    StdSelectag::Locked(self.locked),
                    StdSelectag::BorderVisual,
                    //StdSelectag::Pressed(self.state.get())
                ][..])
                .fill_border_inner(l.ctx);
        }
        {
            let text_border = Border::new(size+4/*TODO fix border impl*/*2,0,0,0);
            r.inside_border_specific(&text_border)
                .with(&[
                    StdSelectag::ObjForeground,
                    StdSelectag::ObjText,
                    StdSelectag::Hovered(l.is_hovered()),
                    StdSelectag::Focused(l.is_focused()),
                    StdSelectag::Locked(self.locked),
                ][..])
                .render_widget(l.for_child(0).unwrap());
        }
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));
        //let mut invalid = false;
        if e.event.is_hover_update() || e.event.is_kbd_down().is_some() || e.event.is_kbd_up().is_some() {
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.event.is_mouse_up() {
            if ee.key == MatchKeyCode::MouseLeft && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                let new = !self.state.get(l.ctx);
                self.set(l,new);
                return true;
            }
        } else if let Some(ee) = e.event.is_kbd_press() {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && ee.down_widget.is(self.id()) {
                let new = !self.state.get(l.ctx);
                self.set(l,new);
                return true;
            }
        }
        e.event.is_mouse_down().is_some()
    }
    fn _size(&self, mut l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
        let mut ms = l.for_child(0).unwrap().size(&e);
        ms.add_x( &self.size );
        ms
    }
    fn childs(&self) -> usize {
        1
    }
    fn childs_ref<'s>(&'s self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        vec![self.text.as_widget_dyn(root,ctx)]
    }
    fn into_childs<'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        vec![self.text.into_widget_dyn(root,ctx)]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        todo!();
        Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    }
    fn focusable(&self) -> bool { true }

    fn child<'s>(&'s self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.text.as_widget_dyn(root,ctx))
    }
    fn into_child<'s>(self: Box<Self>, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        if i != 0 {return Err(());}
        Ok(self.text.into_widget_dyn(root,ctx))
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn ICheckBox<E> => |s| s;
        dyn AtomState<E,bool> => |s| &s.state;
    );
}

impl<'l,E,State,Text,TrMut> AsWidget<E> for CheckBox<'l,E,State,Text,TrMut> where Self: Widget<E>, E: Env {
    type Widget = Self;
    type WidgetOwned = Self;

    #[inline]
    fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget<'w>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(*self)
    }
    #[inline]
    fn as_widget_dyn<'w,'s>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget_dyn<'w,'s>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    #[inline]
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}
