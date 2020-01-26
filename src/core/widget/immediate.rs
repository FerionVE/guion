use std::rc::Rc;
use std::ops::DerefMut;
use super::*;

pub trait WidgetImmediate<'d,E>: Widget<E> where E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<WidgetRef<'d,E>,()>;
}
pub trait WidgetImmediateMut<'d,E>: WidgetImmediate<'d,E> where E: Env {
    fn resolve_mut(self, s: EWPSlice<E>) -> Result<WidgetRefMut<'d,E>,()>;
}

#[allow(type_alias_bounds)]
pub type WidgetRef<'a,E: Env> = Rc<dyn Deref<Target=E::DynWidget>+'a>;
#[allow(type_alias_bounds)]
pub type WidgetRefMut<'a,E: Env> = Box<dyn DerefMut<Target=E::DynWidget>+'a>;

pub trait ResolveRaw<E> where E: Env {
    fn resolve<'a>(&'a self, s: EWPSlice<E>) -> Result<Resolvable<'a,E>,()>;
    fn resolve_mut<'a>(&'a mut self, s: EWPSlice<E>) -> Result<WidgetRefMut<'a,E>,()>;
}