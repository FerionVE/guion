use std::any::Any;
use std::mem::{MaybeUninit, ManuallyDrop};
use std::ops::Range;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{FixedIdx, PathFragment, PathResolvus, PathStack, PathResolvusDyn};
use crate::pathslice::{NewPathStack, PathSliceRef, PathSliceMatch};
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::pane_childs::PaneChildWidget;
use crate::widget::pane_childs::{PaneChildsDyn, fixed_idx::WidgetsFixedIdx};
use crate::widget_decl::WidgetDecl;
use crate::widget_decl::route::UpdateRoute;

use super::PaneChildsDecl;

mod impl_tuple;

impl<E,T> PaneChildsDecl<E> for WidgetsFixedIdx<&[T]> where T: WidgetDecl<E>, E: Env {
    type Retained = WidgetsFixedIdx<Vec<PaneChildWidget<T::Widget,E>>>;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        if let PathSliceMatch::Match(r2, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
            if r2.0 >= 0 && (r2.0 as usize) < self.0.len() {
                self.0[r2.0 as usize].send_mutation(&mut path.with(*r2), resolve_inner, args, root, ctx)
            }
        } else {
            //TODO what happens if the mutor is lost
        }
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained {
        WidgetsFixedIdx(
            self.0.iter()
                .enumerate()
                .map(|(idx,decl)|
                    PaneChildWidget::new( decl.instantiate(&mut path.with(FixedIdx(idx as isize)), root.fork(), ctx) )
                )
                .collect()
        )
    }

    fn update(
        &self,
        w: &mut Self::Retained,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        // If resolve, try only update resolve scope
        if let Some(resolve) = route.resolving() {
            if let PathSliceMatch::Match(r2, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
                if r2.0 >= 0 && (r2.0 as usize) < self.0.len().min(w.0.len()) {
                    let v = self.0[r2.0 as usize].update(&mut w.0[r2.0 as usize].widget, &mut path.with(*r2), route.for_child_1::<FixedIdx>(), root, ctx);
                    w.0[r2.0 as usize].invalidate(v);
                    return v;
                }
            } else {
                //TODO what to do on invalid scope resolves in update?
            }
            return Invalidation::new();
        }

        let mut vali = Invalidation::valid();

        // Remove old tail
        if w.0.len() > self.0.len() {
            for (idx,w) in w.0[self.0.len()..].iter_mut().enumerate() {
                w.widget.end(&mut path.with(FixedIdx((self.0.len() + idx) as isize)), root.fork(), ctx);
                vali = Invalidation::new();
            }
        }
        // Update persisted exising area
        for (idx,(d,w)) in self.0.iter().zip(w.0.iter_mut()).enumerate() {
            let v = d.update(&mut w.widget, &mut path.with(FixedIdx(idx as isize)), route.for_child_1::<FixedIdx>(), root.fork(), ctx);
            w.invalidate(v);
            vali |= v;
        }
        // Instantiate new tail
        if self.0.len() > w.0.len() {
            for (idx,d) in self.0[w.0.len()..].iter().enumerate() {
                let mut path = path.with(FixedIdx((w.0.len() + idx) as isize));
                w.0.push(PaneChildWidget::new( d.instantiate(&mut path, root.fork(), ctx) ));
                vali = Invalidation::new();
            }
        }
        assert_eq!(self.0.len(), w.0.len());

        vali
    }

    fn update_restore(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Retained,Invalidation) {
        let prev_len = prev.len();
        
        let mut vali = Invalidation::valid();

        // Remove old tail
        if prev_len > self.0.len() {
            end_range_dyn(prev, self.0.len() .. prev_len, path, root.fork(), ctx);
            vali = Invalidation::new();
        }

        let mut new = Vec::with_capacity(self.0.len());
        
        let restorable = prev_len.min(self.0.len());

        // Restore-update persisted exising area
        prev.idx_range_dyn_mut(0 .. restorable, &mut |result| {
            let mut path = path.with(result.child_id);
            let d = &self.0[result.idx as usize];
            let (w,v) = d.update_restore(result.widget, &mut path, root.fork(), ctx);
            let mut w = PaneChildWidget::new(w);
            w.invalidate(v);
            new.push(w);
            vali |= v;
        });

        // Instantiate new tail
        if self.0.len() > restorable {
            for (idx,d) in self.0[restorable..].iter().enumerate() {
                let mut path = path.with(FixedIdx((restorable + idx) as isize));
                new.push(PaneChildWidget::new( d.instantiate(&mut path, root.fork(), ctx) ));
                vali = Invalidation::new();
            }
        }

        assert_eq!(self.0.len(), new.len());

        (WidgetsFixedIdx(new), vali)
    }
}

