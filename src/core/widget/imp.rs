use super::*;

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
        r.childs_ref().short_lt()
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
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        let r: &'s dyn Widget<'l,E> = *self;
        r.resolve(i)
            .map(|e| e.short_lt() )
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
        Some((&**self).short_lt())
    }
    fn child<'a>(&'a self, i: usize) -> Result<Resolvable<'a,E>,()> where 's: 'a {
        (**self).child(i)
    }
    fn child_box(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        let r: &'s dyn Widget<'l,E> = *self;
        r.child(i)
            .map(|e| e.short_lt() )
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
        r.childs_ref().short_lt()
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
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'s,E>,()> {
        let r: &'s dyn Widget<'l,E> = (**self).base();
        r.resolve(i)
            .map(|e| e.short_lt() )
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
    fn child_box(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> {
        let r: &'s dyn Widget<'l,E> = (**self).base();
        r.child(i)
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
    fn inner_mut<'a>(&'a mut self) -> Option<&'a mut dyn WidgetMut<'s,E>> {
        Some((&mut **self).short_lt())
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 's: 'a {
        (**self).child_mut(i)
    }
    fn child_box_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'s,E>,()> {
        let r: &'s mut dyn WidgetMut<'l,E> = &mut **self;
        r.child_mut(i)
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
        Widget::childs_box(*self).short_lt()
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
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        Widget::resolve_box(*self,i)
            .map(|e| e.short_lt() )
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
    fn child_box(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::child_box(*self,i)
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
        Widget::childs_box(*self).short_lt()
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
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        /*let s: &'s T = *self;
        let e: Result<Resolvable<'s,E>,()> = T::resolve_ref(s,i)
            .map(|e| short_resolvable(e) );
        e*/
        Widget::resolve_box(*self,i)
            .map(|e| e.short_lt() )
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
    fn child_box(self: Box<Self>, i: usize) -> Result<Resolvable<'w,E>,()> {
        Widget::child_box(*self,i)
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
    fn inner_mut<'s>(&'s mut self) -> Option<&'s mut dyn WidgetMut<'w,E>> {
        Some((&mut **self).short_lt())
    }
    fn child_mut<'a>(&'a mut self, i: usize) -> Result<ResolvableMut<'a,E>,()> where 'w: 'a {
        WidgetMut::child_mut(&mut **self,i)
    }
    fn child_box_mut(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> {
        WidgetMut::child_box_mut(*self,i)
    }
}

/*pub trait DeriveWidget<'w,E> where E: Env {
    fn as_ref<'s>(&'s self) -> &'s dyn Widget<'w,E> where 'w: 's;
    fn consume_ref(self) -> WidgetRef<'w,E>;
}
pub trait DeriveWidgetMut<'w,E>: DeriveWidget<'w,E> where E: Env {
    fn as_mut<'s>(&'s mut self) -> &'s mut dyn WidgetMut<'w,E> where 'w: 's;
    fn consume_mut(self) -> WidgetRefMut<'w,E>;
}

impl<'w,T,E> Widget<'w,E> for T where T: DeriveWidget<'w,E>+Statize, E: Env {
    fn id(&self) -> E::WidgetID {
        self.as_ref().id()
    }
    fn render(&self, l: Link<E>, r: &mut RenderLink<E>) -> bool {
        self.as_ref().render(l,r)
    }
    fn event(&self, l: Link<E>, e: (EEvent<E>,&Bounds,u64)) {
        self.as_ref().event(l,e)
    }
    fn size(&self, l: Link<E>) -> ESize<E> {
        self.as_ref().size(l)
    }
    fn childs(&self) -> usize {
        self.as_ref().childs()
    }
    fn childs_ref<'a>(&'a self) -> Vec<Resolvable<'a,E>> where 'w: 'a {
        self.as_ref().childs_ref()
    }
    fn childs_box(self: Box<Self>) -> Vec<Resolvable<'w,E>> {
        short_resolvable_vec(self.consume_ref().childs_ref())
    }
    fn _trace_bounds(&self, l: Link<E>, i: usize, b: &Bounds, force: bool) -> Result<Bounds,()> {
        self.as_ref()._trace_bounds(l, i, b, force)
    }
    fn focusable(&self) -> bool {
        self.as_ref().focusable()
    }
    fn invalid(&self) -> bool {
        self.as_ref()
    }
    #[allow(deprecated)]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        self.as_ref().child_paths(own_path)
    }
    fn resolve<'a>(&'a self, i: E::WidgetPath) -> Result<Resolvable<'a,E>,()> where 'w: 'a {
        self.as_ref().resolve(i)
    }
    fn resolve_box(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,()> {
        self.consume_ref().resolve(i)
            .map(|e| short_resolvable(e) )
    }
    fn resolve_child(&self, p: &EWPSub<E>) -> Result<usize,()> {
        self.as_ref().resolve_child(p)
    }
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, force: bool) -> Result<Bounds,()> {
        self.as_ref().trace_bounds(l, i, b, force)
    }
    fn self_in_parent(&self, parent: E::WidgetPath) -> E::WidgetPath {
        self.as_ref().self_in_parent(parent)
    }
    fn is_subpath(&self, p: &EWPSub<E>) -> bool {
        self.as_ref().is_subpath(p)
    }
    fn _focus_on_mouse_down(&self) -> bool {
        self.as_ref()._focus_on_mouse_down()
    }
    fn _tabulate_by_tab(&self) -> bool {
        self.as_ref()._tabulate_by_tab()
    }
    fn style(&self, s: &mut ESVariant<E>) {
        self.as_ref().style(s)
    }
    fn border(&self, b: &mut Border) {
        self.as_ref().border(b)
    }
    fn debug_type_name(&self) {
        eprintln!("\t{}",self.type_name());
        self.as_ref().debug_type_name();
    }
    fn inner<'a>(&'a self) -> Option<&'a dyn Widget<'w,E>> {
        Some(short_widget_ref(self.as_ref()))
    }
}*/