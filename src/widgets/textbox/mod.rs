use super::*;
use std::marker::PhantomData;
use util::caption::Caption;
use state::Cursor;

pub mod widget;
pub mod state;
pub mod imp;

pub struct TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    V: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub text: Text,
    pub scroll: Scroll,
    pub cursor: Curs,
    pub cursor_stick_x: CursorStickX,
    pub validation: V,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> TextBox<'w,E,String,(u32,u32),Cursor,Option<u32>,bool,()> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: (),
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Cursor{select: 0, caret: 0}, //TODO default trait
            cursor_stick_x: None,
            validation: false, //would work perfectly on owned, for immediate state-stored AtomStateX can be used
            p: PhantomData,
        }
    }
}

impl<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> where
    E: Env,
    Text: 'w,
    Scroll: 'w,
    Curs: 'w,
    CursorStickX: 'w,
    V: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> TextBox<'w,E,T,Scroll,Curs,CursorStickX,V,Stil> where T: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    #[inline]
    pub fn with_states<PScroll,CCurs,XCursorStickX>(self, scroll: PScroll, cursor: CCurs, cursor_stick_x: XCursorStickX) -> TextBox<'w,E,Text,PScroll,CCurs,XCursorStickX,V,Stil> where PScroll: 'w, CCurs: 'w, XCursorStickX: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style: self.style,
            text: self.text,
            scroll,
            cursor,
            cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,SStil> where SStil: 'w {
        TextBox{
            id: self.id,
            size: self.size,
            style,
            text: self.text,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            scroll: self.scroll,
            validation: self.validation,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> Statize<E> for TextBox<'w,E,Text,Scroll,Curs,CursorStickX,V,Stil> where
    E: Env,
    Text: StatizeSized<E>,
    Scroll: StatizeSized<E>,
    Curs: StatizeSized<E>,
    CursorStickX: StatizeSized<E>,
    V: StatizeSized<E>,
    Stil: StatizeSized<E>,
{
    type Statur = TextBox<'static,E,Text::StaturSized,Scroll::StaturSized,Curs::StaturSized,CursorStickX::StaturSized,V::StaturSized,Stil::StaturSized>;
}
