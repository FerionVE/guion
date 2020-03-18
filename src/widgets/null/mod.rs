use super::*;

pub struct Null<E> where E: Env {
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
}

impl<E> Null<E> where E: Env {
    pub fn new(id: E::WidgetID) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: vec![],
        }
    } 
}

impl<E> Widget<'static,E> for Null<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
{
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, _: Link<E>, r: &mut RenderLink<E>) -> bool {
        r.fill_rect();
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
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> where 'static: 's {
        vec![]
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'static,E>> {
        vec![]
    }
    fn _trace_bounds(&self, _: Link<E>, _: usize, _: &Bounds, _: bool) -> Result<Bounds,()> {
        Err(())
    }
    fn focusable(&self) -> bool {
        false
    }
}

impl<E> WidgetMut<'static,E> for Null<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'static: 's {
        vec![]
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'static,E>> {
        vec![]
    }
}

unsafe impl<E> Statize<E> for Null<E> where E: Env {
    type Statur = Self;
}
