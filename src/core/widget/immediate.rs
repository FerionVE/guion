use std::ops::DerefMut;
use super::*;

pub trait WidgetImmediate<'d,E>: Widget<E> where E: Env {
    fn resolve(self, s: EWPSlice<E>) -> Result<WidgetRef<'d,E>,()>;
}
pub trait WidgetImmediateMut<'d,E>: WidgetImmediate<'d,E> where E: Env {
    fn resolve_mut(self, s: EWPSlice<E>) -> Result<WidgetRefMut<'d,E>,()>;
}

pub type WidgetRef<'a,E: Env> = Box<dyn Deref<Target=E::DynWidget>+'a>;
pub type WidgetRefMut<'a,E: Env> = Box<dyn DerefMut<Target=E::DynWidget>+'a>;