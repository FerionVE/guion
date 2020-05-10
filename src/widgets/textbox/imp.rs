use super::*;
use util::{state::*, caption::CaptionMut};
use state::{Cursor, State};

impl<'w,E,S,P,C> Widget<'w,E> for TextBox<'w,E,S,P,C> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
    S: Caption<'w>+Statize, S::Statur: Sized,
    P: AtomStateX<E,(u32,u32)>+Statize, P::Statur: Sized,
    C: AtomStateX<E,Cursor>+Statize, C::Statur: Sized,
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
        for b in s.selection_box() {
            let b = b - s.off2();
            r.slice(&b)
                .with(&[
                    StdVerb::ObjForeground,
                ])
                .fill_rect();
        }
        if let Some(c) = s.cursor_display_pos(s.cursor.caret) { //TODO fix as it should work if cursor is at end
            let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, 20);
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
        let mut cursor = self.cursor.get(l.ctx);
        let border = Border::new(4, 4, 4, 4);
        let b = e.1.inside_border(&border);

        if let Some(ee) = e.0.is_text_input() {
            let s = ee.text;
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let mut wc = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                if cursor.is_selection() {
                    cursor.del_selection(&mut wc);
                }
                wc.push(cursor.caret as usize,&s);
                cursor.caret += s.len() as u32;
                cursor.limit(wc.len() as u32);
                cursor.unselect();
                w.traitcast_mut::<dyn AtomStateXMut<E,Cursor>>().unwrap().set(cursor,ctx);
            }),true);
        } else if let Some(ee) = e.0.is_kbd_press() {
            if
                ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::BACKSPACE ||
                ee.key == EEKey::<E>::LEFT || ee.key == EEKey::<E>::RIGHT
            {
                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let mut wc = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                    if ee.key == EEKey::<E>::BACKSPACE {
                        if cursor.is_selection() {
                            cursor.del_selection(&mut wc);
                        }else{
                            wc.pop_left(cursor.caret as usize,1);
                            cursor.unselect_sub(1);
                        }
                    }
                    if ee.key == EEKey::<E>::ENTER {
                        if cursor.is_selection() {
                            cursor.del_selection(&mut wc);
                        }
                        wc.push(cursor.caret as usize,"\n");
                        cursor.unselect_add(1);
                    }
                    if ee.key == EEKey::<E>::LEFT {
                        cursor.unselect_sub(1);
                    }
                    if ee.key == EEKey::<E>::RIGHT {
                        cursor.unselect_add(1);
                    }
                    cursor = cursor.min(wc.len() as u32);
                    w.traitcast_mut::<dyn AtomStateXMut<E,Cursor>>().unwrap().set(cursor,ctx);
                }),true);
            }
        } else if let Some(ee) = e.0.is_mouse_scroll() {
            let s = State::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,&b);
            
            let off = (
                s.off.0 as i32 + ee.x,
                s.off.1 as i32 + ee.y,
            );
            let off = s.bound_off((off.0.max(0) as u32, off.1.max(0) as u32));
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let w = w.traitcast_mut::<dyn AtomStateXMut<E,(u32,u32)>>().unwrap();
                w.set(off,ctx);
            }),true);
        } else {
            if let Some(mouse) = l.state().cursor_pos() { //TODO strange event handling
                let s = State::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,&b);

                let mut tpos = mouse - b.off + Offset::from(s.off);
                tpos.y += s.glyphs.line_ascent() as i32; //TODO FIX boundary precision all over the place
                
                if let Some(ee) = e.0.is_mouse_down() {
                    cursor.select = s.cursor_pos_reverse(tpos);
                    cursor.caret = cursor.select;
                    //cursor.unselect();
                    assert!(cursor.select < s.glyphs.chars());

                    l.mutate_closure(Box::new(move |mut w,ctx,_| {
                        w.traitcast_mut::<dyn AtomStateXMut<E,Cursor>>().unwrap().set(cursor,ctx)
                    }),true);
                } else if l.is_hovered() && l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],self.id.clone()).is_some() {
                    cursor.caret = s.cursor_pos_reverse(tpos);
                    //cursor.unselect();
                    assert!(cursor.caret < s.glyphs.chars());

                    l.mutate_closure(Box::new(move |mut w,ctx,_| {
                        w.traitcast_mut::<dyn AtomStateXMut<E,Cursor>>().unwrap().set(cursor,ctx)
                    }),true);
                }
            }
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
    P: AtomStateXMut<E,(u32,u32)>+Statize, P::Statur: Sized,
    C: AtomStateXMut<E,Cursor>+Statize, C::Statur: Sized,
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
        dyn AtomStateXMut<E,(u32,u32)> => |s| &s.scroll;
        dyn AtomStateXMut<E,Cursor> => |s| &s.cursor;
    );
    impl_traitcast_mut!(
        dyn CaptionMut => |s| &mut s.text;
        dyn AtomStateXMut<E,(u32,u32)> => |s| &mut s.scroll;
        dyn AtomStateXMut<E,Cursor> => |s| &mut s.cursor;
    );
}