impl<E,T,const N: usize> PaneChildsDecl<E> for WidgetsFixedIdx<[T;N]> where T: WidgetDecl<E>, E: Env {
    type Retained = WidgetsFixedIdx<[PaneChildWidget<T::Widget,E>;N]>;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        if let PathSliceMatch::Match(r2, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
            if r2.0 >= 0 && (r2.0 as usize) < self.0.len() {
                self.0[r2.0 as usize].send_mutation(&mut path.with(*r2), resolve_inner, args, root, ctx)
            }
        } else {
            //TODO what happens if the mutor is lost
        }
    }

    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized {
        WidgetsFixedIdx(
            trans_array_enumerated(self.0, |idx,decl|
                PaneChildWidget::new( decl.build(&mut path.with(FixedIdx(idx as isize)), root.fork(), ctx) )
            )
        )
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained {
        WidgetsFixedIdx(
            trans_array_enumerated_ref(&self.0, |idx,decl|
                PaneChildWidget::new( decl.build(&mut path.with(FixedIdx(idx as isize)), root.fork(), ctx) )
            )
        )
    }

    fn update(
        &self,
        w: &mut Self::Retained,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        // If resolve, try only update resolve scope
        if let Some(resolve) = route.resolving() {
            if let PathSliceMatch::Match(r2, resolve_inner) = resolve.fetch().slice_forward::<FixedIdx>() {
                if r2.0 >= 0 && (r2.0 as usize) < self.0.len().min(w.0.len()) {
                    let v = self.0[r2.0 as usize].update(&mut w.0[r2.0 as usize].widget, &mut path.with(*r2), route.for_child_1::<FixedIdx>(), root, ctx);
                    w.0[r2.0 as usize].invalidate(v);
                    return v;
                }
            } else {
                //TODO what to do on invalid scope resolves in update?
            }
            return Invalidation::new();
        }

        let mut vali = Invalidation::valid();

        // Update persisted exising area
        for (idx,(d,w)) in self.0.iter().zip(w.0.iter_mut()).enumerate() {
            let v = d.update(&mut w.widget, &mut path.with(FixedIdx(idx as isize)), route.for_child_1::<FixedIdx>(), root.fork(), ctx);
            w.invalidate(v);
            vali |= v;
        }

        vali
    }

    fn update_restore(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Retained,Invalidation) {
        let prev_len = prev.len();

        let mut vali = Invalidation::valid();

        // Remove old tail
        if prev_len > N {
            end_range_dyn(prev, N .. prev_len, path, root.fork(), ctx);
            vali = Invalidation::new();
        }

        let mut new: ManuallyDrop<MaybeUninit<[PaneChildWidget<T::Widget,E>;N]>> = ManuallyDrop::new(MaybeUninit::uninit());

        let new_mut = unsafe { &mut *(new.as_mut_ptr() as *mut [MaybeUninit<PaneChildWidget<T::Widget,E>>;N]) };

        let mut add_pos = 0;
        
        let restorable = prev_len.min(self.0.len());

        // Restore-update persisted exising area
        prev.idx_range_dyn_mut(0 .. restorable, &mut |result| {
            debug_assert_eq!(result.idx as usize, add_pos);
            let mut path = path.with(result.child_id);
            let d = &self.0[result.idx as usize];
            let (w,v) = d.update_restore(result.widget, &mut path, root.fork(), ctx);
            let mut w = PaneChildWidget::new(w);
            w.invalidate(v);
            new_mut[add_pos].write(w);
            add_pos += 1;
            vali |= v;
        });

        assert_eq!(add_pos, restorable);

        // Instantiate new tail
        if N > restorable {
            for d in &self.0[restorable..] {
                let mut path = path.with(FixedIdx(add_pos as isize));
                new_mut[add_pos].write(PaneChildWidget::new( d.instantiate(&mut path, root.fork(), ctx) ));
                add_pos += 1;
                vali |= Invalidation::new();
            }
        }

        assert_eq!(add_pos, N);

        (WidgetsFixedIdx(unsafe { ManuallyDrop::into_inner(new).assume_init() }), vali)
    }
}

