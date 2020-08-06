//! redirective implementation for references of widgets
use super::*;

impl<'s,'l,E> Widget<'s,E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    #[inline]
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        (**self)._render(l,r)
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        (**self)._event_direct(l,e)
    }
    #[inline]
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        (**self).childs_ref()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b, force)
    }
    #[inline]
    fn focusable(&self) -> bool {
        (**self).focusable()
    }

    #[allow(deprecated)]
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    #[inline]
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        (**self).resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b, force)
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        (**self).in_parent_path(parent)
    }
    #[inline]
    fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        (**self).resolves_by(p)
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    #[inline]
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'s,E>> {
        unsafe{Some((**self).short_lt())}
    }
    #[inline]
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).child(i)
    }
    #[inline]
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        (**self).child(i)
    }
}
impl<'s,'l,E> Widget<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    #[inline]
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        (**self)._render(l,r)
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        (**self)._event_direct(l,e)
    }
    #[inline]
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        (**self).childs_ref()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b, force)
    }
    #[inline]
    fn focusable(&self) -> bool {
        (**self).focusable()
    }

    #[allow(deprecated)]
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    #[inline]
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        (**self).resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b, force)
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        (**self).in_parent_path(parent)
    }
    #[inline]
    fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        (**self).resolves_by(p)
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    #[inline]
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'s,E>> {
        unsafe{Some((**self).base().short_lt())}
    }
    #[inline]
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).child(i)
    }
    #[inline]
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        let r: &'s dyn Widget<'l,E> = (**self).base();
        r.child(i)
    }
}
impl<'s,'l,E> WidgetMut<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    #[allow(deprecated)]
    #[inline]
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 's: 'a {
        (**self).childs_mut()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'s,E>> {
        (**self).childs_mut()
    }
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        (**self)._set_invalid(v)
    }
    #[inline]
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath) -> Result<ResolvableMut<'a,E>,()> where 's: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i)
    }
    #[inline]
    fn into_resolve_mut(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'s,E>,()> {
        (**self).resolve_mut(i)
    }
    #[inline]
    fn inner_mut<'a>(&'a mut self) -> Option<&'a mut dyn WidgetMut<'s,E>> {
        unsafe{Some((**self).short_lt())}
    }
    #[inline]
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 's: 'a {
        (**self).child_mut(i)
    }
    #[inline]
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'s,E>,()> {
        (**self).child_mut(i)
    }
}
impl<'w,E> Widget<'w,E> for Box<dyn Widget<'w,E>> where E: Env {
    #[inline]
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        (**self)._render(l,r)
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        (**self)._event_direct(l,e)
    }
    #[inline]
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    #[inline]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        unsafe{Widget::into_childs(*self).short_lt()}
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b, force)
    }
    #[inline]
    fn focusable(&self) -> bool {
        (**self).focusable()
    }

    #[allow(deprecated)]
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    #[inline]
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        Widget::into_resolve(*self,i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b, force)
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        (**self).in_parent_path(parent)
    }
    #[inline]
    fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        (**self).resolves_by(p)
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    #[inline]
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'w,E>> {
        Some(&**self)
    }
    #[inline]
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).child(i)
    }
    #[inline]
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::into_child(*self,i)
    }
}
impl<'w,E> Widget<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    #[inline]
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
        (**self)._render(l,r)
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        (**self)._event_direct(l,e)
    }
    #[inline]
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    #[inline]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        unsafe{Widget::into_childs(*self).short_lt()}
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b, force)
    }
    #[inline]
    fn focusable(&self) -> bool {
        (**self).focusable()
    }

    #[allow(deprecated)]
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    #[inline]
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        Widget::into_resolve(*self,i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b, force)
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        (**self).in_parent_path(parent)
    }
    #[inline]
    fn resolves_by(&self, p: &EWPSub<E>) -> bool {
        (**self).resolves_by(p)
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        (**self)._focus_on_mouse_down()
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        (**self)._tabulate_by_tab()
    }
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    #[inline]
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'w,E>> {
        Some((**self).base())
    }
    #[inline]
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).child(i)
    }
    #[inline]
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::into_child(*self,i)
    }
}
impl<'w,E> WidgetMut<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    #[allow(deprecated)]
    #[inline]
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 'w: 'a {
        (**self).childs_mut()
    }
    #[inline]
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        WidgetMut::into_childs_mut(*self)
    }
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        (**self)._set_invalid(v)
    }
    #[inline]
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i)
    }
    #[inline]
    fn into_resolve_mut(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::into_resolve_mut(*self, i)
    }
    #[inline]
    fn inner_mut<'s>(&'s mut self) -> Option<&'s mut dyn WidgetMut<'w,E>> {
        unsafe{Some((&mut **self).short_lt())}
    }
    #[inline]
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        WidgetMut::child_mut(&mut **self,i)
    }
    #[inline]
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::into_child_mut(*self,i)
    }
}
