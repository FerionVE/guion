use super::*;
use util::{state::*, caption::CaptionMut};
use state::{Cursor, TBState};
use super::imp::ITextBoxMut;

impl<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> Widget<'w,E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>, //TODO make clipboard support optional; e.g. generic type ClipboardAccessProxy
    Text: Caption<'w>+StatizeSized<E>,
    Scroll: AtomState<E,(u32,u32)>+StatizeSized<E>,
    Curs: AtomState<E,Cursor>+StatizeSized<E>,
    CursorStickX: AtomState<E,Option<u32>>+StatizeSized<E>,
    V: AtomState<E,bool>+StatizeSized<E>,
    Stil: StatizeSized<E>+Clone,
{
    fn child_paths(&self, _: E::WidgetPath) -> Vec<E::WidgetPath> {
        vec![]
    }
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn _render(&self, mut l: Link<E>, r: &mut RenderLink<E>) {
        let mut r = r.with(&self.style);
        let mut r = r.inside_border_by(StdTag::BorderOuter,l.ctx);
        r.with(&[
            StdTag::ObjBorder,
            StdTag::Focused(l.is_focused()),
            StdTag::BorderVisual,
        ][..])
            .fill_border_inner(l.ctx);
        let mut r = r.inside_border_by(&[StdTag::BorderVisual,StdTag::BorderMultiplier(2)][..],l.ctx);
        let s = TBState::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,r.bounds());
        for b in s.selection_box() {
            let b = b - s.off2();
            r.slice(&b)
                .with(StdTag::ObjForeground)
                .fill_rect(l.ctx);
        }
        if let Some(c) = s.cursor_display_pos(s.cursor.caret) { //TODO fix as it should work if cursor is at end
            let b = Bounds::from_xywh(c.0 as i32, c.1 as i32 - s.glyphs.line_ascent() as i32, 2, s.glyphs.line_height());
            let b = b - s.off2();
            r.slice(&b)
                .with(StdTag::ObjActive)
                .fill_rect(l.ctx);
        }

        r.with(&[
                StdTag::ObjForeground,
                StdTag::ObjText,
            ][..])
                .render_preprocessed_text(&s.glyphs, s.off2(), &mut l.ctx);
    }
    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> EventResp {
        let e = try_or_false!(e.filter_bounds_by_border(l.style_provider(),StdTag::BorderOuter));

        //e.0._debug_type_name();
        let mut cursor = self.cursor.get(l.ctx);
        let border = l.style_provider().border(
            &StyleVariantSupport::new_with(
                &[StdTag::BorderVisual,StdTag::BorderMultiplier(2)][..]
            )
        );
        let b = e.1.inside_border(&border);

        let mut passed = false;

        if let Some(ee) = e.0.is_text_input() {
            let s = ee.text;
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                w.insert_text(&s,ctx);
                w.scroll_to_cursor(ctx,&b);
            }));
            passed = true;
        } else if let Some(ee) = e.0.is_kbd_press() {
            if
                ee.key == EEKey::<E>::ENTER || ee.key == EEKey::<E>::BACKSPACE ||
                ee.key == EEKey::<E>::LEFT || ee.key == EEKey::<E>::RIGHT
            {
                let ctrl = l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some();

                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let mut w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                    if ee.key == EEKey::<E>::BACKSPACE {
                        w.remove_selection_or_n(1,ctx);
                    }
                    if ee.key == EEKey::<E>::ENTER {
                        w.insert_text("\n",ctx);
                    }
                    if ee.key == EEKey::<E>::LEFT {
                        w.move_cursor_x(-1,ctrl,ctx);
                    }
                    if ee.key == EEKey::<E>::RIGHT {
                        w.move_cursor_x(1,ctrl,ctx);
                    }
                    w.scroll_to_cursor(ctx,&b);
                }));
                passed = true;
            }else if ee.key == EEKey::<E>::A && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let wc = w.traitcast_mut::<dyn CaptionMut>().unwrap();
                    cursor.select = 0;
                    cursor.caret = wc.len() as u32;
                    w.traitcast_mut::<dyn AtomStateMut<E,Cursor>>().unwrap().set(cursor,ctx);
                    w.traitcast_mut::<dyn AtomStateMut<E,Option<u32>>>().unwrap().set(None,ctx);
                }));
                passed = true;
            }else if ee.key == EEKey::<E>::V && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                if let Some(text) = l.clipboard_get_text() {
                    self._event_direct(l,&EventCompound(Event::from(TextInput{text}),e.1,e.2,Default::default(),e.4.clone(),e.5));
                }
                passed = true;
            }else if (ee.key == EEKey::<E>::C || ee.key == EEKey::<E>::X) && l.state().is_pressed(&[EEKey::<E>::CTRL]).is_some() {
                if cursor.is_selection() {
                    let range = cursor.range_usize();
                    let text = self.text.caption();
                    let text = &text.as_ref()[range];
                    l.clipboard_set_text(text);

                    if ee.key == EEKey::<E>::X {
                        l.mutate_closure(Box::new(move |mut w,ctx,_| {
                            let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                            w.remove_selection(ctx);
                            w.scroll_to_cursor(ctx,&b);
                        }));
                    }
                }
                passed = true;
            }else if ee.key == EEKey::<E>::UP || ee.key == EEKey::<E>::DOWN {
                let b = b.clone();
                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();

                    if ee.key == EEKey::<E>::UP {
                        w.move_cursor_y(-1,false,ctx,&b);
                    }
                    if ee.key == EEKey::<E>::DOWN {
                        w.move_cursor_y(1,false,ctx,&b);
                    }
                    w.scroll_to_cursor(ctx,&b);
                }));
            }
            passed = true;
        } else if let Some(ee) = e.0.is_mouse_scroll() {
            let s = TBState::<E>::retrieve(&self.text,&self.scroll,&self.cursor,&mut l.ctx,&b);
            
            let off = (
                s.off.0 as i32 + ee.x,
                s.off.1 as i32 + ee.y,
            );
            let off = s.bound_off((off.0.max(0) as u32, off.1.max(0) as u32));
            l.mutate_closure(Box::new(move |mut w,ctx,_| {
                let w = w.traitcast_mut::<dyn AtomStateMut<E,(u32,u32)>>().unwrap();
                w.set(off,ctx);
            }));
            passed = true;
        } else {
            if let Some(mouse) = l.state().cursor_pos() { //TODO strange event handling

                let mouse_down = e.0.is_mouse_down();
                let mouse_pressed = l.is_hovered() && l.state().is_pressed_and_id(&[EEKey::<E>::MOUSE_LEFT],self.id.clone()).is_some();
                let b = b.clone();

                l.mutate_closure(Box::new(move |mut w,ctx,_| {
                    let w = w.traitcast_mut::<dyn ITextBoxMut<E>>().unwrap();
                    w._m(mouse_down,mouse_pressed,mouse,b,ctx);
                    if mouse_pressed {
                        w.scroll_to_cursor(ctx,&b);
                    }
                }));
            }
            passed = true;
        }
        passed
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
        true
    }
    fn child<'a>(&'a self, _: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        Err(())
    }
    fn into_child(self: Box<Self>, _: usize) -> Result<Resolvable<'w,E>,()> {
        Err(())
    }
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> WidgetMut<'w,E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdTag<E>> + for<'z> StyleVariantSupport<&'z [StdTag<E>]> + for<'z> StyleVariantSupport<&'z Stil>,
    E::Context: CtxStdState<E> + CtxClipboardAccess<E>,
    Text: CaptionMut<'w>+StatizeSized<E>,
    Scroll: AtomStateMut<E,(u32,u32)>+StatizeSized<E>,
    Curs: AtomStateMut<E,Cursor>+StatizeSized<E>,
    CursorStickX: AtomStateMut<E,Option<u32>>+StatizeSized<E>,
    V: AtomState<E,bool>+StatizeSized<E>,
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
        dyn CaptionMut => |s| &s.text;
        dyn AtomStateMut<E,(u32,u32)> => |s| &s.scroll;
        dyn AtomStateMut<E,Cursor> => |s| &s.cursor;
        dyn AtomStateMut<E,Option<u32>> => |s| &s.cursor_stick_x;
        dyn ITextBoxMut<'w,E> => |s| s;
    );
    impl_traitcast_mut!(
        dyn CaptionMut => |s| &mut s.text;
        dyn AtomStateMut<E,(u32,u32)> => |s| &mut s.scroll;
        dyn AtomStateMut<E,Cursor> => |s| &mut s.cursor;
        dyn AtomStateMut<E,Option<u32>> => |s| &mut s.cursor_stick_x;
        dyn ITextBoxMut<'w,E> => |s| s;
    );
}

macro_rules! akw {
    () => {
        V: AtomState<E,bool>+Statize<E>, V::Statur: Sized,
    }
}