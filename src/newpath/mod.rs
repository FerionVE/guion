use std::any::Any;
use std::marker::PhantomData;
use std::ops::Add;
use std::sync::Arc;
use crate::env::Env;

pub trait PathFragment<E>: 'static {
    type Stack<I>: PathStack<E> where I: PathStack<E> + Sized;
    type Resolvus<I>: PathResolvus<E> where I: PathResolvus<E> + Sized;

    fn _as_any(&self) -> &dyn Any;

    fn push_on_stack<I>(&self, target_stack: I) -> Self::Stack<I> where I: PathStack<E> + Sized;
}

pub trait PathStack<E> { // reverse: inner = parent widget
    fn _erase(&self) -> &(dyn PathStackDyn<E>+'_);

    fn inner(&self) -> Option<&(dyn PathStackDyn<E>+'_)>; //always returns Some, unless called on the tail ()
    fn _fragment(&self) -> &dyn Any;

    #[inline]
    fn tail(&self) -> bool {
        false
    }

    #[inline]
    fn try_fragment<F>(&self) -> Option<&F> where F: PathFragment<E> {
        self._fragment().downcast_ref::<F>()
    }

    #[inline]
    fn into_resolvus(&self) -> Arc<dyn PathResolvusDyn<E>> {
        self._build_resolvus(())
    }

    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static;// {
    //     self.inner._build_resolvus(SelfResolvus{inner: i, fragment: self.fragment})
    // }

    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        self._build_resolvus_for_fwd_compare((),other)
    }

    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat where I: PathResolvus<E>;
}

pub trait PathResolvus<E> { // non-reverse: inner = child widget
    fn _erase(&self) -> &(dyn PathResolvusDyn<E>+'_);

    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)>;
    fn _fragment(&self) -> &dyn Any;

    #[inline]
    fn try_fragment<F>(&self) -> Option<&F> where F: PathFragment<E> {
        (self._fragment() as &dyn Any).downcast_ref::<F>()
    }

    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat;

    //fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>>;
}

#[derive(PartialEq,Clone,Copy)]
pub enum FwdCompareStat {
    Falsified,
    Equal,
    ChildOfSelf,
    ParentOfSelf,
}

pub trait PathStackDyn<E> {
    fn inner_dyn(&self) -> Option<&(dyn PathStackDyn<E>+'_)>;
    fn _fragment_dyn(&self) -> &dyn Any;
    fn _build_resolvus_dyn(&self, i: Box<dyn PathResolvusDyn<E>>) -> Arc<dyn PathResolvusDyn<E>>;
    fn _build_resolvus_for_fwd_compare_dyn(&self, i: &(dyn PathResolvusDyn<E>+'_), other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat;
}

pub trait PathResolvusDyn<E> { // non-reverse: inner = child widget
    fn inner_dyn(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)>;
    //fn _clone_into_box_dyn(&self) -> Arc<dyn PathResolvusDyn<E>>;
    fn _fragment_dyn(&self) -> &dyn Any;
    fn fwd_compare_dyn<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat;
}

impl<T,E> PathStackDyn<E> for T where T: PathStack<E>, E: 'static {
    #[inline]
    fn inner_dyn(&self) -> Option<&(dyn PathStackDyn<E>+'_)> {
        (*self).inner()
    }
    #[inline]
    fn _fragment_dyn(&self) -> &dyn Any {
        (*self)._fragment()
    }
    #[inline]
    fn _build_resolvus_dyn(&self, i: Box<dyn PathResolvusDyn<E>>) -> Arc<dyn PathResolvusDyn<E>> {
        (*self)._build_resolvus(i)
    }
    #[inline]
    fn _build_resolvus_for_fwd_compare_dyn(&self, i: &(dyn PathResolvusDyn<E>+'_), other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        (*self)._build_resolvus_for_fwd_compare(i, other)
    }
}

impl<T,E> PathResolvusDyn<E> for T where T: PathResolvus<E> {
    #[inline]
    fn inner_dyn(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)>{
        (*self).inner()
    }
    #[inline]
    fn _fragment_dyn(&self) -> &dyn Any {
        (*self)._fragment()
    }
    #[inline]
    fn fwd_compare_dyn<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        (*self).fwd_compare(other)
    }
    // #[inline]
    // fn _clone_into_box_dyn(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     (*self)._clone_into_box()
    // }
}

impl<E> PathStack<E> for dyn PathStackDyn<E> + '_ {
    #[inline]
    fn _erase(&self) -> &(dyn PathStackDyn<E>+'_) {
        self
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathStackDyn<E>+'_)> {
        (*self).inner_dyn()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (*self)._fragment_dyn()
    }
    #[inline]
    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static {
        (*self)._build_resolvus_dyn(Box::new(i))
    }
    #[inline]
    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat where I: PathResolvus<E> {
        (*self)._build_resolvus_for_fwd_compare_dyn(i._erase(), other)
    }
}

impl<E> PathResolvus<E> for dyn PathResolvusDyn<E> + '_ {
    #[inline]
    fn _erase(&self) -> &(dyn PathResolvusDyn<E>+'_) {
        self
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)> {
        (*self).inner_dyn()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (*self)._fragment_dyn()
    }
    #[inline]
    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        (*self).fwd_compare_dyn(other)
    }
    // #[inline]
    // fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     (*self)._clone_into_box_dyn()
    // }
}

