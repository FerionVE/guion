use super::*;

impl<'w,E,Text> Widget<'w,E> for Button<'w,E,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    Text: Caption<'w>+Statize<E>, Text::Statur: Sized,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjButton]);
        s.attach(&self.style[..]);
    }
    fn border(&self, b: &mut Border) {
        if let Some(senf) = &self.border {
            *b = *senf;
        }
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        r.with(&[
            StdVerb::ObjForeground,
            StdVerb::Hovered(l.is_hovered()),
            StdVerb::Focused(l.is_focused()),
            StdVerb::Locked(self.locked),
            StdVerb::Pressed(Self::pressed(&l).is_some())
        ])
            .fill_rect();
        r.with(&[
            StdVerb::ObjBorder,
            StdVerb::Hovered(l.is_hovered()),
            StdVerb::Focused(l.is_focused()),
            StdVerb::Locked(self.locked),
            StdVerb::Pressed(Self::pressed(&l).is_some())
        ])
            .border_rect(2);
        r.with(&[
            StdVerb::ObjForeground,
            StdVerb::ObjText,
            StdVerb::Hovered(l.is_hovered()),
            StdVerb::Focused(l.is_focused()),
            StdVerb::Locked(self.locked),
            StdVerb::Pressed(Self::pressed(&l).is_some())
        ])
            .render_text(self.text.caption().as_ref(),l.ctx);
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //e.0._debug_type_name();
        //let mut invalid = false;
        if e.0.is_hover_update() || e.0.is_kbd_press().is_some() || e.0.is_kbd_up().is_some() { //TODO catch down and press
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.0.is_mouse_up() {
            if ee.key == EEKey::<E>::MOUSE_LEFT && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                (self.trigger)(l)
            }
        } else if let Some(ee) = e.0.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.is(self.id()) {
                (self.trigger)(l)
            }
        }
    }
    fn _size(&self, _: Link<E>) -> ESize<E> {
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        vec![]
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        vec![]
    }
    
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![]) //TODO or should None be returned for child-free widgets?? check this
    }
    fn focusable(&self) -> bool { true }

    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,Text> WidgetMut<'w,E> for Button<'w,E,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    Text: Caption<'w>+Statize<E>, Text::Statur: Sized,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![]
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![]
    }
    fn child_mut<'a>(&'a mut self, _: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child_mut(self: Box<Self>, _: usize) -> Result<ResolvableMut<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,S> Button<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: Caption<'w>+Statize<E>, S::Statur: Sized,
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
