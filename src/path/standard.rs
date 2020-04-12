use super::*;
use std::{ops::Range, sync::Arc, marker::PhantomData};

#[allow(type_alias_bounds)]
pub type StandardPath<E: Env> = SimplePath<E,StdID>;

#[derive(PartialEq,Clone)]
pub struct SimplePath<E,S> {
    v: Arc<Vec<S>>,
    slice: Range<usize>,
    _p: PhantomData<E>,
}

impl<E,S> WidgetPath<E> for SimplePath<E,S> where
    E: Env,
    S: SubPath<E> + From<E::WidgetID>+Into<E::WidgetID> + Send+Sync + 'static,
    Self: From<E::WidgetPath>+Into<E::WidgetPath>
{
    type SubPath = S;
    fn attach(&mut self, sub: S) {
        Arc::make_mut(&mut self.v).push(sub);
        self.slice.end += 1;
    }
    fn attached(mut self, sub: S) -> Self { //TODO can be default impl
        self.attach(sub);
        self
    }
    fn attach_subpath(&mut self, sub: &Self) {
        let senf = Arc::make_mut(&mut self.v);
        senf.extend_from_slice(sub._get());
        self.slice.end += ExactSizeIterator::len(&sub.slice); //todo fix ambi
    }
    fn id(&self) -> E::WidgetID {
        self.tip().clone().into()
    }
    fn tip(&self) -> &S {
        &self.v[self.slice.end-1]
    }
    fn parent(&self) -> Option<Self> {
        if self.is_empty() {return None;}
        Some(Self{
            v: self.v.refc(),
            slice: self.slice.start .. self.slice.end-1,
            _p: PhantomData,
        })
    }
    fn is_empty(&self) -> bool {
        ExactSizeIterator::len(&self.slice) == 0
    }
    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize> {
        Self{
            v: self.v.refc(),
            slice: slice_range(&self.slice,range),
            _p: PhantomData,
        }
    }
    fn index<T>(&self, i: T) -> &S where T: SliceIndex<[S],Output=S> {
        &self._get()[i]
    }
}

impl<E,S> SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    pub fn new(range: &[S], root_id: S) -> Self {
        let mut dest = Vec::with_capacity(range.len()+1);
        dest.push(root_id);
        dest.extend_from_slice(range);
        Self{
            slice: 1..dest.len(),
            v: Arc::new(dest),
            _p: PhantomData,
        }
    }
    fn _get(&self) -> &[S] {
        &self.v[self.slice.clone()]
    }
    fn _root(&self) -> &S {
        &self.v[0]
    }
}

impl<E,S> RefClonable for SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    fn refc(&self) -> Self {
        self.clone()
    }
}

//TODO fix the AsWidget generic impl
/*impl<E,S> AsWidget<'static,E> for SimplePath<E,S> where E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'static: 's {
        Resolvable::Path(self.clone().into())
    }
    fn into_ref(self) -> Resolvable<'static,E> {
        Resolvable::Path(self.clone().into())
    }
}
impl<E,S> AsWidgetMut<'static,E> for SimplePath<E,S> where E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'static: 's {
        ResolvableMut::Path(self.clone().into())
    }
    fn into_mut(self) -> ResolvableMut<'static,E> {
        ResolvableMut::Path(self.clone().into())
    }
}*/

fn slice_range<S>(range: &Range<usize>, slice: S) -> Range<usize> where S: RangeBounds<usize> {
    let (os,oe) = (range.start,range.end);
    let (mut s,mut e) = (os,oe);
    match slice.end_bound() {
        std::ops::Bound::Included(b) => e = oe.min(b-1+os),
        std::ops::Bound::Excluded(b) => e = oe.min(b+os),
        std::ops::Bound::Unbounded => (),
    }
    match slice.start_bound() {
        std::ops::Bound::Included(b) => s = os.max(b+os),
        std::ops::Bound::Excluded(b) => s = os.max(b-1+os),
        std::ops::Bound::Unbounded => (),
    }
    s..e
}