impl<E> PathStack<E> for () where E: 'static {
    #[inline]
    fn _erase(&self) -> &(dyn PathStackDyn<E>+'_) {
        self
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathStackDyn<E>+'_)> {
        None
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static {
        Arc::new(i) //TODO avoid boxing twice at this step
    }
    #[inline]
    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat where I: PathResolvus<E> {
        i.fwd_compare(other)
    }
}

impl<E> PathResolvus<E> for () {
    #[inline]
    fn _erase(&self) -> &(dyn PathResolvusDyn<E>+'_) {
        self
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)> {
        None
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        if let Some(_) = other.inner() {
            FwdCompareStat::ChildOfSelf
        } else {
            FwdCompareStat::Equal
        }
    }
    // #[inline]
    // fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     Box::new(())
    // }
}

impl<E,T> PathStack<E> for Box<T> where T: PathStack<E> + ?Sized {
    #[inline]
    fn _erase(&self) -> &(dyn PathStackDyn<E>+'_) {
        (**self)._erase()
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathStackDyn<E>+'_)> {
        (**self).inner()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (**self)._fragment()
    }
    #[inline]
    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static {
        (**self)._build_resolvus(i)
    }
    #[inline]
    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat where I: PathResolvus<E> {
        (**self)._build_resolvus_for_fwd_compare(i, other)
    }
}

impl<E,T> PathStack<E> for &T where T: PathStack<E> + ?Sized {
    #[inline]
    fn _erase(&self) -> &(dyn PathStackDyn<E>+'_) {
        (**self)._erase()
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathStackDyn<E>+'_)> {
        (**self).inner()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (**self)._fragment()
    }
    #[inline]
    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static {
        (**self)._build_resolvus(i)
    }
    #[inline]
    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat where I: PathResolvus<E> {
        (**self)._build_resolvus_for_fwd_compare(i, other)
    }
}

impl<E,T> PathResolvus<E> for Box<T> where T: PathResolvus<E> + ?Sized {
    #[inline]
    fn _erase(&self) -> &(dyn PathResolvusDyn<E>+'_) {
        (**self)._erase()
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)> {
        (**self).inner()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (**self)._fragment()
    }
    #[inline]
    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        (**self).fwd_compare(other)
    }
    // #[inline]
    // fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     (**self)._clone_into_box()
    // }
}

impl<E,T> PathResolvus<E> for &T where T: PathResolvus<E> + ?Sized {
    #[inline]
    fn _erase(&self) -> &(dyn PathResolvusDyn<E>+'_) {
        (**self)._erase()
    }
    #[inline]
    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E>+'_)> {
        (**self).inner()
    }
    #[inline]
    fn _fragment(&self) -> &dyn Any {
        (**self)._fragment()
    }
    #[inline]
    fn fwd_compare<'a,'b>(&self, other: &(dyn PathResolvusDyn<E>+'_)) -> FwdCompareStat {
        (**self).fwd_compare(other)
    }
    // #[inline]
    // fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     (**self)._clone_into_box()
    // }
}

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct SimpleId<V>(pub V) where V: Clone + PartialEq + 'static;

