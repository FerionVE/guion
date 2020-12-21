use super::*;
use util::state::AtomStateMut;
use imp::ICheckBox;
use super::imp::ICheckBoxMut;

impl<'w,E,State,Text,Stil> Widget<E> for CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E>,
    State: AtomState<E,bool>+StatizeSized<E>,
    Text: Caption<E>+'w+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        if l.state().is_hovered(&self.id) {
            r.with(StdTag::CursorHand)
                    .set_cursor(l.ctx);
        }
        let size = r.bounds().size.h;
        {
            let rect = Bounds::from_wh(size,size);
            let mut r = r.slice(&rect);
            r.with(&[
                    StdTag::ObjForeground,
                ][..])
                .fill_rect(l.ctx);
            r.inside_border_by(&[StdTag::BorderVisual,StdTag::BorderMultiplier(3)][..],l.ctx)
                .with(&[
                    StdTag::ObjForeground,
                    StdTag::Hovered(l.is_hovered()),
                    StdTag::Focused(l.is_focused()),
                    StdTag::Locked(self.locked),
                    StdTag::Pressed(self.state.get(l.ctx))
                ][..])
                .fill_rect(l.ctx);
            r.with(&[
                    StdTag::ObjBorder,
                    StdTag::Hovered(l.is_hovered()),
                    StdTag::Focused(l.is_focused()),
                    StdTag::Locked(self.locked),
                    StdTag::BorderVisual,
                    //StdTag::Pressed(self.state.get())
                ][..])
                .fill_border_inner(l.ctx);
        }
        {
            let text_border = Border::new(size+4/*TODO fix border impl*/*2,0,0,0);
            r.inside_border_specific(&text_border)
                .with(&[
                    StdTag::ObjForeground,
                    StdTag::ObjText,
                    StdTag::Hovered(l.is_hovered()),
                    StdTag::Focused(l.is_focused()),
                    StdTag::Locked(self.locked),
                ][..])
                .render_text(self.text.caption().as_ref(),(0.0,0.5),l.ctx);
        }
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderOuter));
        //let mut invalid = false;
        if e.event.is_hover_update() || e.event.is_kbd_down().is_some() || e.event.is_kbd_up().is_some() {
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.event.is_mouse_up() {
            if ee.key == EEKey::<E>::MOUSE_LEFT && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                let new = !self.state.get(l.ctx);
                (self.trigger)(l.reference(),new);
                Self::set(l,new);
                return true;
            }
        } else if let Some(ee) = e.event.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.is(self.id()) {
                let new = !self.state.get(l.ctx);
                (self.trigger)(l.reference(),new);
                Self::set(l,new);
                return true;
            }
        }
        e.event.is_mouse_down().is_some()
    }
    fn _size(&self, _: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref(&self) -> Vec<Resolvable<'_,E>> {
        vec![]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &ESVariant<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool { true }

    fn child(&self, _: usize) -> Result<Resolvable<'_,E>,()> {
        Err(())
    }
    fn into_child<'a>(self: Box<Self>, _: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast!(
        dyn ICheckBox<E> => |s| s;
        dyn AtomState<E,bool> => |s| &s.state;
    );
}

impl<'w,E,State,Text,Stil> WidgetMut<E> for CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E>,
    State: AtomStateMut<E,bool>+StatizeSized<E>,
    Text: Caption<E>+'w+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<'_,E>> {
        vec![]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![]
    }
    fn child_mut(&mut self, _: usize) -> Result<ResolvableMut<'_,E>,()> {
        Err(())
    }
    fn into_child_mut<'a>(self: Box<Self>, _: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast_mut!(
        dyn ICheckBox<E> => |s| s;
        dyn ICheckBoxMut<E> => |s| s;
        dyn AtomState<E,bool> => |s| &mut s.state;
        dyn AtomStateMut<E,bool> => |s| &mut s.state;
    );
}

impl<'w,E,State,Text,Stil> CheckBox<'w,E,State,Text,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>>,
    E::Context: CtxStdState<E>,
    State: AtomState<E,bool>+StatizeSized<E>+'w,
    Text: Caption<E>+'w+StatizeSized<E>,
{
    pub fn set(mut l: Link<E>, v: bool) {
        l.mutate_closure(Box::new(move |mut w,c,_|{
            //w.traitcast_mut::<dyn AtomStateMut<E,bool>>().unwrap().set(v,c);
            let w = w.traitcast_mut::<dyn ICheckBoxMut<E>>().unwrap();
            w.state_mut().set(v,c);
        }));
    }
}
