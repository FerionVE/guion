use super::*;
use util::{state::*};

impl<'w,E,W,Scroll,MutFn> Widget<E> for Area<'w,E,W,Scroll,MutFn> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    for<'a> E::Context<'a>: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    W: AsWidget<E>,
    Scroll: AtomState<E,(i32,i32)>,
    MutFn: TriggerMut<E>,
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

        let (osx,osy) = self.scroll.get(l.ctx);

        let inner_size: ESize<E> = l.for_child(0).unwrap().size(&e.style);
        let (iw,ih) = (inner_size.x().preferred(),inner_size.y().preferred());

        let (sx,sy) = fix_pox((osx,osy), (iw,ih).into(), rect.size);

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
                    let (mut nx,mut ny) = (sx,sy);

                    if ee.key == MatchKeyCode::KbdUp {
                        ny = ny.saturating_sub(4);
                    }
                    if ee.key == MatchKeyCode::KbdDown {
                        ny += 4;
                    }
                    if ee.key == MatchKeyCode::KbdLeft {
                        nx = nx.saturating_sub(4);
                    }
                    if ee.key == MatchKeyCode::KbdRight {
                        nx += 4;
                    }

                    let (nx,ny) = fix_pox((nx,ny), (iw,ih).into(), rect.size);

                    let su = ScrollUpdate{offset:(nx-osx,ny-osy)};

                    if let Some(t) = self.scroll_updater.boxed(su) {
                        l.mutate_closure(t);
                        passed = true;
                    }
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
    fn childs_ref<'s>(&'s self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        vec![self.inner.as_widget_dyn(root,ctx)]
    }
    fn into_childs<'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        vec![self.inner.into_widget_dyn(root,ctx)]
    }
    
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        todo!() // TODO complete inner bounds or just view
    }
    fn focusable(&self) -> bool {
        false //TODO
    }
    fn child<'s>(&'s self, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        if i != 0 {return Err(());}
        Ok(self.inner.as_widget_dyn(root,ctx))
    }
    fn into_child<'s>(self: Box<Self>, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        if i != 0 {return Err(());}
        Ok(self.inner.into_widget_dyn(root,ctx))
    }

    impl_traitcast!( dyn Widget<E>:
        dyn AtomState<E,(i32,i32)> => |s| &s.scroll;
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

impl<'l,E,W,Scroll,MutFn> AsWidget<E> for Area<'l,E,W,Scroll,MutFn> where Self: Widget<E>, E: Env {
    type Widget = Self;
    type WidgetOwned = Self;

    #[inline]
    fn as_widget<'w>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget<'w>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(*self)
    }
    #[inline]
    fn as_widget_dyn<'w,'s>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget_dyn<'w,'s>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    #[inline]
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}
