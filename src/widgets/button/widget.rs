use super::*;

impl<'w,E,Text,Stil> Widget<'w,E> for Button<'w,E,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag> + StyleVariantSupport<Stil>,
    E::Context: CtxStdState<E>,
    Text: Caption<'w>+StatizeSized<E>,
    Stil: StatizeSized<E>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.inside_border(self.border.as_ref().unwrap_or(l.default_border()));
        r.with(&[
            StdTag::ObjForeground,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some())
        ])
            .fill_rect();
        r.with(&[
            StdTag::ObjBorder,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some())
        ])
            .border_rect(l.default_thicc());
        r.with(&[
            StdTag::ObjForeground,
            StdTag::ObjText,
            StdTag::Hovered(l.is_hovered()),
            StdTag::Focused(l.is_focused()),
            StdTag::Locked(self.locked),
            StdTag::Pressed(Self::pressed(&l).is_some())
        ])
            .render_text(self.text.caption().as_ref(),l.ctx);
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = 
            if let Some(e) =
                e.inside_border( self.border.as_ref()
                    .unwrap_or(l.default_border())
                ).filter_bounds()
            {
                //eprintln!("E");
                e
            }else{
                //eprintln!("X");
                return false;
            };
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
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, _: bool) -> Result<Vec<Bounds>,()> {
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

impl<'w,E,Text,Stil> WidgetMut<'w,E> for Button<'w,E,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag> + StyleVariantSupport<Stil>,
    E::Context: CtxStdState<E>,
    Text: Caption<'w>+StatizeSized<E>,
    Stil: StatizeSized<E>,
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

impl<'w,E,S,Stil> Button<'w,E,S,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag>,
    E::Context: CtxStdState<E>,
    S: Caption<'w>+StatizeSized<E>
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
