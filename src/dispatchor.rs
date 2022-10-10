use std::marker::PhantomData;

use crate::env::Env;
use crate::view::View;
use crate::widget::Widget;
use crate::widget::as_widget::AsWidget;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::dyn_tunnel::WidgetDyn;


pub trait AsWidgetDispatch<'z,V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub trait AsWidgetsDispatch<'z,V,R,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub trait AsWidgetsIndexedDispatch<'z,V,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub struct AsWidgetClosure<'z,C,V,R,E>(C,PhantomData<(Box<V>,R,E,&'z ())>)
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R; //TODO find some way to do FnOnce thru reference

impl<'z,C,V,R,E> AsWidgetClosure<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsClosure<'z,C,V,R,E>(C,PhantomData<(Box<V>,R,E,&'z ())>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<'z,C,V,R,E> AsWidgetsClosure<'z,C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsAllClosure<'z,C,V,E>(C,PhantomData<(Box<V>,E,&'z ())>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>);

impl<'z,C,V,E> AsWidgetsAllClosure<'z,C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetClosure<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(widget,root,ctx)
    }
}

impl<'z,C,V,R,E> AsWidgetsDispatch<'z,V,R,E> for AsWidgetsClosure<'z,C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(idx,bound,child_id,widget,root,ctx)
    }
}

impl<'z,C,V,E> AsWidgetsIndexedDispatch<'z,V,E> for AsWidgetsAllClosure<'z,C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(idx,bound,child_id,widget,root,ctx)
    }
}

pub struct AsWidgetsIndexedWrap<C>(pub C);

impl<'z,C,V,E> AsWidgetsDispatch<'z,V,(),E> for AsWidgetsIndexedWrap<C>
where
    C: AsWidgetsIndexedDispatch<'z,V,E>,
    V: AsWidgets<E> + ?Sized,
    E: Env,
    //for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        self.0.call(idx,bound,child_id,widget,root,ctx)
    }
}

// pub struct AsWidgetToSDispatchErase<'z,C,V,W,R,E>(pub usize, pub W::Bound, pub W::ChildID, pub C, pub PhantomData<(Box<V>,Box<R>,Box<E>)>)
// where
//     C: AsWidgetsDispatch<'z,W,R,E>,
//     V: AsWidget<E> + ?Sized,
//     W: ?Sized,
//     for<'a> W: AsWidgets<'z,E,Widget<'a>=(dyn WidgetDyn<E> + 'a)> + 'z,
//     E: Env,
//     Self: 'z;

// impl<'z,C,V,W,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetToSDispatchErase<'z,C,V,W,R,E>
// where
//     C: AsWidgetsDispatch<'z,W,R,E>,
//     V: AsWidget<E> + ?Sized,
//     W: ?Sized,
//     for<'a> W: AsWidgets<'z,E,Widget<'a>=(dyn WidgetDyn<E> + 'a)> + 'z,
//     E: Env,
//     Self: 'z
// {

// }

pub struct AsWidgetClosureErased<'z,C,V,R,E>(C,PhantomData<(Box<V>,R,E,&'z ())>)
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<'z,C,V,R,E> AsWidgetClosureErased<'z,C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,R,E> AsWidgetDispatch<'z,V,R,E> for AsWidgetClosureErased<'z,C,V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(widget.erase(),root,ctx)
    }
}
