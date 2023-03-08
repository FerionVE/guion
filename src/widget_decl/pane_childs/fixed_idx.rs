use std::any::Any;
use std::mem::{MaybeUninit, ManuallyDrop};
use std::ops::Range;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{FixedIdx, PathFragment, PathResolvus, PathStack, PathResolvusDyn};
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

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
            if r2.0 >= 0 && (r2.0 as usize) < self.0.len() {
                self.0[r2.0 as usize].send_mutation(&r2.push_on_stack(path), resolve.inner().unwrap(), args, root, ctx)
            }
        } else {
            //TODO what happens if the mutor is lost
        }
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        WidgetsFixedIdx(
            self.0.iter()
                .enumerate()
                .map(|(idx,decl)|
                    PaneChildWidget::new( decl.instantiate(&FixedIdx(idx as isize).push_on_stack(path), root.fork(), ctx) )
                )
                .collect()
        )
    }

    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        // If resolve, try only update resolve scope
        if let Some(resolve) = route.resolving() {
            if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
                if r2.0 >= 0 && (r2.0 as usize) < self.0.len().min(w.0.len()) {
                    return self.0[r2.0 as usize].update(&mut w.0[r2.0 as usize].widget, &r2.push_on_stack(path), route.for_child_1(), root, ctx);
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
                w.widget.end(&FixedIdx((self.0.len() + idx) as isize).push_on_stack(path), root.fork(), ctx);
                vali = Invalidation::new();
            }
        }
        // Update persisted exising area
        for (idx,(d,w)) in self.0.iter().zip(w.0.iter_mut()).enumerate() {
            vali |= d.update(&mut w.widget, &FixedIdx(idx as isize).push_on_stack(path), route.for_child_1(), root.fork(), ctx)
        }
        // Instantiate new tail
        if self.0.len() > w.0.len() {
            for (idx,d) in self.0[w.0.len()..].iter().enumerate() {
                let path = FixedIdx((w.0.len() + idx) as isize).push_on_stack(path);
                w.0.push(PaneChildWidget::new( d.instantiate(&path, root.fork(), ctx) ));
                vali = Invalidation::new();
            }
        }
        assert_eq!(self.0.len(), w.0.len());

        vali
    }

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        let prev_len = prev.len();
        
        let mut vali = Invalidation::valid();

        // Remove old tail
        if prev_len > self.0.len() {
            if end_range_dyn(prev, self.0.len() .. prev_len, path, root.fork(), ctx) != 0 {
                vali = Invalidation::new();
            }
        }

        let mut new = Vec::with_capacity(self.0.len());
        
        let restorable = prev_len.min(self.0.len());

        // Restore-update persisted exising area
        prev.idx_range_dyn_mut(0 .. restorable, &mut |result| {
            let path = result.child_id.push_on_stack(path);
            let d = &self.0[result.idx as usize];
            let (w,v) = d.update_restore(result.widget, &path, root.fork(), ctx);
            let mut w = PaneChildWidget::new(w);
            w.vali |= v;
            new.push(w);
            vali |= v;
        });

        // Instantiate new tail
        if self.0.len() > restorable {
            for (idx,d) in self.0[restorable..].iter().enumerate() {
                let path = FixedIdx((restorable + idx) as isize).push_on_stack(path);
                new.push(PaneChildWidget::new( d.instantiate(&path, root.fork(), ctx) ));
                vali = Invalidation::new();
            }
        }

        assert_eq!(self.0.len(), new.len());

        WidgetsFixedIdx(new)
    }
}

