use std::ops::DerefMut;
use std::ops::Deref;
use super::*;

/// type erased widget, reference and owned.
pub trait DynWidget<E>: Widget<E> where E: Env + 'static {
    type Owned: Deref<Target=Self> + DerefMut<Target=Self> + Widget<E>;
    //type Ref: Widget<E>;
    //type RefMut: Widget<E>;

    #[inline]
    fn is<T: Any>(&self) -> bool {
        self.as_any().is::<T>() || self.as_any_inner().is::<T>()
    }
    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
        .or(
            self.as_any_inner().downcast_ref()
        )
    }
    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.as_any().is::<T>() {
            self.as_any_mut().downcast_mut()
        }else{
            self.as_any_inner_mut().downcast_mut()
        }
    }
    fn erase<T: Widget<E>>(w: &T) -> &Self;
    fn erase_mut<T: Widget<E>>(w: &mut T) -> &mut Self;
    fn erase_move<T: Widget<E>>(w: T) -> Self::Owned;
    //fn downcast_into<T: Widget<E>>(e: Self::Owned) -> Result<T,Self::Owned>;
}

impl<E> DynWidget<E> for dyn Widget<E> where E: Env + 'static {
    type Owned = Box<dyn Widget<E>>;

    fn erase<T: Widget<E>>(w: &T) -> &Self {w}
    fn erase_mut<T: Widget<E>>(w: &mut T) -> &mut Self {w}
    fn erase_move<T: Widget<E>>(w: T) -> Self::Owned {Box::new(w)}
    //fn downcast_into<T: Widget<E>>(e: Self::Owned) -> Result<T,Self::Owned> {Box::downcast(e)}
}
pub trait WidgetAsAny<E>: 'static where E: Env {
    fn _as_any(&self) -> &dyn Any;
    fn _as_any_mut(&mut self) -> &mut dyn Any;
    fn _erase(&self) -> &E::DynWidget;
    fn _erase_mut(&mut self) -> &mut E::DynWidget;
    fn _as_immediate(&self) -> WidgetRef<E>;
    fn _as_immediate_mut(&mut self) -> WidgetRefMut<E>;
}

impl<T,E> WidgetAsAny<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn _as_any(&self) -> &dyn Any {self}
    #[inline]
    fn _as_any_mut(&mut self) -> &mut dyn Any {self}
    #[inline]
    fn _erase(&self) -> &E::DynWidget {
        DynWidget::erase(self)
    }
    #[inline]
    fn _erase_mut(&mut self) -> &mut E::DynWidget {
        DynWidget::erase_mut(self)
    }
    #[inline]
    fn _as_immediate<'a>(&'a self) -> WidgetRef<'a,E> {
        Box::new(self)
    }
    #[inline]
    fn _as_immediate_mut<'a>(&'a mut self) -> WidgetRefMut<'a,E> {
        Box::new(self)
    }
}

impl<E> Widget<E> for Box<dyn Widget<E>> where E: Env {
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        (**self).render(l,r)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        (**self).event(l,e)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        (**self).size(l)
    }
    fn childs(&self) -> usize {
        (**self).childs()
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> {
        (**self).childs_ref()
    }
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> {
        (**self).childs_mut()
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        (**self).invalid()
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn erase(&self) -> &E::DynWidget {
        (**self).erase()
    }
    fn erase_mut(&mut self) -> &mut E::DynWidget {
        (**self).erase_mut()
    }
    fn erase_move(self) -> EDynOwned<E> where Self: Sized {
        todo!()
    }
    fn as_immediate(&self) -> WidgetRef<E> {
        (**self).as_immediate()
    }
    fn as_immediate_mut(&mut self) -> WidgetRefMut<E> {
        (**self).as_immediate_mut()
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> {
        (**self).resolve_mut(i, invalidate)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> {
        (**self).resolve(i)
    }
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b, force)
    }
    fn self_in_parent(&self, parent: E::WidgetPath) -> E::WidgetPath {
        (**self).self_in_parent(parent)
    }
    fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        (**self).is_subpath(p)
    }
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    fn style(&self, s: &mut ESVariant<E>) {
        (**self).style(s)
    }
    fn border(&self, b: &mut Border) {
        (**self).border(b)
    }
    fn as_any(&self) -> &dyn Any {
        (**self).as_any()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_mut()
    }
    fn as_any_inner(&self) -> &dyn Any {
        (**self).as_any_inner()
    }
    fn as_any_inner_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_inner_mut()
    }
    
}