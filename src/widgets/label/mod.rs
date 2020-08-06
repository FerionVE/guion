use super::*;
use std::marker::PhantomData;
use util::caption::Caption;

pub mod widget;

pub struct Label<'w,E,S,Stil> where
    E: Env,
    S: 'w,
    Stil: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub text: S,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> Label<'w,E,&'static str,()> where
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID) -> Self {
        Self{
            id,
            size: Size::empty().into(),
            style: (),
            text: "",
            p: PhantomData,
        }
    }
}

impl<'w,E,S,Stil> Label<'w,E,S,Stil> where
    E: Env,
    S: 'w,
{
    #[inline]
    pub fn with_text<T>(self, text: T) -> Label<'w,E,T,Stil> where T: Caption<'w>+Statize<E>, T::Statur: Sized {
        Label{
            id: self.id,
            size: self.size,
            style: self.style,
            text,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> Label<'w,E,S,SStil> where SStil: 'w {
        Label{
            id: self.id,
            size: self.size,
            style,
            text: self.text,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,S,Stil> Statize<E> for Label<'w,E,S,Stil> where
    E: Env,
    S: StatizeSized<E>,
    Stil: StatizeSized<E>+'w,
{
    type Statur = Label<'static,E,S::StaturSized,Stil::StaturSized>;
}
