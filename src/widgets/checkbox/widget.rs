use crate::style::standard::cursor::StdCursor;

use super::*;
use util::state::AtomStateMut;
use imp::ICheckBox;
use super::imp::ICheckBoxMut;

impl<'w,E,State,Text> Widget<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E>,
    State: AtomState<E,bool>,
    Text: AsWidget<E>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
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
                (self.trigger)(l.reference(),new);
                Self::set(l,new);
                return true;
            }
        } else if let Some(ee) = e.event.is_kbd_press() {
            if (ee.key == MatchKeyCode::KbdReturn || ee.key == MatchKeyCode::KbdSpace) && ee.down_widget.is(self.id()) {
                let new = !self.state.get(l.ctx);
                (self.trigger)(l.reference(),new);
                Self::set(l,new);
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
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        vec![self.text.as_ref()]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![self.text.into_ref()]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        todo!();
        Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    }
    fn focusable(&self) -> bool { true }

    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        if i != 0 {return Err(());}
        Ok(self.text.as_ref())
    }
    fn into_child<'a>(self: Box<Self>, i: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        if i != 0 {return Err(());}
        Ok(self.text.into_ref())
    }

    impl_traitcast!(
        dyn ICheckBox<E> => |s| s;
        dyn AtomState<E,bool> => |s| &s.state;
    );
}

impl<'w,E,State,Text> WidgetMut<E> for CheckBox<'w,E,State,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E>,
    State: AtomStateMut<E,bool>,
    Text: AsWidgetMut<E>,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        vec![self.text.as_mut()]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![self.text.into_mut()]
    }
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        if i != 0 {return Err(());}
        Ok(self.text.as_mut())
    }
    fn into_child_mut<'a>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        if i != 0 {return Err(());}
        Ok(self.text.into_mut())
    }

    impl_traitcast_mut!(
        dyn ICheckBox<E> => |s| s;
        dyn ICheckBoxMut<E> => |s| s;
        dyn AtomState<E,bool> => |s| &mut s.state;
        dyn AtomStateMut<E,bool> => |s| &mut s.state;
    );
}

impl<'w,E,State,Text> CheckBox<'w,E,State,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E>,
    State: AtomState<E,bool>+'w,
    Text: AsWidget<E>,
{
    pub fn set(mut l: Link<E>, v: bool) {
        l.mutate_closure(Box::new(move |mut w,c,_|{
            //w.traitcast_mut::<dyn AtomStateMut<E,bool>>().unwrap().set(v,c);
            let w = w.traitcast_mut::<dyn ICheckBoxMut<E>>().unwrap();
            w.state_mut().set(v,c);
        }));
    }
}
