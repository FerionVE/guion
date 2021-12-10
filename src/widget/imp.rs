use std::ops::{Deref, DerefMut};

use super::*;

pub enum AWidget<'a,E> where E: Env {
    Ref(&'a (dyn Widget<E>+'a)),
    Mut(&'a (dyn Widget<E>+'a)),
    Box(Box<dyn Widget<E>+'a>),
}

pub enum AWidgetMut<'a,E> where E: Env {
    Mut(&'a mut (dyn WidgetMut<E>+'a)),
    Box(Box<dyn WidgetMut<E>+'a>),
}

/*
match self {
            Self::Ref(s) => ,
            Self::Mut(s) => ,
            Self::Box(s) => ,
        }
*/

macro_rules! match_view {
    ($senf:ident|$s:ident|$f:expr) => {
        match $senf {
            Self::Ref($s) => $f,
            Self::Mut($s) => $f,
            Self::Box($s) => $f,
        }
    }
}
macro_rules! match_mut {
    ($senf:ident|$s:ident|$f:expr) => {
        match $senf {
            Self::Mut($s) => $f,
            Self::Box($s) => $f,
        }
    }
}

impl<'i,E> Widget<E> for AWidget<'i,E> where E: Env {
    #[inline]
    fn id(&self) -> E::WidgetID {
        match_view!(self|s| s.id() )
    }
    #[inline]
    fn childs(&self) -> usize {
        match_view!(self|s| s.childs() )
    }
    #[inline]
    fn focusable(&self) -> bool {
        match_view!(self|s| s.focusable() )
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        match_view!(self|s| s.in_parent_path(parent) )
    }
    #[inline]
    fn resolved_by_path(&self, sub_path: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match_view!(self|s| s.resolved_by_path(sub_path) )
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        match_view!(self|s| s._focus_on_mouse_down() )
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        match_view!(self|s| s._tabulate_by_tab() )
    }
    #[inline]
    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
        match_view!(self|s| s.debug_type_name(dest) )
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut ERenderer<'_,E>) {
        match_view!(self|s| s._render(l,r) )
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        match_view!(self|s| s._event_direct(l,e) )
    }
    #[inline]
    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        match_view!(self|s| s._size(l,e) )
    }
    #[inline]
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> {
        match_view!(self|s| s.child(i) )
    }
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        match_view!(self|s| s.child_paths(own_path) )
    }
    #[inline]
    fn into_child<'s>(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> where Self: 's {
        match *self {
            Self::Ref(s) => s.child(i),
            Self::Mut(s) => s.child(i),
            Self::Box(s) => s.into_child(i),
        }
    }
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        match *self {
            Self::Ref(s) => s.childs_ref(),
            Self::Mut(s) => s.childs_ref(),
            Self::Box(s) => s.into_childs(),
        }
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        match_view!(self|s| s.child_bounds(l,b,e,force) )
    }
    #[inline]
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> {
        match_view!(self|s| s.childs_ref() )
    }
    #[inline]
    fn resolve<'s>(&'s self, i: E::WidgetPath) -> Result<Resolvable<'s,E>,E::Error> {
        match_view!(self|s| s.resolve(i) )
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,E::Error> where Self: 'w {
        match *self {
            Self::Ref(s) => s.resolve(i),
            Self::Mut(s) => s.resolve(i),
            Self::Box(s) => s.into_resolve(i),
        }
    }
    #[inline]
    fn resolve_child(&self, sub_path: &E::WidgetPath) -> Result<(usize,E::WidgetPath),E::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        match_view!(self|s| s.resolve_child(sub_path) )
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
        match_view!(self|s| s.trace_bounds(l,i,b,e,force) )
    }
    #[inline]
    fn _tabulate_next_child(&self, l: Link<E>, origin: TabulateNextChildOrigin, dir: TabulateDirection) -> TabulateNextChildResponse {
        //TODO is this private fn should we warn?
        match_view!(self|s| s._tabulate_next_child(l,origin,dir) )
    }
    #[inline]
    fn _tabulate(&self, mut l: Link<E>, op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,E::Error> {
        match_view!(self|s| s._tabulate(l,op,dir) )
    }
    #[inline]
    fn inner(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        match_view!(self|s| s.inner() )
    }
    #[inline]
    fn innest(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        match_view!(self|s| s.innest() )
    }
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        todo!() // we m a y b e can pull this off
    }
    #[inline]
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> {
        match_view!(self|s| s.box_ref() )
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        self.boxed()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        match self {
            Self::Ref(s) => s.box_ref(),
            Self::Mut(s) => s.box_ref(),
            Self::Box(s) => s.box_box(),
        }
    }
    #[inline]
    fn gen_diag_error_resolve_fail(&self, sub_path: &E::WidgetPath, op: &'static str) -> E::Error {
        match_view!(self|s| s.gen_diag_error_resolve_fail(sub_path,op) )
    }
}