impl<E,T,const N: usize> PaneChildsDecl<E> for WidgetsFixedIdx<&[T;N]> where T: WidgetDecl<E>, E: Env {
    type Retained = WidgetsFixedIdx<[PaneChildWidget<T::Widget,E>;N]>;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) {
        (*bender(self)).send_mutation(path, resolve, args, root, ctx)
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained {
        (*bender(self)).instantiate(path, root, ctx)
    }

    fn update(
        &self,
        w: &mut Self::Retained,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        (*bender(self)).update(w, path, route, root, ctx)
    }

    fn update_restore(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Retained,Invalidation) {
        (*bender(self)).update_restore(prev, path, root, ctx)
    }
}

#[inline]
fn end_range_dyn<CID,E>(w: &mut (dyn PaneChildsDyn<E,ChildID=CID> + '_), range: Range<usize>, parent_path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> usize
where
    CID: PathFragment<E> + Clone + Copy + 'static,
    E: Env,
{
    let mut n = 0;
    w.idx_range_dyn_mut(range, &mut |result| {
        result.widget.end(&mut parent_path.with(result.child_id), root.fork(), ctx);
        n += 1;
    });
    n
}

#[inline]
fn bender<'a,'b,T>(v: &'a WidgetsFixedIdx<&'b T>) -> &'a WidgetsFixedIdx<T> where 'b: 'a, T: 'b + Sized {
    unsafe{std::mem::transmute(v)}
}

#[inline]
pub(crate) fn trans_array_enumerated<T,U,const N: usize>(v: [T;N], mut f: impl FnMut(usize,T) -> U) -> [U;N] {
    unsafe {
        let mut dest: ManuallyDrop<MaybeUninit<[U;N]>> = ManuallyDrop::new(MaybeUninit::uninit());
        for (i,(entry,src)) in (*(dest.as_mut_ptr() as *mut [MaybeUninit<U>;N])).iter_mut().zip(v).enumerate() {
            entry.write( f(i,src) );
        }
        ManuallyDrop::into_inner(dest).assume_init()
    }
}

#[inline]
pub(crate) fn trans_array_enumerated_ref<T,U,const N: usize>(v: &[T;N], mut f: impl FnMut(usize,&T) -> U) -> [U;N] {
    unsafe {
        let mut dest: ManuallyDrop<MaybeUninit<[U;N]>> = ManuallyDrop::new(MaybeUninit::uninit());
        for (i,(entry,src)) in (*(dest.as_mut_ptr() as *mut [MaybeUninit<U>;N])).iter_mut().zip(v).enumerate() {
            entry.write( f(i,src) );
        }
        ManuallyDrop::into_inner(dest).assume_init()
    }
}

#[inline]
pub(crate) fn trans_array_enumerated_mut<T,U,const N: usize>(v: &mut [T;N], mut f: impl FnMut(usize,&mut T) -> U) -> [U;N] {
    unsafe {
        let mut dest: ManuallyDrop<MaybeUninit<[U;N]>> = ManuallyDrop::new(MaybeUninit::uninit());
        for (i,(entry,src)) in (*(dest.as_mut_ptr() as *mut [MaybeUninit<U>;N])).iter_mut().zip(v).enumerate() {
            entry.write( f(i,src) );
        }
        ManuallyDrop::into_inner(dest).assume_init()
    }
}
