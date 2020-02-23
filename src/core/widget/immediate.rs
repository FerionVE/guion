use super::*;

/// a Widget implementor which references to the widget tree
pub trait WidgetImmediate<'d,E> where E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> where Self: Sized;
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()>;
    fn resolve_ref(&self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()>;
    fn widget(&self) -> &E::DynWidget;
    fn cloned<'s>(&'s self) -> WidgetRef<'s,E> where 'd: 's;
}
pub trait WidgetImmediateMut<'d,E> where E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> where Self: Sized;
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()>;
    fn resolve_mut(self, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'d,E>,()> where Self: Sized;
    fn resolve_mut_box(self: Box<Self>, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'d,E>,()>;
    fn widget(&self) -> &E::DynWidget;
    fn widget_mut(&mut self) -> &mut E::DynWidget;
    fn cloned_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'd: 's;
}

impl<'d,T,E> WidgetImmediate<'d,E> for &'d T where T: Widget<E>, E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(self,s)
    }
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(*self,s)
    }
    fn resolve_ref(&self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(*self,s)
    }
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
    fn cloned<'s>(&'s self) -> WidgetRef<'s,E> where 'd: 's {
        Box::new(*self)
    }
}
impl<'d,T,E> WidgetImmediateMut<'d,E> for &'d mut T where T: Widget<E>, E: Env {
    fn resolve(self, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(self,s)
    }
    fn resolve_box(self: Box<Self>, s: WPSlice<E>) -> Result<Resolvable<'d,E>,()> {
        <T as Widget<E>>::resolve(*self,s)
    }
    fn resolve_mut(self, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'d,E>,()> {
        <T as Widget<E>>::resolve_mut(self,s,invalidate)
    }
    fn resolve_mut_box(self: Box<Self>, s: WPSlice<E>, invalidate: bool) -> Result<WidgetRefMut<'d,E>,()> {
        <T as Widget<E>>::resolve_mut(*self,s,invalidate)
    }
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
    fn widget_mut(&mut self) -> &mut E::DynWidget {
        self.erase_mut()
    }
    fn cloned_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> where 'd: 's {
        Box::new(&mut **self)
    }
}

/*pub trait AsDynWidgetRef<E> where E: Env {
    fn widget(&self) -> &E::DynWidget;
}
pub trait AsDynWidgetRefMut<E> where E: Env {
    fn widget(&self) -> &E::DynWidget;
    fn widget_mut(&mut self) -> &mut E::DynWidget;
}

impl<E,T> AsDynWidgetRef<E> for T where T: Widget<E>, E: Env {
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
}
impl<E,T> AsDynWidgetRefMut<E> for T where T: Widget<E>, E: Env {
    fn widget(&self) -> &E::DynWidget {
        self.erase()
    }
    fn widget_mut(&mut self) -> &mut E::DynWidget {
        self.erase_mut()
    }
}*/

#[allow(type_alias_bounds)]
pub type WidgetRef<'a,E: Env> = Box<dyn WidgetImmediate<'a,E>+'a>;
#[allow(type_alias_bounds)]
pub type WidgetRefMut<'a,E: Env> = Box<dyn WidgetImmediateMut<'a,E>+'a>;