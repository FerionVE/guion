use super::*;
use std::marker::PhantomData;
use button::AsCaption;

pub struct Label<'w,E,S> where
    E: Env,
    S: AsCaption<'w>,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub text: S,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E,S> Widget<'w,E> for Label<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    S: AsCaption<'w>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjText]);
        s.attach(&self.style[..]);
    }
    fn border(&self, b: &mut Border) {
        if let Some(senf) = &self.border {
            *b = *senf;
        }
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        self.text.cap(|s| 
            r.with(&[
                StdVerb::ObjForeground,
                StdVerb::ObjText,
            ])
                .render_text(s,l.ctx)
        );
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
}

impl<'w,E,S> WidgetMut<'w,E> for Label<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    S: AsCaption<'w>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![]
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![]
    }
}

impl<'w,E,S> Label<'w,E,S> where
    E: Env,
    S: AsCaption<'w>,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            text: S::default(),
            p: PhantomData,
        }
    }

    pub fn with_text(mut self, text: S) -> Self {
        self.text = text;
        self
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,S> Statize<E> for Label<'w,E,S> where
    E: Env,
    S: AsCaption<'w>,
{
    type Statur = Label<'static,E,S::Statur>;
}
