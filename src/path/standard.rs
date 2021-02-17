use super::*;
use std::marker::PhantomData;
use arc_slice::ArcSlice;

#[allow(type_alias_bounds)]
pub type StdPath<E: Env> = SimplePath<E,StdID>;

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
    #[inline]
    fn attach(&mut self, sub: S) {
        self.v.push(sub);
    }
    #[inline]
    fn attached(mut self, sub: S) -> Self { //TODO can be default impl
        self.attach(sub);
        self
    }
    #[inline]
    fn attach_path(&mut self, sub: &Self) {
        self.v.extend_from_slice(&sub.v);
    }
    #[inline]
    fn tip(&self) -> Option<&S> {
        self.v.get(self.v.len()-1)
    }
    #[inline]
    fn exact_eq(&self, o: &Self) -> bool {
        self.v[..] == o.v[..]
    }
    #[inline]
    fn parent(&self) -> Option<Self> {
        if self.is_empty() {return None;}
        let mut parent = self.v.refc();
        parent.pop().unwrap();
        Some(Self{
            v: parent,
            _p: PhantomData,
        })
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
    #[inline]
    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize> {
        Self{
            v: self.v.slice(range),
            _p: PhantomData,
        }
    }
    #[inline]
    fn index<T>(&self, i: T) -> Option<&S> where T: SliceIndex<[S],Output=S> {
        self.v.get(i)
    }
    #[inline]
    fn empty() -> Self {
        Self{
            v: ArcSlice::new(),
            _p: PhantomData,
        }
    }

    fn _resolves_thru<W>(child: &W, sub_path: &Self) -> Option<ResolvesThruResult<E>> where W: Widget<E>+?Sized {
        let sub_path_dest_id = sub_path.index(0).unwrap().clone().into_id(); //TODO deprecated old PathSub thing
        let child_id = child.id();
        if child_id == sub_path_dest_id {
            // sub_path does indeed resolve to or through child
            Some(ResolvesThruResult{
                //TODO optimize .clone().attached() efficiency
                sub_path: sub_path.slice(1..).into(),
            })
        }else{
            None
        }
    }

    fn for_child_widget<W>(&self, child: &W) -> Self where W: Widget<E>+?Sized {
        self.clone().attached(SubPath::from_id(child.id()))
    }
}

impl<E,S> SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    #[inline]
    pub fn new(range: &[S]) -> Self {
        Self{
            v: ArcSlice::from(range),
            _p: PhantomData,
        }
    }
}

impl<E,S> RefClonable for SimplePath<E,S> where E: Env, S: SubPath<E> + Send+Sync + 'static {
    #[inline]
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
