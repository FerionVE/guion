use std::any::{TypeId, Any};
use std::marker::PhantomData;
use std::ops::BitOr;

use crate::env::Env;
use crate::invalidation::Invalidation;
use crate::newpath::{PathStack, PathStackDyn, PathResolvusDyn};
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

pub trait WidgetDecl<E> where E: Env {
    type Widget: Widget<E> + 'static;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized;

    #[inline]
    fn build<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.instantiate(path, root, ctx)
    }

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized;

    #[inline]
    fn build_boxed<Ph>(self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Self: Sized, Ph: PathStack<E> + ?Sized {
        self.instantiate_boxed(path, root, ctx)
    }

    // dyn flattening
    #[inline]
    fn instantiate_boxed<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        Box::new(self.instantiate(path, root, ctx))
    }

    // update to reconcile decl and widget (state)
    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation where Ph: PathStack<E> + ?Sized;

    /// This function not to be called from outside
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) where Ph: PathStack<E> + ?Sized;

    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Box<dyn WidgetDyn<E> + 'static>,Invalidation) where Ph: PathStack<E> + ?Sized {
        let (widget, vali) = self.update_restore(prev, path, root, ctx);
        (Box::new(widget), vali)
    }

    // dyn flattening
    #[inline]
    fn update_dyn<Ph>(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
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

    #[inline]
    fn callback(self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult where Self: Sized {
        match v.command {
            WidgetDeclCallbackMode::Instantiate(dest) =>
                *dest = Some(self.build(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
                *dest = Some(self.build_boxed(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::Update(widget, vali) =>
                *vali = self.update(widget, v.path, v.route, v.root, ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget, vali) =>
                *vali = self.update_dyn(widget, v.path, v.route, v.root, ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest, vali) => {
                let (d,v) = self.update_restore(prev, v.path, v.root, ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest, vali) => {
                let (d,v) = self.update_restore_boxed(prev, v.path, v.root, ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::SendMutation(resolve, args) => 
                self.send_mutation(v.path, resolve, args, v.root, ctx),
            
        }
        WidgetDeclCallbackResult(PhantomData)
    }

    #[inline]
    fn call_on(&self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult {
        match v.command {
            WidgetDeclCallbackMode::Instantiate(dest) =>
                *dest = Some(self.build(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
                *dest = Some(self.build_boxed(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::Update(widget, vali) =>
                *vali = self.update(widget, v.path, v.route, v.root, ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget, vali) =>
                *vali = self.update_dyn(widget, v.path, v.route, v.root, ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest, vali) => {
                let (d,v) = self.update_restore(prev, v.path, v.root, ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest, vali) => {
                let (d,v) = self.update_restore_boxed(prev, v.path, v.root, ctx);
                *dest = Some(d); *vali = v;
            },
            WidgetDeclCallbackMode::SendMutation(resolve, args) => 
                self.send_mutation(v.path, resolve, args, v.root, ctx),
        }
        WidgetDeclCallbackResult(PhantomData)
    }
}

pub struct WidgetDeclCallback<'a,W,E> where W: Widget<E> + 'static, E: Env {
    root: E::RootRef<'a>,
    pub(crate) path: &'a (dyn PathStackDyn<E> + 'a),
    pub(crate) route: UpdateRoute<'a,E>,
    pub(crate) command: WidgetDeclCallbackMode<'a,W,E>,
    _p: PhantomData<&'a mut ()>,
}

impl<'a,W,E> WidgetDeclCallback<'a,W,E> where W: Widget<E> + 'static, E: Env {
    #[inline]
    pub fn root(&self) -> E::RootRef<'a> {
        self.root.fork()
    }
    #[inline]
    pub fn path(&self) -> &'a (dyn PathStackDyn<E> + 'a) {
        self.path
    }

    // pub fn sub<'s>(self) -> WidgetDeclCallback<'s,W,E> where 'a: 's {
    //     self
    // }

    pub fn new(root: E::RootRef<'a>, path: &'a (dyn PathStackDyn<E> + 'a), route: UpdateRoute<'a,E>, command: WidgetDeclCallbackMode<'a,W,E>) -> Self {
        WidgetDeclCallback {
            root,
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
    SendMutation(&'a (dyn PathResolvusDyn<E>+'a), &'a dyn Any),
}

pub struct WidgetDeclCallbackResult(PhantomData<()>);

// impl BitOr<Invalidation> for WidgetDeclCallbackResult {
//     type Output = Invalidation;

//     fn bitor(self, rhs: Invalidation) -> Self::Output {
//         self.0 | rhs
//     }
// }

pub type WidgetDeclCallbackDyn<'a,E> = WidgetDeclCallback<'a,Box<dyn WidgetDyn<E> + 'static>,E>;

pub trait WidgetDeclExt<E>: WidgetDecl<E> where E: Env {
    #[inline]
    fn callback_erased(self, v: WidgetDeclCallback<'_,Box<dyn WidgetDyn<E>+'static>,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult where Self: Sized {
        Erased(self).callback(v, ctx)
    }

    #[inline]
    fn call_on_erased(&self, v: WidgetDeclCallback<'_,Box<dyn WidgetDyn<E>+'static>,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult {
        Erased(self).callback(v, ctx)
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