impl<'i,E> Widget<E> for AWidgetMut<'i,E> where E: Env {
    #[inline]
    fn id(&self) -> E::WidgetID {
        match_mut!(self|s| s.id() )
    }
    #[inline]
    fn childs(&self) -> usize {
        match_mut!(self|s| s.childs() )
    }
    #[inline]
    fn focusable(&self) -> bool {
        match_mut!(self|s| s.focusable() )
    }
    #[inline]
    fn in_parent_path(&self, parent: E::WidgetPath) -> E::WidgetPath {
        match_mut!(self|s| s.in_parent_path(parent) )
    }
    #[inline]
    fn resolved_by_path(&self, sub_path: &E::WidgetPath) -> Option<ResolvesThruResult<E>> {
        match_mut!(self|s| s.resolved_by_path(sub_path) )
    }
    #[inline]
    fn _focus_on_mouse_down(&self) -> bool {
        match_mut!(self|s| s._focus_on_mouse_down() )
    }
    #[inline]
    fn _tabulate_by_tab(&self) -> bool {
        match_mut!(self|s| s._tabulate_by_tab() )
    }
    #[inline]
    fn debug_type_name(&self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
        match_mut!(self|s| s.debug_type_name(dest) )
    }
    #[inline]
    fn _render(&self, l: Link<E>, r: &mut ERenderer<'_,E>) {
        match_mut!(self|s| s._render(l,r) )
    }
    #[inline]
    fn _event_direct(&self, l: Link<E>, e: &EventCompound<E>) -> EventResp {
        match_mut!(self|s| s._event_direct(l,e) )
    }
    #[inline]
    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        match_mut!(self|s| s._size(l,e) )
    }
    #[inline]
    fn child<'s>(&'s self, i: usize) -> Result<Resolvable<'s,E>,()> {
        match_mut!(self|s| s.child(i) )
    }
    #[inline]
    fn child_paths(&self, own_path: E::WidgetPath) -> Vec<E::WidgetPath> {
        match_mut!(self|s| s.child_paths(own_path) )
    }
    #[inline]
    fn into_child<'s>(self: Box<Self>, i: usize) -> Result<Resolvable<'s,E>,()> where Self: 's {
        match *self {
            Self::Mut(s) => s.child(i),
            Self::Box(s) => s.into_child(i),
        }
    }
    #[inline]
    fn into_childs<'w>(self: Box<Self>) -> Vec<Resolvable<'w,E>> where Self: 'w {
        match *self {
            Self::Mut(s) => s.childs_ref(),
            Self::Box(s) => s.into_childs(),
        }
    }
    #[inline]
    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        match_mut!(self|s| s.child_bounds(l,b,e,force) )
    }
    #[inline]
    fn childs_ref<'s>(&'s self) -> Vec<Resolvable<'s,E>> {
        match_mut!(self|s| s.childs_ref() )
    }
    #[inline]
    fn resolve<'s>(&'s self, i: E::WidgetPath) -> Result<Resolvable<'s,E>,E::Error> {
        match_mut!(self|s| s.resolve(i) )
    }
    #[inline]
    fn into_resolve<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<Resolvable<'w,E>,E::Error> where Self: 'w {
        match *self {
            Self::Mut(s) => s.resolve(i),
            Self::Box(s) => s.into_resolve(i),
        }
    }
    #[inline]
    fn resolve_child(&self, sub_path: &E::WidgetPath) -> Result<(usize,E::WidgetPath),E::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        match_mut!(self|s| s.resolve_child(sub_path) )
    }
    #[inline]
    fn trace_bounds(&self, l: Link<E>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
        match_mut!(self|s| s.trace_bounds(l,i,b,e,force) )
    }
    #[inline]
    fn _tabulate_next_child(&self, l: Link<E>, origin: TabulateNextChildOrigin, dir: TabulateDirection) -> TabulateNextChildResponse {
        //TODO is this private fn should we warn?
        match_mut!(self|s| s._tabulate_next_child(l,origin,dir) )
    }
    #[inline]
    fn _tabulate(&self, mut l: Link<E>, op: TabulateOrigin<E>, dir: TabulateDirection) -> Result<TabulateResponse<E>,E::Error> {
        match_mut!(self|s| s._tabulate(l,op,dir) )
    }
    #[inline]
    fn inner(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        match_mut!(self|s| s.inner() )
    }
    #[inline]
    fn innest(&self) -> Option<&dyn Widget<E>> { // fn inner<'s,'w>(&'s self) -> Option<&'s (dyn Widget<E>+'w)> where Self: 'w
        match_mut!(self|s| s.innest() )
    }
    #[inline]
    unsafe fn _as_trait_ref(&self, t: TypeId) -> Option<TraitObject> {
        todo!() // we m a y b e can pull this off
    }
    #[inline]
    fn box_ref<'s>(&'s self) -> WidgetRef<'s,E> {
        match_mut!(self|s| s.box_ref() )
    }
    #[inline]
    fn box_box<'w>(self: Box<Self>) -> WidgetRef<'w,E> where Self: 'w {
        self.boxed()
    }
    #[inline]
    fn boxed<'w>(self) -> WidgetRef<'w,E> where Self: Sized+'w {
        match self {
            Self::Mut(s) => s.box_ref(),
            Self::Box(s) => s.box_box(),
        }
    }
    #[inline]
    fn gen_diag_error_resolve_fail(&self, sub_path: &E::WidgetPath, op: &'static str) -> E::Error {
        match_mut!(self|s| s.gen_diag_error_resolve_fail(sub_path,op) )
    }
}

