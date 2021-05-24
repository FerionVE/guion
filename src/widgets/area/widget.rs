use super::*;
use util::{state::*};

impl<'w,E,W,Scroll> Widget<E> for Area<'w,E,W,Scroll> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<E>+'w,
    Scroll: AtomState<E,(u32,u32)>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        r.with(&[
            StdSelectag::ObjBorder,
            StdSelectag::Focused(l.is_focused()),
            StdSelectag::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        let mut r = r.inside_border_by(StdSelectag::BorderVisual,l.ctx);

        let rect = *r.bounds();

        let (sx,sy) = self.scroll.get(l.ctx);

        let inner_size: ESize<E> = l.for_child(0).unwrap().size(r.style());
        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        r.fork_with(Some(inner_rect), Some(rect), None, None)
            .render_widget(l.for_child(0).unwrap());
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = e.with_style(&self.style);
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderOuter,l.ctx));
        let e = try_or_false!(e.filter_inside_bounds_by_style(StdSelectag::BorderVisual,l.ctx));
        
        let rect = e.bounds;

        let (sx,sy) = self.scroll.get(l.ctx);

        let inner_size: ESize<E> = l.for_child(0).unwrap().size(&e.style);
        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let inner_rect = Bounds::from_xywh(rect.x()-sx as i32, rect.y()-sy as i32, iw, ih);

        let mut passed = false;

        {
            let mut l = l.for_child(0).unwrap();
            let e = e.with_bounds(inner_rect);
            if let Some(ee) = e.filter(&l) { //TODO API OOF not filtering breaks for_child mechanism
                passed |= l.event_direct(&ee);
            }
        }

        if !passed {
            if let Some(ee) = e.event.is_kbd_press() {
                if
                    ee.key == EEKey::<E>::UP || ee.key == EEKey::<E>::DOWN ||
                    ee.key == EEKey::<E>::LEFT || ee.key == EEKey::<E>::RIGHT
                {
                    l.mutate_closure(Box::new(move |mut w,ctx,_| {
                        let w = w.traitcast_mut::<dyn AtomStateMut<E,(u32,u32)>>().unwrap();
                        let mut v = w.get(ctx);
                        if ee.key == EEKey::<E>::UP {
                            v.1 = v.1.saturating_sub(4);
                        }
                        if ee.key == EEKey::<E>::DOWN {
                            v.1 += 4;
                        }
                        if ee.key == EEKey::<E>::LEFT {
                            v.0 = v.0.saturating_sub(4);
                        }
                        if ee.key == EEKey::<E>::RIGHT {
                            v.0 += 4;
                        }
                        w.set(v,ctx);
                    }));
                    passed = true;
                }
            }
        }

        passed
    }
    fn _size(&self, _: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
        self.size.clone()
    }
    fn childs(&self) -> usize {
        1
    }
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        vec![self.inner.as_ref()]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![self.inner.into_ref()]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        todo!() // TODO complete inner bounds or just view
    }
    fn focusable(&self) -> bool {
        false //TODO
    }
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        if i != 0 {return Err(());}
        Ok(self.inner.as_ref())
    }
    fn into_child<'a>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'a {
        if i != 0 {return Err(());}
        Ok(self.inner.into_ref())
    }

    impl_traitcast!(
        dyn AtomState<E,(u32,u32)> => |s| &s.scroll;
    );
}

impl<'w,E,W,Scroll> WidgetMut<E> for Area<'w,E,W,Scroll> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>+'r,
    EEvent<E>: StdVarSup<E>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    W: AsWidgetMut<E>+'w,
    Scroll: AtomStateMut<E,(u32,u32)>,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        vec![self.inner.as_mut()]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![self.inner.into_mut()]
    }
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        if i != 0 {return Err(());}
        Ok(self.inner.as_mut())
    }
    fn into_child_mut<'a>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        if i != 0 {return Err(());}
        Ok(self.inner.into_mut())
    }

    impl_traitcast_mut!(
        dyn AtomState<E,(u32,u32)> => |s| &mut s.scroll;
        dyn AtomStateMut<E,(u32,u32)> => |s| &mut s.scroll;
    );
}
