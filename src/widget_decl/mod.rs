use std::any::TypeId;
use std::marker::PhantomData;

use crate::env::Env;
use crate::newpath::{PathResolvusDyn, PathStack, PathStackDyn};
use crate::root::RootRef;
use crate::widget::Widget;
use crate::widget::dyn_tunnel::WidgetDyn;

use self::imp::Erased;
use self::memoize::Memoize;

pub mod mut_target;
pub mod mutor_trait;

pub mod imp;

pub mod dyn_tunnel;

pub mod sub;

pub mod decl_list;

pub mod memoize;

pub trait WidgetDecl<E> where E: Env {
    type Widget: Widget<E> + 'static;

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
        resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized;

    /// This function not to be called from outside
    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        //resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Self::Widget where Ph: PathStack<E> + ?Sized;

    fn update_restore_boxed<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        //resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Box<dyn WidgetDyn<E> + 'static> where Ph: PathStack<E> + ?Sized {
        Box::new(self.update_restore(prev, path, root, ctx))
    }

    // dyn flattening
    #[inline]
    fn update_dyn<Ph>(
        &self,
        w: &mut Box<dyn WidgetDyn<E> + 'static>,
        path: &Ph,
        resolve: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized {
        if TypeId::of::<Self::Widget>() == TypeId::of::<Box<dyn WidgetDyn<E> + 'static>>() {
            // Due to the flattening, downcast to Box<dyn Widget> isn't possible
            let w = unsafe {
                &mut *(w as *mut Box<dyn WidgetDyn<E> + 'static> as *mut Self::Widget)
            };
            return self.update(w, path, resolve, root, ctx);
        }

        //let w_inner = w.erase2_mut();

        if let Some(v) = w.as_any_mut().downcast_mut::<Self::Widget>() {
            self.update(v, path, resolve, root, ctx);
        } else {
            let new = self.update_restore_boxed(w, path, root, ctx);
            *w = new;
        }
    }

    #[inline]
    fn callback(self, v: WidgetDeclCallback<'_,Self::Widget,E>, ctx: &mut E::Context<'_>) -> WidgetDeclCallbackResult where Self: Sized {
        match v.command {
            WidgetDeclCallbackMode::Instantiate(dest) =>
                *dest = Some(self.build(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::InstantiateBoxed(dest) =>
                *dest = Some(self.build_boxed(v.path, v.root, ctx)),
            WidgetDeclCallbackMode::Update(widget) =>
                self.update(widget, v.path, v.resolve, v.root, ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget) =>
                self.update_dyn(widget, v.path, v.resolve, v.root, ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest) =>
                *dest = Some(self.update_restore(prev, v.path, v.root, ctx)),
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest) =>
                *dest = Some(self.update_restore_boxed(prev, v.path, v.root, ctx)),
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
            WidgetDeclCallbackMode::Update(widget) =>
                self.update(widget, v.path, v.resolve, v.root, ctx),
            WidgetDeclCallbackMode::UpdateDyn(widget) =>
                self.update_dyn(widget, v.path, v.resolve, v.root, ctx),
            WidgetDeclCallbackMode::UpdateRestore(prev, dest) =>
                *dest = Some(self.update_restore(prev, v.path, v.root, ctx)),
            WidgetDeclCallbackMode::UpdateRestoreBoxed(prev, dest) =>
                *dest = Some(self.update_restore_boxed(prev, v.path, v.root, ctx)),
        }
        WidgetDeclCallbackResult(PhantomData)
    }
}

pub struct WidgetDeclCallback<'a,W,E> where W: Widget<E> + 'static, E: Env {
    pub(crate) root: E::RootRef<'a>,
    pub(crate) path: &'a (dyn PathStackDyn<E> + 'a),
    pub(crate) resolve: Option<&'a (dyn PathResolvusDyn<E> + 'a)>,
    pub(crate) command: WidgetDeclCallbackMode<'a,W,E>,
}

impl<'a,W,E> WidgetDeclCallback<'a,W,E> where W: Widget<E> + 'static, E: Env {
    #[inline]
    pub fn root(&self) -> E::RootRef<'a>{
        self.root.fork()
    }
    #[inline]
    pub fn path(&self) -> &'a (dyn PathStackDyn<E> + 'a) {
        self.path
    }
}

pub enum WidgetDeclCallbackMode<'a,W,E> where W: Widget<E> + 'static, E: Env {
    Instantiate(&'a mut Option<W>),
    InstantiateBoxed(&'a mut Option<Box<dyn WidgetDyn<E> + 'static>>),
    Update(&'a mut W),
    UpdateDyn(&'a mut Box<dyn WidgetDyn<E> + 'static>),
    UpdateRestore(&'a mut dyn WidgetDyn<E>, &'a mut Option<W>),
    UpdateRestoreBoxed(&'a mut dyn WidgetDyn<E>, &'a mut Option<Box<dyn WidgetDyn<E> + 'static>>),
}

pub struct WidgetDeclCallbackResult(std::marker::PhantomData<()>);

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
}

impl<E,T> WidgetDeclExt<E> for T where T: WidgetDecl<E>, E: Env {}
