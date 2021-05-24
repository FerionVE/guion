use super::*;
use super::super::util::state::*;

impl<'w,E> Widget<E> for ProgressBar<'w,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
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
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        vec![]
    }
    fn into_childs<'a>(self: Box<Self>) -> Vec<Resolvable<'a,E>> where Self: 'a {
        vec![]
    }
    fn child_bounds(&self, _: Link<E>, _: &Bounds, e: &EStyle<E>, _: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }
    fn child(&self, _: usize) -> Result<Resolvable<E>,()> {
        Err(())
    }
    fn into_child<'a>(self: Box<Self>, _: usize) -> Result<Resolvable<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast!(
        dyn AtomState<E,f32> => |s| &s.value;
    );
}

impl<'w,E> WidgetMut<E> for ProgressBar<'w,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
{
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        vec![]
    }
    fn into_childs_mut<'a>(self: Box<Self>) -> Vec<ResolvableMut<'a,E>> where Self: 'a {
        vec![]
    }
    fn child_mut(&mut self, _: usize) -> Result<ResolvableMut<E>,()> {
        Err(())
    }
    fn into_child_mut<'a>(self: Box<Self>, _: usize) -> Result<ResolvableMut<'a,E>,()> where Self: 'a {
        Err(())
    }

    impl_traitcast_mut!(
        dyn AtomStateMut<E,f32> => |s| &mut s.value;
    );
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> Bounds {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.clamp(0.0,1.0) ) as u32;

    Bounds::from_ori(x, y, w, h, o)
}
