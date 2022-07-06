use std::marker::PhantomData;

use crate::env::Env;
use crate::view::View;
use crate::widget::as_widget::AsWidget;
use crate::widget::as_widgets::AsWidgets;


pub trait AsWidgetDispatch<'z,V,E>
where
    V: AsWidget<'z,E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(self, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww;
}

pub trait ViewDispatch<'z,V,MutFn,E>
where
    V: View<'z,E> + ?Sized,
    E: Env, MutFn: 'static,
{
    fn call<'w,'ww,'r,'c,'cc>(self, widget: &'w V::Viewed<'ww,MutFn>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww;
}

pub trait AsWidgetsDispatch<'z,V,E>
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww;
}

pub trait AsWidgetsIndexedDispatch<'z,V,E>
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww;
}

pub struct CallbackClosure<'z,C,V,M,E>(C,PhantomData<(Box<V>,M,E,&'z ())>)
where
    V: 'z + ?Sized,
    E: Env;


impl<'z,C,V,E> CallbackClosure<'z,C,V,(),E> 
where
    V: AsWidget<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn for_as_widget(c: C) -> Self {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,MutFn,E> CallbackClosure<'z,C,V,MutFn,E> 
where
    V: View<'z,E> + ?Sized,
    MutFn: 'static,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(&'w V::Viewed<'ww,MutFn>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn for_view(c: C) -> Self {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,E> CallbackClosure<'z,C,V,(),E> 
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(usize,V::Bound,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn for_as_widgets(c: C) -> Self {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,E> CallbackClosure<'z,C,V,(),E> 
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn for_as_widgets_all(c: C) -> Self {
        Self(c,PhantomData)
    }
}

impl<'z,C,V,M,E> AsWidgetDispatch<'z,V,E> for CallbackClosure<'z,C,V,M,E> 
where
    V: AsWidget<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(self, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(widget,root,ctx)
    }
}

impl<'z,C,V,MutFn,E> ViewDispatch<'z,V,MutFn,E> for CallbackClosure<'z,C,V,MutFn,E> 
where
    V: View<'z,E> + ?Sized,
    MutFn: 'static,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(&'w V::Viewed<'ww,MutFn>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(self, widget: &'w V::Viewed<'ww,MutFn>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where 'ww: 'w, 'z: 'ww
    {
        (self.0)(widget,root,ctx)
    }
}

impl<'z,C,V,M,E> AsWidgetsDispatch<'z,V,E> for CallbackClosure<'z,C,V,M,E> 
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnOnce(usize,V::Bound,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(idx,bound,child_id,widget,root,ctx)
    }
}

impl<'z,C,V,M,E> AsWidgetsIndexedDispatch<'z,V,E> for CallbackClosure<'z,C,V,M,E> 
where
    V: AsWidgets<'z,E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, bound: V::Bound, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(idx,bound,child_id,widget,root,ctx)
    }
}
