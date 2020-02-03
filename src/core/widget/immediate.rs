use std::rc::Rc;
use std::ops::DerefMut;
use super::*;

/// a Widget implementor which references to the widget tree
pub trait WidgetImmediate<'d,E> where E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> where Self: Sized;
    fn resolve_box(self: Box<Self>, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        todo!()
        //(*self).resolve(s)
    }
    fn resolve_ref(&self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()>;
    fn widget(&self) -> &E::DynWidget;
}
pub trait WidgetImmediateMut<'d,E> where E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> where Self: Sized;
    fn resolve_box(self: Box<Self>, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        todo!()
        //(*self).resolve(s)
    }
    fn resolve_mut(self, s: EWPSlice<E>) -> Result<WidgetRefMut<'d,E>,()> where Self: Sized;
    fn resolve_mut_box(self: Box<Self>, s: EWPSlice<E>) -> Result<WidgetRefMut<'d,E>,()> {
        todo!()
        //(*self).resolve_mut(s)
    }
    fn widget(&self) -> &E::DynWidget;
    fn widget_mut(&mut self) -> &mut E::DynWidget;
}

impl<'d,T,E> WidgetImmediate<'d,E> for &'d T where T: Widget<E>, E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(self,s)
    }
    fn resolve_ref(&self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(*self,s)
    }
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
}
impl<'d,T,E> WidgetImmediateMut<'d,E> for &'d mut T where T: Widget<E>, E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(self,s)
    }
    fn resolve_mut(self, s: EWPSlice<E>) -> Result<WidgetRefMut<'d,E>,()> {
        <T as Widget<E>>::resolve_mut(self,s)
    }
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
    fn widget_mut(&mut self) -> &mut E::DynWidget {
        self.erase_mut()
    }
}

#[allow(type_alias_bounds)]
pub type WidgetRef<'a,E: Env> = Box<dyn WidgetImmediate<'a,E>+'a>;
#[allow(type_alias_bounds)]
pub type WidgetRefMut<'a,E: Env> = Box<dyn WidgetImmediateMut<'a,E>+'a>;