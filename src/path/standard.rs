use super::*;
use std::marker::PhantomData;
use arc_slice::ArcSlice;

#[allow(type_alias_bounds)]
pub type StandardPath<E: Env> = SimplePath<E,StdID>;

#[derive(PartialEq,Clone)]
pub struct SimplePath<E,S> {
    v: ArcSlice<S>,
    _p: PhantomData<E>,
}

impl<E,S> WidgetPath<E> for SimplePath<E,S> where
    E: Env,
    S: SubPath<E> + From<E::WidgetID>+Into<E::WidgetID> + Send+Sync + 'static,
    Self: From<E::WidgetPath>+Into<E::WidgetPath>
{
    type SubPath = S;
    fn attach(&mut self, sub: S) {
        self.v.push(sub);
    }
    fn attached(mut self, sub: S) -> Self { //TODO can be default impl
        self.attach(sub);
        self
    }
    fn attach_subpath(&mut self, sub: &Self) {
        self.v.extend_from_slice(&sub.v);
    }
    fn tip(&self) -> Option<&S> {
        self.v.get(self.v.len()-1)
    }
    fn exact_eq(&self, o: &Self) -> bool {
        self.v[..] == o.v[..]
    }
    fn parent(&self) -> Option<Self> {
        if self.is_empty() {return None;}
        let mut parent = self.v.refc();
        parent.pop().unwrap();
        Some(Self{
            v: parent,
            _p: PhantomData,
        })
    }
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize> {
        Self{
            v: self.v.slice(range),
            _p: PhantomData,
        }
    }
    fn index<T>(&self, i: T) -> &S where T: SliceIndex<[S],Output=S> {
        &self.v[i] //TODO eventually non-panic refactor
    }
}

impl<E,S> SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    pub fn new(range: &[S]) -> Self {
        Self{
            v: ArcSlice::from(range),
            _p: PhantomData,
        }
    }
}

impl<E,S> RefClonable for SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    fn refc(&self) -> Self {
        self.clone()
    }
}

unsafe impl<E,S> Statize<E> for SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    type Statur = Self;
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

