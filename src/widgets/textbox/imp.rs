use super::*;
use util::{state::{AtomStateMut, AtomState}, caption::CaptionMut};

impl<'w,E,S,P> Widget<'w,E> for TextBox<'w,E,S,P> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: Caption<'w>+Statize, S::Statur: Sized,
    P: AtomState<(u32,u32)>+Statize, P::Statur: Sized,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjText]);
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
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) -> bool {
        r.with(&[
            StdVerb::ObjBorder,
            StdVerb::Focused(l.is_focused()),
        ])
            .border_rect(2);
        let off = self.scroll.get();
        let pp = self.text.caption();
        let pp = pp.as_ref();
        let pp = ESPPText::<E>::generate(pp,(20.0,20.0),&mut l.ctx);
        let siz = pp.size();
        let max_off = (
            siz.w.saturating_sub( r.b.w().saturating_sub(4) ),
            siz.h.saturating_sub( r.b.h().saturating_sub(4) ),
        );
        let off = Offset{
            x: off.0.min(max_off.0) as i32,
            y: off.1.min(max_off.1) as i32,
        };
        r.inside_border(&Border::new(4, 4, 4, 4))
            .with(&[
                StdVerb::ObjForeground,
                StdVerb::ObjText,
            ])
                .render_preprocessed_text(&pp, off, &mut l.ctx);
        true
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //e.0._debug_type_name();
        if let Some(ee) = e.0.is_text_input() {
            let s = ee.text;
            l.mutate_closure(Box::new(move |mut w,_,_| {
                let w = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                w.push(&s);
            }),true);
        } else if let Some(ee) = e.0.is_kbd_press() {
            if ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::BACKSPACE {
                l.mutate_closure(Box::new(move |mut w,_,_| {
                    let w = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                    if ee.key == EEKey::<E>::BACKSPACE {
                        w.pop(1);
                    }
                    if ee.key == EEKey::<E>::ENTER {
                        w.push("\n")
                    }
                }),true);
            }
        } else if let Some(ee) = e.0.is_mouse_scroll() {
            let off = self.scroll.get();
            let off = (
                off.0 as i32 + ee.x,
                off.1 as i32 + ee.y,
            );
            let pp = ESPPText::<E>::generate(self.text.caption().as_ref(),(20.0,20.0),&mut l.ctx);
            let siz = pp.size();
            let max_off = (
                siz.w.saturating_sub( e.1.w().saturating_sub(4) ),
                siz.h.saturating_sub( e.1.h().saturating_sub(4) ),
            );
            let off = (
                off.0.max(0).min(max_off.0 as i32) as u32,
                off.1.max(0).min(max_off.1 as i32) as u32,
            );
            l.mutate_closure(Box::new(move |mut w,_,_| {
                let w = w.traitcast_mut::<dyn AtomStateMut<(u32,u32)>>().unwrap();
                w.set(off);
            }),true);
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
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        true
    }
    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,S,P> WidgetMut<'w,E> for TextBox<'w,E,S,P> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: CaptionMut<'w>+Statize, S::Statur: Sized,
    P: AtomStateMut<(u32,u32)>+Statize, P::Statur: Sized,
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
        dyn CaptionMut => |s| &s.text;
        dyn AtomStateMut<(u32,u32)> => |s| &s.scroll;
    );
    impl_traitcast_mut!(
        dyn CaptionMut => |s| &mut s.text;
        dyn AtomStateMut<(u32,u32)> => |s| &mut s.scroll;
    );
}

