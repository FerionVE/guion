use super::*;

pub struct ProgressBar<E> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub value: f32,
    pub orientation: Orientation,
}

impl<E> ProgressBar<E> where E: Env {
    pub fn new(id: E::WidgetID, o: Orientation) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            value: 0.0,
            orientation: o,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }

    pub fn with_value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }
}

impl<'w,E> Widget<'w,E> for ProgressBar<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, _: Link<E>, r: &mut RenderLink<E>) -> bool {
        r.with(&[
            StdVerb::ObjBackground,
        ])
            .fill_rect();
        r.slice_abs(&crop(&r.b, self.value, self.orientation))
            .with(&[
                StdVerb::ObjActive,
            ])
            .fill_rect();
        r.with(&[
            StdVerb::ObjBorder,
        ])
            .border_rect(2);
        true
    }
    fn event(&self, _: Link<E>, _: (EEvent<E>,&Bounds,u64)) {
        
    }
    fn size(&self, _: Link<E>) -> ESize<E> {
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'w: 's {
        vec![]
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        vec![]
    }
    fn _trace_bounds(&self, _: Link<E>, _: usize, _: &Bounds, _: bool) -> Result<Bounds,()> {
        Err(())
    }
    fn focusable(&self) -> bool {
        false
    }
    fn border(&self, b: &mut Border) {
        if let Some(senf) = &self.border {
            *b = *senf;
        }
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjDefault]);
        s.attach(&self.style[..]);
    }
}

impl<'w,E> WidgetMut<'w,E> for ProgressBar<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![]
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![]
    }
}

unsafe impl<E> Statize<E> for ProgressBar<E> where E: Env {
    type Statur = Self;
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> Bounds {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.max(0.0).min(1.0) ) as u32;

    Bounds::from_ori(x, y, w, h, o)
}