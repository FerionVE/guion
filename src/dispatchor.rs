use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::PathResolvusDyn;
use crate::widget::Widget;
use crate::widget::as_widget::AsWidget;
use crate::widget::as_widgets::AsWidgets;
use crate::widget::dyn_tunnel::WidgetDyn;

pub trait AsWidgetDispatch<V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w;
}

pub trait AsWidgetsDispatch<V,R,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w;
}

pub struct AsWidgetsResult<'w,'ww,V,E>
where
    'ww: 'w, V: 'ww,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub idx: usize,
    pub child_id: V::ChildID,
    pub widget: &'w V::Widget<'ww>,
}

impl<'w,'ww,'y,V,E> AsWidgetsResult<'w,'ww,&'y V,E>
where
    'ww: 'w,
    'w: 'y, 'ww: 'y,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn deref(self) -> AsWidgetsResult<'w,'ww,V,E> {
        AsWidgetsResult {
            idx: self.idx,
            child_id: self.child_id,
            widget: self.widget,
        }
    }
}

impl<'w,'ww,V,E> AsWidgetsResult<'w,'ww,V,E>
where
    'ww: 'w,
    V: AsWidgets<E> + ?Sized,
    V::ChildID: Clone,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww>) -> Option<Self> {
        Some(Self { idx, child_id, widget })
    }

    pub fn convert<W>(self) -> AsWidgetsResult<'w,'ww,W,E> where for<'a> W: AsWidgets<E,Widget<'a>=V::Widget<'a>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
        AsWidgetsResult {
            idx: self.idx,
            child_id: self.child_id.clone(),
            widget: self.widget,
        }
    }
}

pub trait AsWidgetsResolveDispatch<V,R,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'p,'pp,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'pp: 'p;
}

pub struct AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>
where
    'ww: 'w, V: 'ww,
    'pp: 'p,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub idx: usize,
    pub child_id: V::ChildID,
    pub resolvus: &'p (dyn PathResolvusDyn<E>+'pp),
    pub widget: &'w V::Widget<'ww>,
}

impl<'z,'w,'ww,'p,'pp,V,E> AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>
where
    'ww: 'w,
    'pp: 'p,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn from_some(idx: usize, child_id: V::ChildID, resolvus: &'p (dyn PathResolvusDyn<E>+'pp), widget: &'w V::Widget<'ww>) -> Option<Self> {
        Some(Self { idx, child_id, resolvus, widget })
    }

    pub fn convert<W>(self) -> AsWidgetsResolveResult<'w,'ww,'p,'pp,W,E> where for<'a> W: AsWidgets<E,Widget<'a>=V::Widget<'a>,WidgetCache=V::WidgetCache,ChildID=V::ChildID> {
        AsWidgetsResolveResult {
            idx: self.idx,
            child_id: self.child_id.clone(),
            resolvus: self.resolvus,
            widget: self.widget,
        }
    }
}

impl<'w,'ww,'p,'pp,'y,V,E> AsWidgetsResolveResult<'w,'ww,'p,'pp,&'y V,E>
where
    'ww: 'w,
    'pp: 'p,
    'w: 'y, 'ww: 'y,
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    pub fn deref(self) -> AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E> {
        AsWidgetsResolveResult {
            idx: self.idx,
            child_id: self.child_id,
            resolvus: self.resolvus,
            widget: self.widget,
        }
    }
}

pub trait AsWidgetsIndexedDispatch<V,E>
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
{
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w;
}

pub struct AsWidgetClosure<C,V,R,E>(C,PhantomData<(fn(*const V),R,E)>)
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R; //TODO find some way to do FnOnce thru reference

impl<C,V,R,E> AsWidgetClosure<C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsClosure<C,V,R,E>(C,PhantomData<(fn(*const V),R,E)>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<C,V,R,E> AsWidgetsClosure<C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where R: 'a, V: 'a, C: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsAllClosure<C,V,E>(C,PhantomData<(fn(*const V),E)>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>);

impl<C,V,E> AsWidgetsAllClosure<C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a {
        Self(c,PhantomData)
    }
}

pub struct AsWidgetsResolveClosure<C,V,R,E>(C,PhantomData<(fn(*const V),R,E)>)
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R;

impl<C,V,R,E> AsWidgetsResolveClosure<C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    pub fn new<'a>(c: C) -> Self where V: 'a, C: 'a {
        Self(c,PhantomData)
    }
}

impl<C,V,R,E> AsWidgetDispatch<V,R,E> for AsWidgetClosure<C,V,R,E> 
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w
    {
        (self.0)(widget,root,ctx)
    }
}

impl<C,V,R,E> AsWidgetsDispatch<V,R,E> for AsWidgetsClosure<C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResult<'w,'ww,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w
    {
        (self.0)(result,root,ctx)
    }
}

impl<C,V,R,E> AsWidgetsResolveDispatch<V,R,E> for AsWidgetsResolveClosure<C,V,R,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'p,'pp,'w,'ww,'r,'c,'cc> C: FnMut(Option<AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'p,'pp,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResolveResult<'w,'ww,'p,'pp,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w, 'pp: 'p
    {
        (self.0)(result,root,ctx)
    }
}

impl<C,V,E> AsWidgetsIndexedDispatch<V,E> for AsWidgetsAllClosure<C,V,E> 
where
    V: AsWidgets<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, idx: usize, child_id: V::ChildID, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
    where
        'ww: 'w
    {
        (self.0)(idx,child_id,widget,root,ctx)
    }
}

// pub struct AsWidgetsIndexedWrap<C>(pub C);

// impl<'z,C,V,E> AsWidgetsDispatch<V,(),E> for AsWidgetsIndexedWrap<C>
// where
//     C: AsWidgetsIndexedDispatch<V,E>,
//     V: AsWidgets<E> + ?Sized,
//     E: Env,
//     //for<'w,'ww,'r,'c,'cc> C: FnMut(usize,V::Bound,V::ChildID,&'w V::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>)
// {
//     #[inline]
//     fn call<'w,'ww,'r,'c,'cc>(&mut self, result: Option<AsWidgetsResult<'w,'ww,V,E>>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>)
//     where
//         'ww: 'w
//     {
//         self.0.call(result,root,ctx)
//     }
// }

// pub struct AsWidgetToSDispatchErase<'z,C,V,W,R,E>(pub usize, pub W::Bound, pub W::ChildID, pub C, pub PhantomData<(*const V,Box<R>,Box<E>)>)
// where
//     C: AsWidgetsDispatch<W,R,E>,
//     V: AsWidget<E> + ?Sized,
//     W: ?Sized,
//     for<'a> W: AsWidgets<'z,E,Widget<'a>=(dyn WidgetDyn<E> + 'a)> + 'z,
//     E: Env,
//     Self: 'z;

// impl<'z,C,V,W,R,E> AsWidgetDispatch<V,R,E> for AsWidgetToSDispatchErase<'z,C,V,W,R,E>
// where
//     C: AsWidgetsDispatch<W,R,E>,
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

impl<'z,C,V,R,E> AsWidgetDispatch<V,R,E> for AsWidgetClosureErased<'z,C,V,R,E>
where
    V: AsWidget<E> + ?Sized,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w (dyn WidgetDyn<E> + 'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R
{
    #[inline]
    fn call<'w,'ww,'r,'c,'cc>(&mut self, widget: &'w V::Widget<'ww>, root: E::RootRef<'r>, ctx: &'c mut E::Context<'cc>) -> R
    where
        'ww: 'w
    {
        (self.0)(widget.erase(),root,ctx)
    }
}
