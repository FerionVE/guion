use super::*;
use crate::core::event::key::Key;
use std::marker::PhantomData;

pub struct Button<'w,E,S> where
    E: Env,
    S: AsCaption<'w>,
{
    pub trigger: for<'a> fn(Link<'a,E>),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: S,
    p: PhantomData<&'w mut ()>,
}

impl<'w,E,S> Widget<'w,E> for Button<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: AsCaption<'w>,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn style(&self, s: &mut ESVariant<E>) {
        s.attach(&[StdVerb::ObjButton]);
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
        r.with(&[
            StdVerb::ObjForeground,
            StdVerb::Hovered(l.is_hovered()),
            StdVerb::Focused(l.is_focused()),
            StdVerb::Locked(self.locked),
            StdVerb::Pressed(Self::pressed(&l).is_some())
        ])
            .fill_rect();
        r.with(&[
            StdVerb::ObjBorder,
            StdVerb::Hovered(l.is_hovered()),
            StdVerb::Focused(l.is_focused()),
            StdVerb::Locked(self.locked),
            StdVerb::Pressed(Self::pressed(&l).is_some())
        ])
            .border_rect(2);
        self.text.cap(|s| 
            r.with(&[
                StdVerb::ObjForeground,
                StdVerb::ObjText,
                StdVerb::Hovered(l.is_hovered()),
                StdVerb::Focused(l.is_focused()),
                StdVerb::Locked(self.locked),
                StdVerb::Pressed(Self::pressed(&l).is_some())
            ])
                .render_text(s,l.ctx)
        );
        true
    }
    fn event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //let mut invalid = false;
        if e.0.is_hover_update() || e.0.is_kbd_down().is_some() || e.0.is_kbd_up().is_some() {
            l.enqueue_invalidate()
        }
        if let Some(ee) = e.0.is_mouse_up() {
            if ee.down_widget.tip().eq_id(self.id()) && l.is_hovered() && !self.locked {
                (self.trigger)(l)
            }
        } else if let Some(ee) = e.0.is_kbd_press() {
            if (ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::SPACE) && ee.down_widget.tip().eq_id(self.id()) {
                (self.trigger)(l)
            }
        }
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
    fn focusable(&self) -> bool { true }
}

impl<'w,E,S> WidgetMut<'w,E> for Button<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: AsCaption<'w>,
{
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> where 'w: 's {
        vec![]
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        vec![]
    }
}

impl<'w,E,S> Button<'w,E,S> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: AsCaption<'w>,
{
    pub fn new(id: E::WidgetID, size: ESize<E>) -> Self {
        Self{
            id,
            size,
            style: vec![],
            trigger: |_|{},
            locked: false,
            border: None,
            text: S::default(),
            p: PhantomData,
        }
    }

    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>)) -> Self {
        self.trigger = fun;
        self
    }
    pub fn with_text(mut self, text: S) -> Self {
        self.text = text;
        self
    }

    pub fn pressed<'l:'s,'s>(l: &'s Link<'l,E>) -> Option<&'s EPressedKey<E>> {
        let id = l.id();
        l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],id.clone())
            .or_else(||
                l.state().is_pressed_and_id(&[EEKey::<E>::ENTER],id.clone())
            )
            .or_else(||
                l.state().is_pressed_and_id(&[EEKey::<E>::SPACE],id)
            )
    }
}

unsafe impl<'w,E,S> Statize<E> for Button<'w,E,S> where
    E: Env,
    S: AsCaption<'w>,
{
    type Statur = Button<'static,E,S::Statur>;
}

pub trait AsCaption<'w>: Clone + 'w {
    type Statur: AsCaption<'static> + 'static;

    fn cap(&self, f: impl FnOnce(&str));
    fn default() -> Self;
}

impl<'w> AsCaption<'w> for &'w str {
    type Statur = &'static str;

    fn cap(&self, f: impl FnOnce(&str)) {
        f(self)
    }
    fn default() -> Self{
        "Button"
    }
}
impl<'w> AsCaption<'w> for &'w String {
    type Statur = &'static String;
    fn cap(&self, f: impl FnOnce(&str)) {
        f(self)
    }
    fn default() -> Self{
        Box::leak(Box::new("Button".to_owned()))
    }
}
impl<'w> AsCaption<'w> for String {
    type Statur = String;
    fn cap(&self, f: impl FnOnce(&str)) {
        f(self)
    }
    fn default() -> Self{
        "Button".to_owned()
    }
}