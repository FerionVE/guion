use super::*;
use util::{state::{AtomStateMut, AtomState}, caption::CaptionMut};

impl<'w,E,S,P,C> Widget<'w,E> for TextBox<'w,E,S,P,C> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: Caption<'w>+Statize, S::Statur: Sized,
    P: AtomState<(u32,u32)>+Statize, P::Statur: Sized,
    C: AtomState<u32>+Statize, C::Statur: Sized,
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
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) -> bool {
        r.with(&[
            StdVerb::ObjBorder,
            StdVerb::Focused(l.is_focused()),
        ])
            .border_rect(2);
        let border = Border::new(4, 4, 4, 4);
        let mut r = r.inside_border(&border);
        let s = State::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,&r.b);
        if let Some(c) = s.cursor_display_pos(s.cursor) { //TODO fix as it should work if cursor is at end
            let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 -10, 1, 20);
            let b = b - s.off2();
            r.slice(&b)
                .with(&[
                    StdVerb::ObjActive,
                ])
                .fill_rect();
        }

        r.with(&[
                StdVerb::ObjForeground,
                StdVerb::ObjText,
            ])
                .render_preprocessed_text(&s.glyphs, s.off2(), &mut l.ctx);
        true
    }
    fn _event(&self, mut l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        //e.0._debug_type_name();
        let mut cursor = self.cursor.get();

        if let Some(ee) = e.0.is_text_input() {
            let s = ee.text;
            l.mutate_closure(Box::new(move |mut w,_,_| {
                let wc = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                wc.push(&s);
                cursor += s.len() as u32;
                cursor = cursor.min(wc.len() as u32);
                w.traitcast_mut::<dyn AtomStateMut<u32>>().unwrap().set(cursor);
            }),true);
        } else if let Some(ee) = e.0.is_kbd_press() {
            if ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::BACKSPACE {
                l.mutate_closure(Box::new(move |mut w,_,_| {
                    let wc = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                    if ee.key == EEKey::<E>::BACKSPACE {
                        wc.pop(1);
                        cursor=cursor.saturating_sub(1);
                    }
                    if ee.key == EEKey::<E>::ENTER {
                        wc.push("\n");
                        cursor+=1;
                    }
                    cursor = cursor.min(wc.len() as u32);
                    w.traitcast_mut::<dyn AtomStateMut<u32>>().unwrap().set(cursor);
                }),true);
            }
        } else if let Some(ee) = e.0.is_mouse_scroll() {
            let border = Border::new(4, 4, 4, 4);
            let s = State::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,&e.1.inside_border(&border));
            let off = (
                s.off.0 as i32 + ee.x,
                s.off.1 as i32 + ee.y,
            );
            let off = s.bound_off((off.0.max(0) as u32, off.1.max(0) as u32));
            l.mutate_closure(Box::new(move |mut w,_,_| {
                let w = w.traitcast_mut::<dyn AtomStateMut<(u32,u32)>>().unwrap();
                w.set(off);
            }),true);
        }
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
    
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        true
    }
    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,S,P,C> WidgetMut<'w,E> for TextBox<'w,E,S,P,C> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: CaptionMut<'w>+Statize, S::Statur: Sized,
    P: AtomStateMut<(u32,u32)>+Statize, P::Statur: Sized,
    C: AtomStateMut<u32>+Statize, C::Statur: Sized,
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
        dyn CaptionMut => |s| &s.text;
        dyn AtomStateMut<(u32,u32)> => |s| &s.scroll;
        dyn AtomStateMut<u32> => |s| &s.cursor;
    );
    impl_traitcast_mut!(
        dyn CaptionMut => |s| &mut s.text;
        dyn AtomStateMut<(u32,u32)> => |s| &mut s.scroll;
        dyn AtomStateMut<u32> => |s| &mut s.cursor;
    );
}

pub struct State<E> where E: Env {
    off: (u32,u32),
    max_off: (u32,u32), 
    cursor: u32,
    glyphs: ESPPText<E>,
}

impl<E> State<E> where E: Env {
    pub fn retrieve<'a,S,P,C>(s: &S, p: &P, c: &C, ctx: &mut E::Context, b: &Bounds) -> Self where S: Caption<'a>, P: AtomState<(u32,u32)>, C: AtomState<u32> {
        let off = p.get();
        let caption = s.caption();
        let glyphs = ESPPText::<E>::generate(caption.as_ref(),(20.0,20.0),ctx);
        assert_eq!(glyphs.chars() as usize,caption.len()+1);
        let siz = glyphs.size();
        let max_off = (
            siz.w.saturating_sub( b.w() ),
            siz.h.saturating_sub( b.h() ),
        );
        let cursor = c.get();
        let num_glyphs = caption.len() as u32;
        let cursor = cursor.min(num_glyphs);
        Self{
            off,
            max_off,
            cursor,
            glyphs,
        }
    }
    pub fn bound_off(&self, o: (u32,u32)) -> (u32,u32) {
        (
            o.0.min(self.max_off.0),
            o.1.min(self.max_off.1),
        )
    }
    pub fn bound_off2(&self, o: (u32,u32)) -> Offset {
        Offset{
            x: o.0.min(self.max_off.0) as i32,
            y: o.1.min(self.max_off.1) as i32,
        }
    }
    pub fn off2(&self) -> Offset {
        Offset{
            x: self.off.0 as i32,
            y: self.off.1 as i32,
        }
    }

    pub fn cursor_display_pos(&self, i: u32) -> Option<(u32,u32)> {
        self.glyphs.char_at(i)
        .map(|i| 
            (
                i.offset.x as u32,
                i.offset.y as u32,
            )
        )
    }
}