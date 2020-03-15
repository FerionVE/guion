use super::*;
use std::marker::PhantomData;
use crate::core::*;

pub mod toggle;
use toggle::*;
use calc::calc_bounds;

pub struct Pane<'c,T,E,M> where E: Env, M: Toggle {
    id: E::WidgetID,
    childs: Vec<T>,
    orientation: Orientation,
    p: PhantomData<&'c mut M>,
}

impl<'c,T,E> Pane<'c,T,E,TOwned> where E: Env {
    pub fn new(id: E::WidgetID, childs: Vec<T>, orientation: Orientation) -> Pane<'static,T,E,TOwned> where T: Widget<E> {
        Pane{
            id,
            childs,
            orientation,
            p: PhantomData,
        }
    }
    pub fn immediate(id: E::WidgetID, childs: Vec<T>, orientation: Orientation) -> Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E> {
        Pane{
            id,
            childs,
            orientation,
            p: PhantomData,
        }
    }
    pub fn immediate_mut(id: E::WidgetID, childs: Vec<T>, orientation: Orientation) -> Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E> {
        Pane{
            id,
            childs,
            orientation,
            p: PhantomData,
        }
    }
}

impl<T,E> Widget<E> for Pane<'static,T,E,TOwned> where T: AsWidget<E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        _trace_bounds(l,i,b,force,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
        //self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> {
        self.childs.iter()
            .map(|c| c.as_ref() )
            .collect::<Vec<_>>()
    }
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> {
        self.childs.iter_mut()
            .map(|c| c.as_mut() )
            .collect::<Vec<_>>()
    }
    fn focusable(&self) -> bool {
        false
    }
}

impl<'c,T,E> Widget<E> for Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        _trace_bounds(l,i,b,force,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> {
        panic!()
    }
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> {
        panic!()
    }
    fn focusable(&self) -> bool {
        false
    }
}

impl<'c,T,E> Widget<E> for Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        _trace_bounds(l,i,b,force,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
    }
    fn childs(&self) -> usize {
        self.childs.len()
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> {
        panic!()
    }
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> {
        panic!()
    }
    fn focusable(&self) -> bool {
        false
    }
    #[inline]
    fn border(&self, b: &mut Border) {
        *b = Border::empty();
    }
}

impl<'c,T,E> WidgetImmediate<'c,E> for Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E>, E: Env {
    fn resolve(self, s: E::WidgetPath) -> Result<Resolvable<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_box(self: Box<Self>, s: E::WidgetPath) -> Result<Resolvable<'c,E>,()> {
        todo!()
    }
    fn resolve_ref(&self, s: E::WidgetPath) -> Result<Resolvable<'c,E>,()> {
        todo!()
    }
    fn widget(&self) -> &E::DynWidget {
        todo!()
    }
    fn cloned<'s>(&'s self) -> WidgetRef<'s,E> where 'c: 's {
        todo!()
    }
}

impl<'c,T,E> WidgetImmediateMut<'c,E> for Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E>, E: Env {
    fn resolve(self, s: E::WidgetPath) -> Result<Resolvable<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_box(self: Box<Self>, s: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'c,E>,()> {
        todo!()
    }
    fn resolve_mut(self, s: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_mut_box(self: Box<Self>, s: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'c,E>,()> {
        todo!()
    }
    fn widget(&self) -> &E::DynWidget {
        todo!()
    }
    fn widget_mut(&mut self) -> &mut E::DynWidget {
        todo!()
    }
    fn cloned_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'c: 's {
        todo!()
    }
}

pub fn _render<E>(mut l: Link<E>, r: &mut RenderLink<E>, o: Orientation) -> bool where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&r.b.size,&sizes,o); 
    
    let mut validate = true;
    let mut i = 0usize;

    l.for_childs(|c| {
        let mut r = r.slice(&bounds[i]);
        validate &= r.render_widget(c);
        i+=1;
    }).expect("Dead Path inside Pane");

    false
}

pub fn _event<E>(mut l: Link<E>, e: (EEvent<E>,&Bounds,u64), o: Orientation) where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&e.1.size,&sizes,o); 

    let mut i = 0usize;

    l.for_childs(|mut c| {
        let sliced = e.1.slice(&bounds[i]);
        if let Some(ee) = e.0.filter_cloned(&sliced) {
            c.event((ee,&sliced,e.2));
        }
        i+=1;
    }).expect("Dead Path inside Pane");
}

pub fn _size<E>(mut l: Link<E>, o: Orientation) -> ESize<E> where
    E: Env,
{
    let mut s = ESize::<E>::empty();
    l.for_childs(&mut |mut l: Link<E>| s.add(&l.size(), o) ).expect("Dead Path inside Pane");
    s
}

pub fn _trace_bounds<E>(mut l: Link<E>, i: usize, b: &Bounds, force: bool, o: Orientation) -> Result<Bounds,()> where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&b.size,&sizes,o); 

    bounds.get(i).map(|w| *w).ok_or(())
}