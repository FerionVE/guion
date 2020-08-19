use super::*;
use std::marker::PhantomData;

pub mod widget;

pub struct ProgressBar<'w,E,Stil> where 
    E: Env,
    Stil: 'w,
{
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Stil,
    pub value: f32,
    pub orientation: Orientation,
    p: PhantomData<&'w mut &'w ()>,
}

impl<'w,E> ProgressBar<'w,E,()> where 
    E: Env,
{
    #[inline]
    pub fn new(id: E::WidgetID, o: Orientation) -> Self {
        Self {
            id,
            size: Size::empty().into(),
            style: (),
            value: 0.0,
            orientation: o,
            p: PhantomData,
        }
    }
}

impl<'w,E,Stil> ProgressBar<'w,E,Stil> where 
    E: Env,
    Stil: 'w,
{
    #[inline]
    pub fn with_value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }

    #[inline]
    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
    #[inline]
    pub fn with_style<SStil>(self, style: SStil) -> ProgressBar<'w,E,SStil> where SStil: 'w {
        ProgressBar{
            id: self.id,
            value: self.value,
            orientation: self.orientation,
            size: self.size,
            style,
            p: PhantomData,
        }
    }
}

unsafe impl<'w,E,Stil> Statize<E> for ProgressBar<'w,E,Stil> where E: Env, Stil: StatizeSized<E>+'w, {
    type Statur = ProgressBar<'static,E,Stil::StaturSized>;
}
