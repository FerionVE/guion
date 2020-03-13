use super::*;
use crate::core::*;
use crate::core::event::key::Key;

pub struct Beton<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
{
    pub trigger: for<'a> fn(Link<E>),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    //pressed: Option<EEKey<E>>,
}

impl<E> Widget<E> for Beton<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
{
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjBackground,StdVerb::Accent(0)]);
        s.attach(&self.style[..]);
    }
    fn border(&self, b: &mut Border) {
        
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        if Self::pressed(&l).is_some() {
            //panic!("baka");
            r.fill_rect();
        }else{
            if l.is_hovered() {
                r.with(&[StdVerb::Hovered(true)])
                    .fill_rect();
            }
            r.border_rect(2);
        }
        true
    }
    fn event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //let mut invalid = false;
        if e.0.is_hover_update() {
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.0.is_mouse_up() {
            if ee.down_widget.tip().eq_id(self.id()) && l.is_hovered() {
                (self.trigger)(l)
            }
        }
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        self.size.clone()
    }
    fn childs(&self) -> usize {
        0
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> {
        vec![]
    }
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> {
        vec![]   
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        Err(())
    }
    fn focusable(&self) -> bool { true }
    
}

impl<E> Beton<E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
{
    pub fn new(id: E::WidgetID, size: ESize<E>) -> Self {
        Self{
            id,
            size,
            style: vec![],
            trigger: |_|{},
            //pressed: None,
        }
    }

    pub fn pressed<'l:'s,'s>(l: &'s Link<'l,E>) -> Option<&'s EPressedKey<E>> {
        let id = l.id();
        l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],id)
    }
}