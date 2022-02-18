use super::*;
use super::super::util::state::*;

impl<'w,E> Widget<E> for ProgressBar<'w,E> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut ERenderer<'_,E>) {
        let mut r = r.with_style(&self.style);
        let mut r = r.inside_border_by(StdSelectag::BorderOuter,l.ctx);
        r.with(StdSelectag::ObjBackground)
            .fill_rect(l.ctx);
        r.slice_abs(&crop(r.bounds(), self.value, self.orientation))
            .with(StdSelectag::ObjActive)
            .fill_rect(l.ctx);
        r.with(&[StdSelectag::ObjBorder,StdSelectag::BorderVisual][..])
            .fill_border_inner(l.ctx);
    }
    fn _event_direct(&self, _: Link<E>, _: &EventCompound<E>) -> EventResp {
        false
    }
    fn _size(&self, _: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'s>(&'s self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> {
        vec![]
    }
    fn into_childs<'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        vec![]
    }
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }
    fn child<'s>(&'s self, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        Err(())
    }
    fn into_child<'s>(self: Box<Self>, _: usize, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        Err(())
    }

    impl_traitcast!( dyn Widget<E>:
        dyn AtomState<E,f32> => |s| &s.value;
    );
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> Bounds {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.clamp(0.0,1.0) ) as u32;

    Bounds::from_ori(x, y, w, h, o)
}

impl<'l,E> AsWidget<E> for ProgressBar<'l,E> where Self: Widget<E>, E: Env {
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
