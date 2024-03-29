use super::*;

pub struct Null<E> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: EStyle<E>,
}

impl<E> Null<E> where E: Env {
    pub fn new(id: E::WidgetID) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: vec![],
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

impl<'w,E> Widget<'w,E> for Null<E> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EStyle<E>: StyleVariantSupport<StdSelectag>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, _: Link<E>, r: &mut ERenderer<'_,E>) {
        r.fill_rect();
    }
    fn _event_direct(&self, _: Link<E>, _: &EventCompound<E>) -> EventResp {
        //let e = e.with_style(&self.style);
        false
    }
    fn _size(&self, _: Link<E>, e: &EStyle<E>) -> ESize<E> {
        let e = e.and(&self.style);
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
        false
    }
    fn border(&self, b: &mut Border) {
        if let Some(senf) = &self.border {
            *b = *senf;
        }
    }
    fn style(&self, s: &mut EStyle<E>) {
        s.attach(&[StdSelectag::ObjDefault]);
        s.attach(&self.style[..]);
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E> WidgetMut<'w,E> for Null<E> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EStyle<E>: StyleVariantSupport<StdSelectag>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![]
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![]
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        Err(())
    }
}

unsafe impl<E> Statize<E> for Null<E> where E: Env {
    type Statur = Self;
}
