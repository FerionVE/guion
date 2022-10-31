use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
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
    fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'z,'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub struct AsWidgetsResult<'z,'w,'ww,V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub idx: usize,
    pub child_id: V::ChildID,
    pub widget: &'w V::Widget<'ww,'z>,
}

impl<'z,'w,'ww,'y,V,E> AsWidgetsResult<'z,'w,'ww,&'y V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    'w: 'y, 'ww: 'y, 'z: 'y,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn deref(self) -> AsWidgetsResult<'z,'w,'ww,V,E> {
        AsWidgetsResult {
            idx: self.idx,
            child_id: self.child_id,
            widget: self.widget,
        }
    }
}

impl<'z,'w,'ww,V,E> AsWidgetsResult<'z,'w,'ww,V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    V: AsWidgets<E> + ?Sized,
    V::ChildID: Clone,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>) -> Option<Self> {
        Some(Self { idx, child_id, widget })
    }

    pub fn convert<W>(self) -> AsWidgetsResult<'z,'w,'ww,W,E> where W: 'z, for<'a,'b> W: AsWidgets<E,Widget<'a,'b>=V::Widget<'a,'b>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
        AsWidgetsResult {
            idx: self.idx,
            child_id: self.child_id.clone(),
            widget: self.widget,
        }
    }
}

pub trait AsWidgetsResolveDispatch<'z,V,R,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'p,'pp,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, 'pp: 'p, 'z: 'pp, V: 'z;
}

pub struct AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    'pp: 'p, 'z: 'pp, V: 'p,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub idx: usize,
    pub child_id: V::ChildID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'pp),
    pub widget: &'w V::Widget<'ww,'z>,
}

impl<'z,'w,'ww,'p,'pp,V,E> AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    'pp: 'p, 'z: 'pp, V: 'p,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: V::ChildID, resolvus: &'p (dyn PathResolvusDyn<E>+'pp), widget: &'w V::Widget<'ww,'z>) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget })
    }

    pub fn convert<W>(self) -> AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,W,E> where W: 'z, W: 'p, for<'a,'b> W: AsWidgets<E,Widget<'a,'b>=V::Widget<'a,'b>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
        AsWidgetsResolveResult {
            idx: self.idx,
            child_id: self.child_id.clone(),
            resolvus: self.resolvus,
            widget: self.widget,
        }
    }
}

impl<'z,'w,'ww,'p,'pp,'y,V,E> AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,&'y V,E>
where
    'ww: 'w, 'z: 'ww, V: 'z,
    'pp: 'p, 'z: 'pp, V: 'p,
    'w: 'y, 'ww: 'y, 'z: 'y,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn deref(self) -> AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E> {
        AsWidgetsResolveResult {
            idx: self.idx,
            child_id: self.child_id,
            resolvus: self.resolvus,
            widget: self.widget,
        }
    }
}

pub trait AsWidgetsIndexedDispatch<'z,V,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww, V: 'z;
}

pub struct AsWidgetClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
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

pub struct AsWidgetsClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<'z,C,V,R,E> AsWidgetsClosure<'z,C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsAllClosure<'z,C,V,E>(C,PhantomData<(fn(*const V),E,&'z ())>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>);

impl<'z,C,V,E> AsWidgetsAllClosure<'z,C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a, 'z: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsResolveClosure<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<'z,C,V,R,E> AsWidgetsResolveClosure<'z,C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
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
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'z,'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'z,'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(result,root,ctx)
    }
}

impl<'z,C,V,R,E> AsWidgetsResolveDispatch<'z,V,R,E> for AsWidgetsResolveClosure<'z,C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'p,'pp,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResolveResult<'z,'w,'ww,'p,'pp,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'z: 'ww, 'pp: 'p, 'z: 'pp, V: 'z
    {
        (self.0)(result,root,ctx)
    }
}

impl<'z,C,V,E> AsWidgetsIndexedDispatch<'z,V,E> for AsWidgetsAllClosure<'z,C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww,'z>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w, 'z: 'ww
    {
        (self.0)(idx,child_id,widget,root,ctx)
    }
}

// pub struct AsWidgetsIndexedWrap<C>(pub C);

// impl<'z,C,V,E> AsWidgetsDispatch<'z,V,(),E> for AsWidgetsIndexedWrap<C>
// where
//     C: AsWidgetsIndexedDispatch<'z,V,E>,
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     //for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>)
// {
//     #[inline]
//     fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'z,'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
//     where
//         'ww: 'w, 'z: 'ww
//     {
//         self.0.call(result,root,ctx)
//     }
// }

// pub struct AsWidgetToSDispatchErase<'z,C,V,W,R,E>(pub usize, pub W::Bound, pub W::ChildID, pub C, pub PhantomData<(*const V,Box<R>,Box<E>)>)
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

pub struct AsWidgetClosureErased<'z,C,V,R,E>(C,PhantomData<(fn(*const V),R,E,&'z ())>)
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
