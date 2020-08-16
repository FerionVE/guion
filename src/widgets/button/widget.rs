use super::*;
use util::caption::CaptionMut;

impl<'w,E,Text,Stil> Widget<'w,E> for Button<'w,E,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E>,
    Text: AsWidget<'w,E>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        r.with(&[
            StdTag::ObjForeground,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some()),
        ][..])
            .fill_rect(l.ctx);
        r.with(&[
            StdTag::ObjBorder,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some()),
            StdTag::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        let mut r = r.inside_border_by(StdTag::BorderVisual,l.ctx);
        r.with(&[
            StdTag::ObjForeground,
            StdTag::ObjText,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some()),
        ][..])
            .render_widget(l.for_child(0).unwrap());
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderOuter));
        //e.0._debug_type_name();
        //let mut invalid = false;
        if e.0.is_hover_update() || e.0.is_kbd_press().is_some() || e.0.is_kbd_up().is_some() { //TODO catch down and press
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.0.is_mouse_up() {
            if ee.key == EEKey::<E>::MOUSE_LEFT && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                (self.trigger)(l);
                return true;
            }
        } else if let Some(ee) = e.0.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.is(self.id()) {
                (self.trigger)(l);
                return true;
            }
        }
        e.0.is_mouse_down().is_some()
    }
    fn _size(&self, mut l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        let mut ms = l.for_child(0).unwrap().size(e).as_std();
        ms += l.style_provider().border(&e.with(StdTag::BorderOuter));
        ms += l.style_provider().border(&e.with(StdTag::BorderVisual));
        ms.max( &self.size.as_std() ).into()
    }
    fn childs(&self) -> usize {
        1
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        vec![self.text.as_ref()]
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        vec![self.text.into_ref()]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &ESVariant<E>, _: bool) -> Result<Vec<Bounds>,()> {
        todo!();
        Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    }
    fn focusable(&self) -> bool { true }

    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        if i != 0 {return Err(());}
        Ok(self.text.as_ref())
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.text.into_ref())
    }
}

impl<'w,E,Text,Stil> WidgetMut<'w,E> for Button<'w,E,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E>,
    Text: AsWidgetMut<'w,E>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![self.text.as_mut()]
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![self.text.into_mut()]
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        if i != 0 {return Err(());}
        Ok(self.text.as_mut())
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.text.into_mut())
    }
}

impl<'w,E,S,Stil> Button<'w,E,S,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>>,
    E::Context: CtxStdState<E>,
    S: AsWidget<'w,E>+StatizeSized<E>
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
