use super::*;

/*impl<'s,'l,T,E> Widget<'s,E> for &'s T where T: Widget<'l,E>+WDC<E>+'l, E: Env, 'l: 's {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        short_resolvable_vec(T::childs_ref(*self))
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        T::resolve(*self,i)
            .map(|e| short_resolvable(e) )
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
}
impl<'s,'l,T,E> Widget<'s,E> for &'s mut T where T: WidgetMut<'l,E>+WDC<E>+'l, E: Env, 'l: 's {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        short_resolvable_vec(T::childs_ref(*self))
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        T::resolve(*self,i)
            .map(|e| short_resolvable(e) )
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
}
impl<'s,'l,T,E> WidgetMut<'s,E> for &'s mut T where T: WidgetMut<'l,E>+WDC<E>+'l, E: Env, 'l: 's {
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 's: 'a {
        (**self).childs_mut()
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'s,E>> {
        (**self).childs_mut()
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 's: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn resolve_box_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'s,E>,()> {
        (**self).resolve_mut(i, invalidate)
    }
}

impl<'s,T,E> Widget<'s,E> for Box<T> where T: Widget<'s,E>+WDC<E> + ?Sized, E: Env {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        short_resolvable_vec(T::childs_box(*self))
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        T::resolve_box(*self,i)
            .map(|e| short_resolvable(e) )
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
}
impl<'s,T,E> WidgetMut<'s,E> for Box<T> where T: WidgetMut<'s,E>+WDC<E> + ?Sized, E: Env {
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 's: 'a {
        (**self).childs_mut()
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'s,E>> {
        T::childs_box_mut(*self)
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 's: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn resolve_box_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'s,E>,()> {
        T::resolve_box_mut(*self, i, invalidate)
    }
}*/

/// shrink the lifetime
pub fn short_widget<'l: 's,'s,E: Env>(i: Box<dyn Widget<'l,E>>) -> Box<dyn Widget<'s,E>> {
    unsafe{
        std::mem::transmute::<Box<dyn Widget<'l,E>>,Box<dyn Widget<'s,E>>>(i) //roast me
    }
}

impl<'s,'l,E> Widget<'s,E> for &'s dyn Widget<'l,E> where E: Env, 'l: 's {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        let r: &'s dyn Widget<'l,E> = *self;
        short_resolvable_vec(r.childs_ref())
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        let r: &'s dyn Widget<'l,E> = *self;
        r.resolve(i)
            .map(|e| short_resolvable(e) )
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
}
impl<'s,'l,E> Widget<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 's: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'s,E>> {
        let r: &'s dyn Widget<'l,E> = (**self).base();
        short_resolvable_vec(r.childs_ref())
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        let r: &'s dyn Widget<'l,E> = (**self).base();
        r.resolve(i)
            .map(|e| short_resolvable(e) )
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
}
impl<'s,'l,E> WidgetMut<'s,E> for &'s mut dyn WidgetMut<'l,E> where E: Env, 'l: 's {
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 's: 'a {
        (**self).childs_mut()
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'s,E>> {
        (**self).childs_mut()
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 's: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn resolve_box_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'s,E>,()> {
        (**self).resolve_mut(i, invalidate)
    }
}
impl<'w,E> Widget<'w,E> for Box<dyn Widget<'w,E>> where E: Env {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        short_resolvable_vec(Widget::childs_box(*self))
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        Widget::resolve_box(*self,i)
            .map(|e| short_resolvable(e) )
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
}
impl<'w,E> Widget<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
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
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        (**self).childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        short_resolvable_vec(Widget::childs_box(*self))
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        (**self)._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        (**self).focusable()
    }
    fn invalid(&self) -> bool {
        true
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        (**self).child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        (**self).resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        Widget::resolve_box(*self,i)
            .map(|e| short_resolvable(e) )
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
}
impl<'w,E> WidgetMut<'w,E> for Box<dyn WidgetMut<'w,E>> where E: Env {
    fn childs_mut<'a>(&'a mut self) -> Vec<ResolvableMut<'a,E>> where 'w: 'a {
        (**self).childs_mut()
    }
    fn childs_box_mut(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> {
        WidgetMut::childs_box_mut(*self)
    }
    fn set_invalid(&mut self, v: bool) {
        (**self).set_invalid(v)
    }
    fn resolve_mut<'a>(&'a mut self, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        (**self).resolve_mut(i, invalidate)
    }
    fn resolve_box_mut(self: Box<Self>, i: E::WidgetPath, invalidate: bool) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::resolve_box_mut(*self, i, invalidate)
    }
}