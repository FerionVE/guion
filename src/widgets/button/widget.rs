use super::*;

impl<'w,E,Text> Widget<E> for Button<'w,E,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E>,
    Text: AsWidget<E>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdSelector::BorderOuter,l.ctx);
        if l.state().is_hovered(&self.id) {
            r.set_cursor(l.ctx,StdSelector::CursorHand StdCursor);
        }
        r.with(&[
            StdSelector::ObjForeground,
            StdSelector::Hovered(l.is_hovered()),
            StdSelector::Focused(l.is_focused()),
            StdSelector::Locked(self.locked),
            StdSelector::Pressed(Self::pressed(&l).is_some()),
        ][..])
            .fill_rect(l.ctx);
        r.with(&[
            StdSelector::ObjBorder,
            StdSelector::Hovered(l.is_hovered()),
            StdSelector::Focused(l.is_focused()),
            StdSelector::Locked(self.locked),
            StdSelector::Pressed(Self::pressed(&l).is_some()),
            StdSelector::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        let mut r = r.inside_border_by(StdSelector::BorderVisual,l.ctx);
        r.with(&[
            StdSelector::ObjForeground,
            StdSelector::ObjText,
            StdSelector::Hovered(l.is_hovered()),
            StdSelector::Focused(l.is_focused()),
            StdSelector::Locked(self.locked),
            StdSelector::Pressed(Self::pressed(&l).is_some()),
        ][..])
            .render_widget(l.for_child(0).unwrap());
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(&self.style,StdSelector::BorderOuter));
        //e.0._debug_type_name();
        //let mut invalid = false;
        if e.event.is_hover_update() || e.event.is_kbd_press().is_some() || e.event.is_kbd_up().is_some() { //TODO catch down and press
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.event.is_mouse_up() {
            if ee.key == EEKey::<E>::MOUSE_LEFT && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                (self.trigger)(l);
                return true;
            }
        } else if let Some(ee) = e.event.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.is(self.id()) {
                (self.trigger)(l);
                return true;
            }
        }
        e.event.is_mouse_down().is_some()
    }
    fn _size(&self, mut l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let mut ms = l.for_child(0).unwrap().size(e);
        ms.add_border( &l.style_provider().border(&e.with(StdSelector::BorderOuter)) );
        ms.add_border( &l.style_provider().border(&e.with(StdSelector::BorderVisual)) );
        ms.max( &self.size )
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
}

impl<'w,E,Text> WidgetMut<E> for Button<'w,E,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E>,
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
}

impl<'w,E,S> Button<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E>,
    S: AsWidget<E>,
{
    pub fn pressed<'l:'s,'s>(l: &'s Link<'l,E>) -> Option<&'s EPressedKey<E>> {
        let id = l.id();
        l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],id.clone())
            .or_else(||
                l.state().is_pressed_and_id(&[EEKey::<E>::ENTER],id.clone())
            )
            .or_else(||
                l.state().is_pressed_and_id(&[EEKey::<E>::SPACE],id)
            )
    }
}