impl<'i,E> WidgetMut<E> for AWidgetMut<'i,E> where E: Env {
    #[inline]
    fn child_mut<'s>(&'s mut self, i: usize) -> Result<ResolvableMut<'s,E>,()> {
        match_mut!(self|s| s.child_mut(i) )
    }
    #[inline]
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        match *self {
            Self::Mut(s) => s.child_mut(i),
            Self::Box(s) => s.into_child_mut(i),
        }
    }
    #[inline]
    fn childs_mut<'s>(&'s mut self) -> Vec<ResolvableMut<'s,E>> {
        match_mut!(self|s| s.childs_mut() )
    }
    #[inline]
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        match *self {
            Self::Mut(s) => s.childs_mut(),
            Self::Box(s) => s.into_childs_mut(),
        }
    }
    #[inline]
    fn message(&mut self, m: E::Message) {
        match_mut!(self|s| s.message(m) )
    }
    #[inline]
    fn _set_invalid(&mut self, v: bool) {
        match_mut!(self|s| s._set_invalid(v) )
    }
    #[inline]
    fn resolve_mut<'s>(&'s mut self, i: E::WidgetPath) -> Result<ResolvableMut<'s,E>,E::Error> { //TODO eventually use reverse "dont_invaldiate"/"keep_valid" bool
        match_mut!(self|s| s.resolve_mut(i) )
    }
    #[inline]
    fn into_resolve_mut<'w>(self: Box<Self>, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,E::Error> where Self: 'w {
        match *self {
            Self::Mut(s) => s.resolve_mut(i),
            Self::Box(s) => s.into_resolve_mut(i),
        }
    }
    #[inline]
    fn resolve_child_mut(&mut self, sub_path: &E::WidgetPath) -> Result<(usize,E::WidgetPath),E::Error> { //TODO descriptive struct like ResolvesThruResult instead confusing tuple
        match_mut!(self|s| s.resolve_child_mut(sub_path) )
    }
    #[inline]
    fn inner_mut(&mut self) -> Option<&mut dyn WidgetMut<E>> {
        match_mut!(self|s| s.inner_mut() )
    }
    #[inline]
    fn pass(self) -> Self where Self: Sized {
        self
    }
    #[inline]
    fn debug_type_name_mut(&mut self, dest: &mut Vec<&'static str>) {
        dest.push(self.type_name());
        match_mut!(self|s| s.debug_type_name_mut(dest) )
    }
    #[inline]
    unsafe fn _as_trait_mut(&mut self, t: TypeId) -> Option<TraitObject> {
        todo!()
    }
    #[inline]
    fn box_mut<'s>(&'s mut self) -> WidgetRefMut<'s,E> {
        match_mut!(self|s| s.box_mut() )
    }
    #[inline]
    fn box_box_mut<'w>(self: Box<Self>) -> WidgetRefMut<'w,E> where Self: 'w {
        self.boxed_mut()
    }
    #[inline]
    fn boxed_mut<'w>(self) -> WidgetRefMut<'w,E> where Self: Sized+'w {
        match self {
            Self::Mut(s) => s.box_mut(),
            Self::Box(s) => s.box_box_mut(),
        }
    }
    #[inline]
    fn gen_diag_error_resolve_fail_mut(&mut self, sub_path: &E::WidgetPath, op: &'static str) -> E::Error {
        match_mut!(self|s| s.gen_diag_error_resolve_fail_mut(sub_path,op) )
    }
}

