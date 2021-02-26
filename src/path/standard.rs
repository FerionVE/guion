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

impl<E,S> SimplePath<E,S> {
    #[inline]
    fn attach(&mut self, sub: S) where S: Clone {
        self.v.push(sub);
    }
    #[inline]
    fn attached(mut self, sub: S) -> Self where S: Clone { //TODO can be default impl
        self.attach(sub);
        self
    }
    #[inline]
    fn tip(&self) -> Option<&S> {
        if self.v.len() == 0 {return None;}
        self.v.get(self.v.len()-1)
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
}

impl<E,S> WidgetPath<E> for SimplePath<E,S> where
    E: Env,
    S: SubPath<E> + From<E::WidgetID>+Into<E::WidgetID> + Debug + Send+Sync + 'static,
    Self: From<E::WidgetPath>+Into<E::WidgetPath>
{
    #[inline]
    fn attach_subpath(&mut self, sub: &Self) {
        self.v.extend_from_slice(&sub.v);
    }
    #[inline]
    fn strip_prefix(&self, prefix: &Self) -> Result<Self,()> {
        /*if prefix.v.len() > self.v.len() {return Err(());}
        if self.v[..prefix.v.len()] != prefix.v[..] {return Err(());}
        Ok(self.slice(prefix.v.len()..))*/
        let ptip = match prefix.tip() {
            Some(v) => v,
            None => return Ok(self.clone()),
        };
        if self.tip().is_none() {return Err(());}
        let idx = self.v.iter().enumerate()
            .find(|(i,v)| v.resolve_to_same_widget(ptip) )
            .map(|(i,_)| i );
        match idx {
            Some(i) => Ok(self.slice(i+1..)),
            None => Err(()),
        }
    }
    #[inline]
    fn _dest_widget(&self) -> Option<E::WidgetID> {
        self.v.get(self.v.len()-1).cloned().map(SubPath::into_id)
    }
    #[inline]
    fn exact_eq(&self, o: &Self) -> bool {
        self.v[..] == o.v[..]
    }
    #[inline]
    fn dest_eq(&self, o: &Self) -> bool {
        if self.tip().is_none() && o.tip().is_none() {todo!();} //return true;
        if let (Some(s),Some(o)) = (self.tip(),o.tip()) {
            return s.resolve_to_same_widget(o);
        }
        false
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
    fn empty() -> Self {
        Self{
            v: ArcSlice::new(),
            _p: PhantomData,
        }
    }

    fn resolves_thru_child_id(child_id: E::WidgetID, sub_path: &Self) -> Option<ResolvesThruResult<E>> {
        if sub_path.is_empty() {return None;}
        let sub_path_dest_id;
        if let Some(s) = sub_path.index(0) {
            sub_path_dest_id = s.clone().into_id(); //TODO deprecated old PathSub thing
        }else{
            return None;
        }
        (child_id == sub_path_dest_id)
            .then(|| 
                ResolvesThruResult{
                    //TODO optimize .clone().attached() efficiency
                    sub_path: sub_path.slice(1..).into(),
                }
            )
    }

    fn resolves_thru_child_path(child_path: &Self, sub_path: &Self) -> Option<ResolvesThruResult<E>> {
        child_path.tip()
            .and_then(|tip| 
                Self::resolves_thru_child_id(tip.clone().into_id(),sub_path)
            )
    }

    fn for_child_widget_id(&self, child_id: E::WidgetID) -> Self {
        self.clone().attached(SubPath::from_id(child_id))
    }

    fn for_child_widget_path(&self, child_path: &Self) -> Self {
        if let Some(tip) = child_path.tip() {
            self.clone().attached(tip.clone()) //TODO doesn't use WidgetID conversion like other fns inconstistence rework StdPath::PathFragment
        }else{
            self.clone()
        }
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

impl<E,S> Debug for SimplePath<E,S> where E: Env, S: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.v.len() {
            self.v[i].fmt(f)?;
            if i+1 < self.v.len() {
                write!(f,"/")?;
            }
        }
        Ok(())
    }
}
