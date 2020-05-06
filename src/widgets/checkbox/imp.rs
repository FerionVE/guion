use super::*;
use util::state::AtomStateMut;
use trayt::ICheckBox;
use std::any::TypeId;
use traitcast::TraitObject;

impl<'w,E,State,Text> Widget<'w,E> for CheckBox<'w,E,State,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    State: AtomState<bool>+Statize+'w, State::Statur: Sized,
    Text: Caption<'w>+Statize+'w, Text::Statur: Sized,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        //s.attach(&[StdVerb::ObjCheckBox]);
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
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        let size = r.b.size.h;
        {
            let rect = Bounds::from_wh(size,size);
            let mut r = r.slice(&rect);
            r.with(&[
                    StdVerb::ObjForeground,
                    StdVerb::Hovered(l.is_hovered()),
                    StdVerb::Focused(l.is_focused()),
                    StdVerb::Locked(self.locked),
                    StdVerb::Pressed(self.state.get())
                ])
                .fill_rect();
            r.with(&[
                    StdVerb::ObjBorder,
                    StdVerb::Hovered(l.is_hovered()),
                    StdVerb::Focused(l.is_focused()),
                    StdVerb::Locked(self.locked),
                    //StdVerb::Pressed(self.state.get())
                ])
                .border_rect(2);
        }
        {
            let text_border = Border::new(size+r.last_border.left*2,0,0,0);
            r.inside_border(&text_border)
                .with(&[
                    StdVerb::ObjForeground,
                    StdVerb::ObjText,
                    StdVerb::Hovered(l.is_hovered()),
                    StdVerb::Focused(l.is_focused()),
                    StdVerb::Locked(self.locked),
                ])
                .render_text_aligned(self.text.caption().as_ref(),(0.0,0.5),l.ctx);
        }
        true
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //let mut invalid = false;
        if e.0.is_hover_update() || e.0.is_kbd_down().is_some() || e.0.is_kbd_up().is_some() {
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.0.is_mouse_up() {
            if ee.key == EEKey::<E>::MOUSE_LEFT && ee.down_widget.is(self.id()) && l.is_hovered() && !self.locked {
                (self.trigger)(l.reference(),!self.state.get());
                Self::toggle(l)
            }
        } else if let Some(ee) = e.0.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.is(self.id()) {
                (self.trigger)(l.reference(),!self.state.get());
                Self::toggle(l)
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
    
    fn _trace_bounds(&self, _: Link<E>, _: usize, _: &Bounds, _: bool) -> Result<Bounds,()> {
        Err(())
    }
    fn focusable(&self) -> bool { true }

    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,State,Text> WidgetMut<'w,E> for CheckBox<'w,E,State,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    State: AtomStateMut<bool>+Statize+'w, State::Statur: Sized,
    Text: Caption<'w>+Statize+'w, Text::Statur: Sized,
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

    impl_traitcast!(
        dyn ICheckBox => |s| s;
        dyn AtomState<bool> => |s| &s.state;
        dyn AtomStateMut<bool> => |s| &s.state;
    );
    impl_traitcast_mut!(
        dyn ICheckBox => |s| s;
        dyn AtomState<bool> => |s| &mut s.state;
        dyn AtomStateMut<bool> => |s| &mut s.state;
    );
}

impl<'w,E,State,Text> CheckBox<'w,E,State,Text> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    State: AtomState<bool>+Statize+'w, State::Statur: Sized,
    Text: Caption<'w>+Statize+'w, Text::Statur: Sized,
{
    pub fn toggle(mut l: Link<E>) {
        l.mutate(|mut w,_,_|{
            w.traitcast_mut::<dyn ICheckBox>().unwrap().toggle();
        },true);
    }
}
