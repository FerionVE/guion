use std::any::{TypeId, Any};
use std::marker::PhantomData;
use std::ops::BitOr;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStackDyn, PathResolvusDyn};
use crate::pathslice::{PathStack, NewPathStack, PathSliceRef};
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;

use self::dyn_tunnel::WidgetDeclDyn;
use self::imp::Erased;
use self::memoize::Memoize;
use self::mutor_trait::MutorEnd;
use self::route::UpdateRoute;
use self::zone::Zone;

pub mod mut_target;
pub mod mutor_trait;

pub mod imp;
pub mod dyn_tunnel;
pub mod route;

pub mod pane_childs;

pub mod sub;
pub mod memoize;
pub mod zone;

pub mod childs_macro;

pub trait WidgetDecl<E> where E: Env {
    type Widget: Widget<E> + 'static;

    fn send_mutation(
        &self,
        path: &mut NewPathStack,
        resolve: PathSliceRef,
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    );

    #[inline]
    fn build(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized {
        self.instantiate(path, root, ctx)
    }

    fn instantiate(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget;

    #[inline]
    fn build_boxed(self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized {
        self.instantiate_boxed(path, root, ctx)
    }

    // dyn flattening
    #[inline]
    fn instantiate_boxed(&self, path: &mut NewPathStack, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> {
        Box::new(self.instantiate(path, root, ctx))
    }

    // update to reconcile decl and widget (state)
    fn update(
        &self,
        w: &mut Self::Widget,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation;

    /// This function not to be called from outside
    fn update_restore(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation);

    fn update_restore_boxed(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &mut NewPathStack,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) {
        let (widget, vali) = self.update_restore(prev, path, root, ctx);
        (Box::new(widget), vali)
    }

    // dyn flattening
    #[inline]
    fn update_dyn(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &mut NewPathStack,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation {
        if TypeId::of::<Self::Widget>() == TypeId::of::<Box<dyn WidgetDyn<E> + 'static>>() {
            // Due to the flattening, downcast to Box<dyn Widget> isn't possible
            let w = unsafe {
                &mut *(w as *mut Box<dyn WidgetDyn<E> + 'static> as *mut Self::Widget)
            };
            return self.update(w, path, route, root, ctx);
        }

        //let w_inner = w.erase2_mut();

        if let Some(v) = w.downcast_mut::<Self::Widget>() {
            self.update(v, path, route, root, ctx)
        } else {
            let (new,vali) = self.update_restore_boxed(w, path, root, ctx);
            *w = new;
            vali
        }
    }

    fn erase(&self) -> &(dyn WidgetDeclDyn<E>+'_) where Self: Sized {
        self
    }

    fn into_erased<'a>(self) -> Box<(dyn WidgetDeclDyn<E>+'a)> where Self: Sized + 'a {
        Box::new(self)
    }

    #[inline]
    fn callback(self, mut v: DeclScope<'_,'_,Self::Widget,E>) -> DeclResult where Self: Sized {
        match v.command {
            WidgetDeclCallbackMode::Instantiate(dest) =>
                *dest = Some(self.build(&mut v.path, v.root, v.ctx)),
            WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
                *dest = Some(self.build_boxed(&mut v.path, v.root, v.ctx)),
            WidgetDeclCallbackMode::Update(widget, vali) =>
                *vali = self.update(widget, &mut v.path, v.route, v.root, v.ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget, vali) =>
                *vali = self.update_dyn(widget, &mut v.path, v.route, v.root, v.ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest, vali) => {
                let (d,v) = self.update_restore(prev, &mut v.path, v.root, v.ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest, vali) => {
                let (d,v) = self.update_restore_boxed(prev, &mut v.path, v.root, v.ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::SendMutation(resolve, args) => 
                self.send_mutation(&mut v.path, resolve, args, v.root, v.ctx),
            
        }
        DeclResult(PhantomData)
    }

    #[inline]
    fn call_on(&self, mut v: DeclScope<'_,'_,Self::Widget,E>) -> DeclResult {
        match v.command {
            WidgetDeclCallbackMode::Instantiate(dest) =>
                *dest = Some(self.build(&mut v.path, v.root, v.ctx)),
            WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
                *dest = Some(self.build_boxed(&mut v.path, v.root, v.ctx)),
            WidgetDeclCallbackMode::Update(widget, vali) =>
                *vali = self.update(widget, &mut v.path, v.route, v.root, v.ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget, vali) =>
                *vali = self.update_dyn(widget, &mut v.path, v.route, v.root, v.ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest, vali) => {
                let (d,v) = self.update_restore(prev, &mut v.path, v.root, v.ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest, vali) => {
                let (d,v) = self.update_restore_boxed(prev, &mut v.path, v.root, v.ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::SendMutation(resolve, args) => 
                self.send_mutation(&mut v.path, resolve, args, v.root, v.ctx),
        }
        DeclResult(PhantomData)
    }
}

pub struct DeclScope<'a,'b,W,E> where W: Widget<E> + 'static, E: Env, 'b: 'a {
    pub(crate) root: E::RootRef<'a>,
    pub(crate) ctx: &'a mut E::Context<'b>,
    pub(crate) path: NewPathStack<'a>,
    pub(crate) route: UpdateRoute<'a,E>,
    pub(crate) command: WidgetDeclCallbackMode<'a,W,E>,
    _p: PhantomData<&'a mut &'b mut ()>,
}

impl<'a,'b,W,E> DeclScope<'a,'b,W,E> where W: Widget<E> + 'static, E: Env, 'b: 'a {
    #[inline]
    pub fn root(&self) -> E::RootRef<'a> {
        self.root.fork()
    }
    #[inline]
    pub fn ctx(&mut self) -> &mut E::Context<'b> {
        self.ctx
    }
    #[inline]
    pub fn path(&mut self) -> &mut NewPathStack<'a> {
        &mut self.path
    }

    // pub fn sub<'s>(self) -> WidgetDeclCallback<'s,W,E> where 'a: 's {
    //     self
    // }

    pub fn new(root: E::RootRef<'a>, ctx: &'a mut E::Context<'b>, path: NewPathStack<'a>, route: UpdateRoute<'a,E>, command: WidgetDeclCallbackMode<'a,W,E>) -> Self {
        DeclScope {
            root,
            ctx,
            path,
            route,
            command,
            _p: PhantomData
        }
    }
}

pub enum WidgetDeclCallbackMode<'a,W,E> where W: Widget<E> + 'static, E: Env {
    Instantiate(&'a mut Option<W>),
    InstantiateBoxed(&'a mut Option<Box<dyn WidgetDyn<E> + 'static>>),
    Update(&'a mut W, &'a mut Invalidation),
    UpdateDyn(&'a mut Box<dyn WidgetDyn<E> + 'static>, &'a mut Invalidation),
    UpdateRestore(&'a mut dyn WidgetDyn<E>, &'a mut Option<W>, &'a mut Invalidation),
    UpdateRestoreBoxed(&'a mut dyn WidgetDyn<E>, &'a mut Option<Box<dyn WidgetDyn<E> + 'static>>, &'a mut Invalidation),
    SendMutation(PathSliceRef<'a>, &'a dyn Any),
}

pub struct DeclResult(PhantomData<()>);

// impl BitOr<Invalidation> for WidgetDeclCallbackResult {
//     type Output = Invalidation;

//     fn bitor(self, rhs: Invalidation) -> Self::Output {
//         self.0 | rhs
//     }
// }

pub type DeclScopeDyn<'a,'b,E> = DeclScope<'a,'b,Box<dyn WidgetDyn<E> + 'static>,E>;

pub trait WidgetDeclExt<E>: WidgetDecl<E> where E: Env {
    #[inline]
    fn callback_erased(self, v: DeclScope<'_,'_,Box<dyn WidgetDyn<E>+'static>,E>) -> DeclResult where Self: Sized {
        Erased(self).callback(v)
    }

    #[inline]
    fn call_on_erased(&self, v: DeclScope<'_,'_,Box<dyn WidgetDyn<E>+'static>,E>) -> DeclResult {
        Erased(self).callback(v)
    }

    #[inline]
    fn memoize_on<M>(self, memoize: M) -> Memoize<M,Self,E> where Self: Sized, M: Clone + PartialEq + 'static {
        Memoize::new(memoize, self)
    }

    #[inline]
    fn zone<Z>(self) -> Zone<Z,Self,E> where Self: Sized, Z: 'static {
        Zone::new(self)
    }
}

impl<E,T> WidgetDeclExt<E> for T where T: WidgetDecl<E>, E: Env {}
