use super::*;
use std::marker::PhantomData;
use crate::core::*;

pub mod toggle;
use toggle::*;

pub struct Pane<'c,T,E,M> where E: Env, M: Toggle {
    childs: Vec<T>,
    orientation: bool,
    p: PhantomData<&'c mut (E,M)>,
}

impl<'c,T,E> Pane<'c,T,E,TOwned> where E: Env {
    pub fn new(childs: Vec<T>, orientation: bool) -> Pane<'static,T,E,TOwned> where T: Widget<E> {
        Pane{
            childs,
            orientation,
            p: PhantomData,
        }
    }
    pub fn immediate(childs: Vec<T>, orientation: bool) -> Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E> {
        Pane{
            childs,
            orientation,
            p: PhantomData,
        }
    }
    pub fn immediate_mut(childs: Vec<T>, orientation: bool) -> Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E> {
        Pane{
            childs,
            orientation,
            p: PhantomData,
        }
    }
}

impl<T,E> Widget<E> for Pane<'static,T,E,TOwned> where T: Widget<E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        todo!()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        todo!()
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        todo!()
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        todo!()
    }
    fn invalid(&self) -> Option<u32> {
        todo!()
    }
    fn set_invalid(&mut self, v: Option<u32>) {
        todo!()
    }
    fn has_childs(&self) -> bool {
        todo!()
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
        todo!()
    }
}

impl<'c,T,E> Widget<E> for Pane<'c,T,E,TRef> where T: WidgetImmediate<'c,E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        todo!()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        todo!()
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        todo!()
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        todo!()
    }
    fn invalid(&self) -> Option<u32> {
        todo!()
    }
    fn set_invalid(&mut self, v: Option<u32>) {
        todo!()
    }
    fn has_childs(&self) -> bool {
        todo!()
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
        todo!()
    }
}

impl<'c,T,E> Widget<E> for Pane<'c,T,E,TMut> where T: WidgetImmediateMut<'c,E>, E: Env, Self: 'static {
    fn id(&self) -> E::WidgetID {
        todo!()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        todo!()
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds)) {
        todo!()
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        todo!()
    }
    fn invalid(&self) -> Option<u32> {
        todo!()
    }
    fn set_invalid(&mut self, v: Option<u32>) {
        todo!()
    }
    fn has_childs(&self) -> bool {
        todo!()
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
        todo!()
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
    fn resolve_mut(self, s: WPSlice<E>) -> Result<WidgetRefMut<'c,E>,()> where Self: Sized {
        todo!()
    }
    fn resolve_mut_box(self: Box<Self>, s: WPSlice<E>) -> Result<WidgetRefMut<'c,E>,()> {
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