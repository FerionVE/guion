//! redirective implementation for references of widgets
use super::*;

impl<'s,'l,E> Widget<'s,E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        (**self)._render(l,r)
    }
    fn _event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        (**self)._event(l,e)
    }
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        (**self).childs_ref()
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
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
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
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'s,E>> {
        Some((**self).short_lt())
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        (**self).child(i)
    }
}
impl<'s,'l,E> Widget<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        (**self)._render(l,r)
    }
    fn _event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        (**self)._event(l,e)
    }
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        (**self).childs_ref()
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
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
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
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'s,E>> {
        Some((**self).base().short_lt())
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        let r: &'s dyn Widget<'l,E> = (**self).base();
        r.child(i)
    }
}
impl<'s,'l,E> WidgetMut<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    #[allow(deprecated)]
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 's: 'a {
        (**self).childs_mut()
    }
    #[allow(deprecated)]
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'s,E>> {
        (**self).childs_mut()
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 's: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn into_resolve_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'s,E>,()> {
        (**self).resolve_mut(i, invalidate)
    }
    fn inner_mut<'a>(&'a mut self) -> Option<&'a mut dyn WidgetMut<'s,E>> {
        Some((**self).short_lt())
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 's: 'a {
        (**self).child_mut(i)
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'s,E>,()> {
        (**self).child_mut(i)
    }
}
impl<'w,E> Widget<'w,E> for Box<dyn Widget<'w,E>> where E: Env {
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        (**self)._render(l,r)
    }
    fn _event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        (**self)._event(l,e)
    }
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        Widget::into_childs(*self).short_lt()
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
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        Widget::into_resolve(*self,i)
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
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'w,E>> {
        Some(&**self)
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::into_child(*self,i)
    }
}
impl<'w,E> Widget<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    fn id(&self) -> E::WidgetID {
        (**self).id()
    }
    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        (**self)._render(l,r)
    }
    fn _event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        (**self)._event(l,e)
    }
    fn _size(&self, l: Link<E>) -> ESize<E> {
        (**self)._size(l)
    }
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    fn into_childs(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        Widget::into_childs(*self).short_lt()
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
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    fn into_resolve(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        Widget::into_resolve(*self,i)
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
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        (**self).debug_type_name();
    }
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'w,E>> {
        Some((**self).base())
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).child(i)
    }
    fn into_child(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::into_child(*self,i)
    }
}
impl<'w,E> WidgetMut<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    #[allow(deprecated)]
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 'w: 'a {
        (**self).childs_mut()
    }
    fn into_childs_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        WidgetMut::into_childs_mut(*self)
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn into_resolve_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::into_resolve_mut(*self, i, invalidate)
    }
    fn inner_mut<'s>(&'s mut self) -> Option<&'s mut dyn WidgetMut<'w,E>> {
        Some((&mut **self).short_lt())
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        WidgetMut::child_mut(&mut **self,i)
    }
    fn into_child_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::into_child_mut(*self,i)
    }
}
