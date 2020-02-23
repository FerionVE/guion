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

impl<T,E> Widget<E> for Pane<'static,T,E,TOwned> where T: Widget<E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        _render(l,r,self.orientation)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
        //self.invalid
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
        //self.invalid = true
    }
    fn has_childs(&self) -> bool {
        true
    }
    fn _childs<'a>(&'a self) -> Vec<WidgetRef<'a,E>> {
        self.childs.iter()
            .map(|c| c.as_immediate() )
            .collect::<Vec<_>>()
    }
    fn _childs_mut<'a>(&'a mut self) -> Vec<WidgetRefMut<'a,E>> {
        self.childs.iter_mut()
            .map(|c| c.as_immediate_mut() )
            .collect::<Vec<_>>()
    }
    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath> {
        self.childs.iter()
            .map(|c| c.self_in_parent(own_path) )
            .collect::<Vec<_>>()
    }
    fn selectable(&self) -> bool {
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
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
    }
    fn has_childs(&self) -> bool {
        true
    }
    fn _childs<'a>(&'a self) -> Vec<WidgetRef<'a,E>> {
        panic!()
    }
    fn _childs_mut<'a>(&'a mut self) -> Vec<WidgetRefMut<'a,E>> {
        panic!()
    }
    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath> {
        self.childs.iter()
            .map(|c| c.widget().self_in_parent(own_path) )
            .collect::<Vec<_>>()
    }
    fn selectable(&self) -> bool {
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
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        _event(l,e,self.orientation)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        _size(l,self.orientation)
    }
    fn invalid(&self) -> bool {
        true
    }
    fn set_invalid(&mut self, v: bool) {
        let _ = v;
    }
    fn has_childs(&self) -> bool {
        true
    }
    fn _childs<'a>(&'a self) -> Vec<WidgetRef<'a,E>> {
        panic!()
    }
    fn _childs_mut<'a>(&'a mut self) -> Vec<WidgetRefMut<'a,E>> {
        panic!()
    }
    fn child_paths(&self, own_path: WPSlice<E>) -> Vec<E::WidgetPath> {
        self.childs.iter()
            .map(|c| c.widget().self_in_parent(own_path) )
            .collect::<Vec<_>>()
    }
    fn selectable(&self) -> bool {
        false
    }
    #[inline]
    fn border(&self, b: &mut Border) {
        *b = Border::empty();
    }
}

impl<'c,T,E> WidgetImmediate<'c,E> for Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E>, E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'c,E>,()> {
        todo!()
    }
    fn resolve_ref(&self, s: WPSlice<E>) -> Result<Resolvable<'c,E>,()> {
        todo!()
    }
    fn widget(&self) -> &E::DynWidget {
        todo!()
    }
    fn cloned(&self) -> WidgetRef<E> {
        todo!()
    }
}

impl<'c,T,E> WidgetImmediateMut<'c,E> for Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E>, E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'c,E>,()> {
        todo!()
    }
    fn resolve_mut(self, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_mut_box(self: Box<Self>, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'c,E>,()> {
        todo!()
    }
    fn widget(&self) -> &E::DynWidget {
        todo!()
    }
    fn widget_mut(&mut self) -> &mut E::DynWidget {
        todo!()
    }
    fn cloned(&mut self) -> WidgetRefMut<E> {
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

    l.for_childs(|mut c| {
        let mut r = r.slice(&bounds[i]);
        validate &= r.render_widget(c);
        i+=1;
    }).expect("Dead Path inside Pane");

    false
}

pub fn _event<E>(mut l: Link<E>, e: (EEvent<E>,&Bounds), o: Orientation) where
    E: Env,
{
    let sizes = l.child_sizes().expect("Dead Path Inside Pane");
    let bounds = calc_bounds(&e.1.size,&sizes,o); 

    let mut i = 0usize;

    l.for_childs(|mut c| {
        let sliced = e.1.slice(&bounds[i]);
        if let Some(e) = e.0.filter_cloned(&sliced) {
            c.event((e,&sliced));
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