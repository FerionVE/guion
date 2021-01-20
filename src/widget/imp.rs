//! redirective implementation for references of widgets
use super::*;

impl<E> Widget<E> for &(dyn Widget<E>+'_) where E: Env {
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
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        (**self)._size(l,e)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        (**self).childs_ref()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b,e, force)
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
    fn resolve(&self, i: E::WidgetPath) -> Result<Resolvable<E>,()> {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (**self).resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b,e, force)
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
    fn inner(&self) -> Option<&dyn Widget<E>> {
        Some(&(**self))
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> { // fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()>
        (**self).child(i)
    }
    #[inline]
    fn into_child<'w>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (**self).child(i)
    }
    #[inline]
    fn box_ref(&self) -> WidgetRef<E> {
        (**self).box_ref()
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        (**self).box_ref()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        (*self).box_ref()
    }
}
impl<E> Widget<E> for &mut (dyn WidgetMut<E>+'_) where E: Env {
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
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        (**self)._size(l,e)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        (**self).childs_ref()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        (**self).childs_ref()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b,e, force)
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
    fn resolve(&self, i: E::WidgetPath) -> Result<Resolvable<E>,()> {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (**self).resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b,e, force)
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
    fn inner(&self) -> Option<&dyn Widget<E>> {
        Some((**self).base())
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        (**self).child(i)
    }
    #[inline]
    fn into_child<'w>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        let r: &dyn Widget<E> = (**self).base();
        r.child(i)
    }
    #[inline]
    fn box_ref(&self) -> WidgetRef<E> {
        (**self).box_ref()
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        (**self).box_ref()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        (*self).box_ref()
    }
}
impl<E> WidgetMut<E> for &mut (dyn WidgetMut<E>+'_) where E: Env {
    #[allow(deprecated)]
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        (**self).childs_mut()
    }
    #[allow(deprecated)]
    #[inline]
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        (**self).childs_mut()
    }
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        (**self)._set_invalid(v)
    }
    #[inline]
    fn resolve_mut(&mut self, i: E::WidgetPath) -> Result<ResolvableMut<E>,()> { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i)
    }
    #[inline]
    fn into_resolve_mut<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        (**self).resolve_mut(i)
    }
    #[inline]
    fn inner_mut(&mut self) -> Option<&mut dyn WidgetMut<E>> {
        Some(&mut(**self))
    }
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        (**self).child_mut(i)
    }
    #[inline]
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        (**self).child_mut(i)
    }
    #[inline]
    fn message(&mut self, m: E::Message) {
        (**self).message(m)
    }
    #[inline]
    fn box_mut(&mut self) -> WidgetRefMut<E> {
        (**self).box_mut()
    }
    #[inline]
    fn box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w {
        (**self).box_mut()
    }
    #[inline]
    fn boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized+'w {
        (*self).box_mut()
    }
}
impl<E> Widget<E> for Box<(dyn Widget<E>+'_)> where E: Env {
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
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        (**self)._size(l,e)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        (**self).childs_ref()
    }
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        (*self).into_childs()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b,e, force)
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
    fn resolve(&self, i: E::WidgetPath) -> Result<Resolvable<E>,()> {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (*self).into_resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b,e, force)
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
    fn inner(&self) -> Option<&dyn Widget<E>> {
        Some(&**self)
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        (**self).child(i)
    }
    #[inline]
    fn into_child<'w>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (*self).into_child(i)
    }
    #[inline]
    fn box_ref(&self) -> WidgetRef<E> {
        (**self).box_ref()
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        (*self).box_box()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        self.box_box()
    }
}
impl<E> Widget<E> for Box<(dyn WidgetMut<E>+'_)> where E: Env {
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
    fn _size(&self, l: Link<E>, e: &ESVariant<E>) -> ESize<E> {
        (**self)._size(l,e)
    }
    #[inline]
    fn childs(&self) -> usize {
        (**self).childs()
    }
    #[allow(deprecated)]
    #[inline]
    fn childs_ref(&self) -> Vec<Resolvable<E>> {
        (**self).childs_ref()
    }
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        (*self).into_childs()
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Vec<Bounds>,()> {
        (**self).child_bounds(l, b,e, force)
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
    fn resolve(&self, i: E::WidgetPath) -> Result<Resolvable<E>,()> where {
        (**self).resolve(i)
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (*self).into_resolve(i)
    }
    #[inline]
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        (**self).resolve_child(p)
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &ESVariant<E>, force: bool) -> Result<Bounds,()> {
        (**self).trace_bounds(l, i, b,e, force)
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
    fn inner(&self) -> Option<&dyn Widget<E>> {
        Some((**self).base())
    }
    #[inline]
    fn child(&self, i: usize) -> Result<Resolvable<E>,()> {
        (**self).child(i)
    }
    #[inline]
    fn into_child<'w>(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> where Self: 'w {
        (*self).into_child(i)
    }
    #[inline]
    fn box_ref(&self) -> WidgetRef<E> {
       (**self).box_ref()
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        (*self).box_box()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        self.box_box()
    }
}
impl<E> WidgetMut<E> for Box<(dyn WidgetMut<E>+'_)> where E: Env {
    #[allow(deprecated)]
    #[inline]
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        (**self).childs_mut()
    }
    #[inline]
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        (*self).into_childs_mut()
    }
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        (**self)._set_invalid(v)
    }
    #[inline]
    fn resolve_mut(&mut self, i: E::WidgetPath) -> Result<ResolvableMut<E>,()> { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i)
    }
    #[inline]
    fn into_resolve_mut<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        (*self).into_resolve_mut(i)
    }
    #[inline]
    fn inner_mut(&mut self) -> Option<&mut dyn WidgetMut<E>> {
        Some(&mut(**self))
    }
    #[inline]
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        (**self).child_mut(i)
    }
    #[inline]
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        (*self).into_child_mut(i)
    }
    #[inline]
    fn message(&mut self, m: E::Message) {
        (**self).message(m)
    }
    #[inline]
    fn box_mut(&mut self) -> WidgetRefMut<E> {
        (**self).box_mut()
    }
    #[inline]
    fn box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w {
        (*self).box_box_mut()
    }
    #[inline]
    fn boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized+'w {
        self.box_box_mut()
    }
}
