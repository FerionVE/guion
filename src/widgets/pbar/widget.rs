use super::*;

impl<'w,E,Stil> Widget<'w,E> for ProgressBar<'w,E,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdTag> + StyleVariantSupport<Stil>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.inside_border(self.border.as_ref().unwrap_or(l.default_border()));
        r.with(&[
            StdTag::ObjBackground,
        ])
            .fill_rect();
        r.slice_abs(&crop(&r.b, self.value, self.orientation))
            .with(&[
                StdTag::ObjActive,
            ])
            .fill_rect();
        r.with(&[
            StdTag::ObjBorder,
        ])
            .border_rect(l.default_thicc());
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
}

impl<'w,E,Stil> WidgetMut<'w,E> for ProgressBar<'w,E,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdTag> + StyleVariantSupport<Stil>,
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

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> Bounds {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.max(0.0).min(1.0) ) as u32;

    Bounds::from_ori(x, y, w, h, o)
}