impl<'i,E> AWidget<'i,E> where E: Env {
    #[inline]
    pub fn reference<'s>(&'s self) -> AWidget<'s,E> {
        match self {
            Self::Ref(s) => AWidget::Ref(*s),
            Self::Mut(s) => AWidget::Ref(*s),
            Self::Box(s) => AWidget::Ref(&**s),
        }
    }

    #[inline]
    pub fn into_child<'s>(self, i: usize) -> Result<Resolvable<'s,E>,()> where Self: 's {
        match self {
            Self::Ref(s) => s.child(i),
            Self::Mut(s) => s.child(i),
            Self::Box(s) => s.into_child(i),
        }
    }
    #[inline]
    pub fn into_childs<'w>(self) -> Vec<Resolvable<'w,E>> where Self: 'w {
        match self {
            Self::Ref(s) => s.childs_ref(),
            Self::Mut(s) => s.childs_ref(),
            Self::Box(s) => s.into_childs(),
        }
    }
    #[inline]
    pub fn into_resolve<'w>(self, i: E::WidgetPath) -> Result<Resolvable<'w,E>,E::Error> where Self: 'w {
        match self {
            Self::Ref(s) => s.resolve(i),
            Self::Mut(s) => s.resolve(i),
            Self::Box(s) => s.into_resolve(i),
        }
    }
}

impl<'i,E> AWidgetMut<'i,E> where E: Env {
    #[inline]
    pub fn reference<'s>(&'s mut self) -> AWidgetMut<'s,E> where 'i: 's {
        match self {
            Self::Mut(s) => AWidgetMut::Mut(*s),
            Self::Box(s) => AWidgetMut::Mut(&mut **s),
        }
    }

    #[inline]
    pub fn into_child_mut<'w>(self, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        match self {
            Self::Mut(s) => s.child_mut(i),
            Self::Box(s) => s.into_child_mut(i),
        }
    }
    #[inline]
    pub fn into_childs_mut<'w>(self) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        match self {
            Self::Mut(s) => s.childs_mut(),
            Self::Box(s) => s.into_childs_mut(),
        }
    }
    #[inline]
    pub fn into_resolve_mut<'w>(self, i: E::WidgetPath) -> Result<ResolvableMut<'w,E>,E::Error> where Self: 'w {
        match self {
            Self::Mut(s) => s.resolve_mut(i),
            Self::Box(s) => s.into_resolve_mut(i),
        }
    }
}

impl<'i,E> Deref for AWidget<'i,E> where E: Env {
    type Target = dyn Widget<E>+'i;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(s) => *s,
            Self::Mut(s) => *s,
            Self::Box(s) => &**s,
        }
    }
}

impl<'i,E> Deref for AWidgetMut<'i,E> where E: Env {
    type Target = dyn WidgetMut<E>+'i;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Mut(s) => *s,
            Self::Box(s) => &**s,
        }
    }
}
impl<'i,E> DerefMut for AWidgetMut<'i,E> where E: Env {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Mut(s) => *s,
            Self::Box(s) => &mut **s,
        }
    }
}
