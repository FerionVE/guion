use super::*;
use std::marker::PhantomData;
use util::caption::Caption;
use state::Cursor;

pub mod widget;
pub mod state;
pub mod imp;

pub struct Area<'w,E,W,Scroll> where
    E: Env,
    W: 'w,
    Scroll: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub border: Option<Border>,
    pub inner: W,
    pub scroll: Scroll,
    p: PhantomData<&'w mut ()>,
}

/*impl<'w,E> Area<'w,E,String,(u32,u32),Cursor,Option<u32>,bool> where
    E: Env,
{
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: vec![],
            border: None,
            text: "".to_owned(),
            scroll: (0,0),
            cursor: Cursor{select: 0, caret: 0}, //TODO default trait
            cursor_stick_x: None,
            validation: false, //would work perfectly on owned, for immediate state-stored AtomStateX can be used
            p: PhantomData,
        }
    }
}*/

/*impl<'w,E,W,Scroll> Area<'w,E,W,Scroll> where
    E: Env,
    W: 'w,
    Scroll: 'w,
{
    pub fn with_text<T>(self, text: T) -> Area<'w,E,T,Scroll,Curs,CursorStickX,V> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text,
            scroll: self.scroll,
            cursor: self.cursor,
            cursor_stick_x: self.cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    //TODO use a unified state object
    pub fn with_states<PScroll,CCurs,XCursorStickX>(self, scroll: PScroll, cursor: CCurs, cursor_stick_x: XCursorStickX) -> Area<'w,E,Text,PScroll,CCurs,XCursorStickX,V> where PScroll: Statize<E>+'w, CCurs: Statize<E>+'w, XCursorStickX: Statize<E>+'w {
        Area{
            id: self.id,
            size: self.size,
            style: self.style,
            border: self.border,
            text: self.text,
            scroll,
            cursor,
            cursor_stick_x,
            validation: self.validation,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}*/

unsafe impl<'w,E,W,Scroll> Statize<E> for Area<'w,E,W,Scroll> where
    E: Env,
    W: Statize<E>, W::Statur: Sized,
    Scroll: Statize<E>, Scroll::Statur: Sized,
{
    type Statur = Area<'static,E,W::Statur,Scroll::Statur>;
}
