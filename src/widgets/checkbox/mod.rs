use super::*;
use crate::core::event::key::Key;
use std::marker::PhantomData;
use util::{state::*, caption::Caption};

pub mod imp;
pub mod trayt;

pub struct CheckBox<'w,E,State,Text> where
    E: Env,
    State: 'w,
    Text: 'w,
{
    pub trigger: for<'a> fn(Link<'a,E>,bool),
    id: E::WidgetID,
    pub size: ESize<E>,
    pub style: Vec<StdVerb>,
    pub locked: bool,
    //pressed: Option<EEKey<E>>,
    pub border: Option<Border>,
    pub text: Text,
    pub state: State,
    p: PhantomData<&'w mut ()>,
}

impl<'w,State,E> CheckBox<'w,E,State,&'static str> where
    E: Env,
{
    pub fn new(id: E::WidgetID, state: State) -> Self {
        Self{
            id,
            size: ESize::<E>::empty(),
            style: vec![],
            trigger: |_,_|{},
            locked: false,
            border: None,
            text: "",
            state,
            p: PhantomData,
        }
    }
}

impl<'w,E,State,Text> CheckBox<'w,E,State,Text> where
    E: Env,
    Text: 'w,
{
    

    pub fn with_trigger(mut self, fun: for<'a> fn(Link<E>,bool)) -> Self {
        self.trigger = fun;
        self
    }
    pub fn with_text<T>(self, text: T) -> CheckBox<'w,E,State,T> where T: Caption<'w>+Statize, T::Statur: Sized {
        CheckBox{
            id: self.id,
            size: self.size,
            style: self.style,
            trigger: self.trigger,
            locked: self.locked,
            border: self.border,
            text,
            state: self.state,
            p: PhantomData,
        }
    }

    pub fn with_size(mut self, s: ESize<E>) -> Self {
        self.size = s;
        self
    }
}

unsafe impl<'w,E,State,Text> Statize for CheckBox<'w,E,State,Text> where
    E: Env,
    State: Statize+'w,
    State::Statur: Sized,
    Text: Statize+'w,
    Text::Statur: Sized,
{
    type Statur = CheckBox<'static,E,State::Statur,Text::Statur>;
}

fn compile_test<E>(id: E::WidgetID) -> WidgetRefMut<'static,E> where
    E: Env,
    ERenderer<E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    ESVariant<E>: StyleVariantSupport<StdVerb>,
    E::Context: AsHandlerStateful<E>,
{
    let b: CheckBox<'static,E,bool,&'static str> = CheckBox::new(id, false);
    //b.into()
    eprintln!("{}",b.childs());
    let mut b = true;
    let c = AtomState::get(&b);
    AtomStateMut::set(&mut b, !c); // Discovery: `AtomState::set` would actually introduce ICE
    eprintln!("{}", <&'static str as Caption>::caption(&"AKW"));
    eprintln!("{:?}",std::any::TypeId::of::< <&'static str as Statize>::Statur >());
    eprintln!("{:?}",std::any::TypeId::of::< <bool as Statize>::Statur >());
    eprintln!("{:?}",std::any::TypeId::of::< <bool as Mutize<bool>>::Mutur >());
    eprintln!("{:?}",std::any::TypeId::of::< <<bool as Mutize<bool>>::Mutur as Statize>::Statur >());
    todo!()
}