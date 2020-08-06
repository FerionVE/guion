use super::*;
use super::super::util::state::*;

impl<'w,E,Stil> Widget<'w,E> for ProgressBar<'w,E,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    Stil: StatizeSized<E>+Clone,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        r.with(StdTag::ObjBackground)
            .fill_rect(l.ctx);
        r.slice_abs(&crop(r.bounds(), self.value, self.orientation))
            .with(StdTag::ObjActive)
            .fill_rect(l.ctx);
        r.with(&[StdTag::ObjBorder,StdTag::BorderVisual][..])
            .fill_border_inner(l.ctx);
    }
    fn _event_direct(&self, _: Link<E>, _: &EventCompound<E>) -> EventResp {
        false
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
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }
    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }

    impl_traitcast!(
        dyn AtomState<E,f32> => |s| &s.value;
    );
}

impl<'w,E,Stil> WidgetMut<'w,E> for ProgressBar<'w,E,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    Stil: StatizeSized<E>+Clone,
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
        dyn AtomStateMut<E,f32> => |s| &s.value;
    );
    impl_traitcast_mut!(
        dyn AtomStateMut<E,f32> => |s| &mut s.value;
    );
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> Bounds {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.max(0.0).min(1.0) ) as u32;

    Bounds::from_ori(x, y, w, h, o)
}