use super::*;
use util::{state::*};

impl<'w,E,W,Scroll> Widget<E> for Area<'w,E,W,Scroll> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<E>+'w,
    Scroll: AtomState<E,(i32,i32)>,
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

        let (sx,sy) = fix_pox((sx,sy), (iw,ih).into(), rect.size);

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

        let (sx,sy) = fix_pox((sx,sy), (iw,ih).into(), rect.size);

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
                    ee.key == MatchKeyCode::KbdUp || ee.key == MatchKeyCode::KbdDown ||
                    ee.key == MatchKeyCode::KbdLeft || ee.key == MatchKeyCode::KbdRight
                {
                    l.mutate_closure(Box::new(move |mut w,ctx,_| {
                        let w = w.traitcast_mut::<dyn AtomStateMut<E,(i32,i32)>>().unwrap();
                        let mut v = w.get(ctx);
                        if ee.key == MatchKeyCode::KbdUp {
                            v.1 = v.1.saturating_sub(4);
                        }
                        if ee.key == MatchKeyCode::KbdDown {
                            v.1 += 4;
                        }
                        if ee.key == MatchKeyCode::KbdLeft {
                            v.0 = v.0.saturating_sub(4);
                        }
                        if ee.key == MatchKeyCode::KbdRight {
                            v.0 += 4;
                        }
                        let v = fix_pox(v, (iw,ih).into(), rect.size);
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
        dyn AtomState<E,(i32,i32)> => |s| &s.scroll;
    );
}

impl<'w,E,W,Scroll> WidgetMut<E> for Area<'w,E,W,Scroll> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>,
    W: AsWidgetMut<E>+'w,
    Scroll: AtomStateMut<E,(i32,i32)>,
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
        dyn AtomState<E,(i32,i32)> => |s| &mut s.scroll;
        dyn AtomStateMut<E,(i32,i32)> => |s| &mut s.scroll;
    );
}

fn fix_pox(viewport_off: (i32,i32), inner_size: Dims, viewport_size: Dims) -> (i32,i32) {
    (
        fix_pos(inner_size.w, viewport_size.w, viewport_off.0),
        fix_pos(inner_size.h, viewport_size.h, viewport_off.1),
    )
}

fn fix_pos(inner_size: u32, viewport_size: u32, viewport_off: i32) -> i32 {
    if viewport_size > inner_size {
        viewport_off.min(0).max(inner_size as i32 - viewport_size as i32)
    }else{
        viewport_off.max(0).min(inner_size as i32 - viewport_size as i32)
    }
}