impl<V,E> PathFragment<E> for SimpleId<V> where V: Clone + PartialEq + 'static, E: Env {
    type Stack<I> = SimplePathStack<Self,E,I> where I: PathStack<E> + Sized;
    type Resolvus<I> = SimplePathResolvus<Self,E,I> where I: PathResolvus<E> + Sized;

    #[inline]
    fn _as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn push_on_stack<I>(&self, target_stack: I) -> Self::Stack<I> where I: PathStack<E> + Sized {
        SimplePathStack { inner: target_stack, value: self.clone(), _p: PhantomData }
    }
}

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct FixedIdx(pub usize);

impl<E> PathFragment<E> for FixedIdx where E: Env {
    type Stack<I> = SimplePathStack<Self,E,I> where I: PathStack<E> + Sized;
    type Resolvus<I> = SimplePathResolvus<Self,E,I> where I: PathResolvus<E> + Sized;

    #[inline]
    fn _as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn push_on_stack<I>(&self, target_stack: I) -> Self::Stack<I> where I: PathStack<E> + Sized {
        SimplePathStack { inner: target_stack, value: self.clone(), _p: PhantomData }
    }
}

// impl<V,E,I> Add<I> for SimpleId<V> where I: PathStack<E> + Sized {
//     type Output = <Self as PathFragment<E>>::Stack<I>;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.push_to_stack(rhs)
//     }
// }

// impl<V,E,I> Add<I> for &SimpleId<V> where I: PathStack<E> + Sized {
//     type Output = <Self as PathFragment<E>>::Stack<I>;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.push_to_stack(rhs)
//     }
// }

pub struct SimplePathStack<V,E,S> where V: PathFragment<E> + Clone + PartialEq, E: Env {
    pub inner: S,
    pub value: V,
    pub _p: PhantomData<E>,
}

pub struct SimplePathResolvus<V,E,S> where V: PathFragment<E> + Clone + PartialEq, E: Env {
    pub inner: S,
    pub value: V,
    pub _p: PhantomData<E>,
}

impl<V,S,E> PathStack<E> for SimplePathStack<V,E,S> where S: PathStack<E>, V: PathFragment<E> + Clone + PartialEq, E: Env {
    #[inline]
    fn _erase(&self) -> &(dyn PathStackDyn<E> + '_) {
        self
    }

    #[inline]
    fn inner(&self) -> Option<&(dyn PathStackDyn<E> + '_)> {
        Some(&self.inner)
    }

    #[inline]
    fn _fragment(&self) -> &dyn Any {
        &self.value
    }

    #[inline]
    fn _build_resolvus<I>(&self, i: I) -> Arc<dyn PathResolvusDyn<E>> where I: PathResolvus<E> + 'static {
        self.inner._build_resolvus(SimplePathResolvus { inner: i, value: self.value.clone(), _p: PhantomData })
    }

    #[inline]
    fn _build_resolvus_for_fwd_compare<I>(&self, i: I, other: &(dyn PathResolvusDyn<E> + '_)) -> FwdCompareStat where I: PathResolvus<E> {
        self.inner._build_resolvus_for_fwd_compare(SimplePathResolvus { inner: i, value: self.value.clone(), _p: PhantomData }, other)
    }
}

impl<V,S,E> PathResolvus<E> for SimplePathResolvus<V,E,S> where S: PathResolvus<E>, V: PathFragment<E> + Clone + PartialEq, E: Env {
    #[inline]
    fn _erase(&self) -> &(dyn PathResolvusDyn<E> + '_) {
        self
    }

    #[inline]
    fn inner(&self) -> Option<&(dyn PathResolvusDyn<E> + '_)> {
        Some(&self.inner)
    }

    #[inline]
    fn _fragment(&self) -> &dyn Any {
        &self.value
    }

    #[inline]
    fn fwd_compare<'a, 'b>(&self, other: &(dyn PathResolvusDyn<E> + '_)) -> FwdCompareStat {
        if let Some(other_inner) = other.inner() {
            if other.try_fragment::<V>() == Some(&self.value) {
                self.inner.fwd_compare(other_inner)
            } else {
                FwdCompareStat::Falsified
            }
        } else {
            // the other stack is already at the end
            FwdCompareStat::ParentOfSelf
        }
    }
    // #[inline]
    // fn _clone_into_box(&self) -> Arc<dyn PathResolvusDyn<E>> {
    //     Box::new(Self{ inner: self.inner._clone_into_box(), value: self.value.clone(), _p: PhantomData })
    // }
}