impl<E,T,const N: usize> PaneChildsDecl<E> for WidgetsFixedIdx<[T;N]> where T: WidgetDecl<E>, E: Env {
    type Retained = WidgetsFixedIdx<[PaneChildWidget<T::Widget,E>;N]>;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
            if r2.0 >= 0 && (r2.0 as usize) < self.0.len() {
                self.0[r2.0 as usize].send_mutation(&r2.push_on_stack(path), resolve.inner().unwrap(), args, root, ctx)
            }
        } else {
            //TODO what happens if the mutor is lost
        }
    }

    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Self: Sized, Ph: PathStack<E> + ?Sized {
        WidgetsFixedIdx(
            trans_array_enumerated(self.0, |idx,decl|
                PaneChildWidget::new( decl.build(&FixedIdx(idx as isize).push_on_stack(path), root.fork(), ctx) )
            )
        )
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        WidgetsFixedIdx(
            trans_array_enumerated_ref(&self.0, |idx,decl|
                PaneChildWidget::new( decl.build(&FixedIdx(idx as isize).push_on_stack(path), root.fork(), ctx) )
            )
        )
    }

    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        // If resolve, try only update resolve scope
        if let Some(resolve) = route.resolving() {
            if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
                if r2.0 >= 0 && (r2.0 as usize) < self.0.len().min(w.0.len()) {
                    let v = self.0[r2.0 as usize].update(&mut w.0[r2.0 as usize].widget, &r2.push_on_stack(path), route.for_child_1(), root, ctx);
                    w.0[r2.0 as usize].vali |= v;
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
            vali |= d.update(&mut w.widget, &FixedIdx(idx as isize).push_on_stack(path), route.for_child_1(), root.fork(), ctx)
        }

        vali
    }

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        let prev_len = prev.len();

        let mut vali = Invalidation::valid();

        // Remove old tail
        if prev_len > N {
            if end_range_dyn(prev, N .. prev_len, path, root.fork(), ctx) != 0 {
                vali = Invalidation::new();
            }
        }

        let mut new: ManuallyDrop<MaybeUninit<[PaneChildWidget<T::Widget,E>;N]>> = ManuallyDrop::new(MaybeUninit::uninit());

        let new_mut = unsafe { &mut *(new.as_mut_ptr() as *mut [MaybeUninit<PaneChildWidget<T::Widget,E>>;N]) };

        let mut add_pos = 0;
        
        let restorable = prev_len.min(self.0.len());

        // Restore-update persisted exising area
        prev.idx_range_dyn_mut(0 .. restorable, &mut |result| {
            debug_assert_eq!(result.idx as usize, add_pos);
            let path = result.child_id.push_on_stack(path);
            let d = &self.0[result.idx as usize];
            let (w,v) = d.update_restore(result.widget, &path, root.fork(), ctx);
            let mut w = PaneChildWidget::new(w);
            w.vali |= v;
            new_mut[add_pos].write(w);
            add_pos += 1;
            vali |= v;
        });

        assert_eq!(add_pos, restorable);

        // Instantiate new tail
        if N > restorable {
            for d in &self.0[restorable..] {
                let path = FixedIdx(add_pos as isize).push_on_stack(path);
                new_mut[add_pos].write(PaneChildWidget::new( d.instantiate(&path, root.fork(), ctx) ));
                add_pos += 1;
                vali |= Invalidation::new();
            }
        }

        assert_eq!(add_pos, N);

        WidgetsFixedIdx(unsafe { ManuallyDrop::into_inner(new).assume_init() })
    }
}

impl<E,T,const N: usize> PaneChildsDecl<E> for WidgetsFixedIdx<&[T;N]> where T: WidgetDecl<E>, E: Env {
    type Retained = WidgetsFixedIdx<[PaneChildWidget<T::Widget,E>;N]>;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        (*bender(self)).send_mutation(path, resolve, args, root, ctx)
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (*bender(self)).instantiate(path, root, ctx)
    }

    fn update<Ph>(
        &self,
        w: &mut Self::Retained,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        (*bender(self)).update(w, path, route, root, ctx)
    }

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn PaneChildsDyn<E,ChildID=<Self::Retained as PaneChildsDyn<E>>::ChildID>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Retained where Ph: PathStack<E> + ?Sized {
        (*bender(self)).update_restore(prev, path, root, ctx)
    }
}

#[inline]
fn end_range_dyn<Ph,CID,E>(w: &mut (dyn PaneChildsDyn<E,ChildID=CID> + '_), range: Range<usize>, parent_path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> usize
where
    Ph: PathStack<E> + ?Sized,
    CID: PathFragment<E> + Clone + 'static,
    E: Env,
{
    let mut n = 0;
    w.idx_range_dyn_mut(range, &mut |result| {
        result.widget.end(&result.child_id.push_on_stack(parent_path), root.fork(), ctx);
        n += 1;
    });
    n
}

#[inline]
fn bender<'a,'b,T>(v: &'a WidgetsFixedIdx<&'b T>) -> &'a WidgetsFixedIdx<T> where 'b: 'a, T: 'b + Sized {
    unsafe{std::mem::transmute(v)}
}

#[inline]
fn trans_array_enumerated<T,U,const N: usize>(v: [T;N], mut f: impl FnMut(usize,T) -> U) -> [U;N] {
    unsafe {
        let mut dest: ManuallyDrop<MaybeUninit<[U;N]>> = ManuallyDrop::new(MaybeUninit::uninit());
        for (i,(entry,src)) in (*(dest.as_mut_ptr() as *mut [MaybeUninit<U>;N])).iter_mut().zip(v).enumerate() {
            entry.write( f(i,src) );
        }
        ManuallyDrop::into_inner(dest).assume_init()
    }
}

#[inline]
fn trans_array_enumerated_ref<T,U,const N: usize>(v: &[T;N], mut f: impl FnMut(usize,&T) -> U) -> [U;N] {
    unsafe {
        let mut dest: ManuallyDrop<MaybeUninit<[U;N]>> = ManuallyDrop::new(MaybeUninit::uninit());
        for (i,(entry,src)) in (*(dest.as_mut_ptr() as *mut [MaybeUninit<U>;N])).iter_mut().zip(v).enumerate() {
            entry.write( f(i,src) );
        }
        ManuallyDrop::into_inner(dest).assume_init()
    }
}
