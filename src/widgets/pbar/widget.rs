use std::marker::PhantomData;

use crate::dispatchor::AsWidgetDispatch;
use crate::layout::Orientation;
use crate::util::bounds::Bounds;
use crate::widget::as_widget::AsWidget;
use crate::widget::cache::{WidgetCache, StdRenderCachors};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widgets::util::state::AtomState;
use crate::{EventResp, event_new};
use crate::aliases::{ERenderer, ESize};
use crate::env::Env;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType, TestStyleBorderType, TestStyleVariant};
use crate::render::widgets::RenderStdWidgets;
use crate::util::tabulate::{TabulateOrigin, TabulateDirection, TabulateResponse};
use crate::widget::{Widget, WidgetWithResolveChildDyn};

use super::ProgressBar;

impl<E> Widget<E> for ProgressBar<E> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
{
    type Cache = ProgressBarCache<E>;
    
    fn _render<P,Ph>(
        &self,
        _path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        let render_props = StdRenderProps::new(stack);

        render_props.current_std_render_cachors()
            .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

        let (intvalue,progress_bounds) = crop(&render_props.inside_spacing_border().absolute_bounds, self.value, self.orientation);

        if cache.intvalue_cachor != Some(intvalue) {
            need_render = true;
            cache.intvalue_cachor = Some(intvalue);
        }

        if !need_render {return;}

        renderer.fill_rect(
            &render_props
                .with_style_color_type(TestStyleColorType::Bg),
            ctx
        );
        
        renderer.fill_rect(
            &render_props
                .inside_spacing_border()
                .slice_absolute(progress_bounds)
                .with_style_color_type(TestStyleColorType::Fg) //TODO yes, stupid test style doesn't have ObjActive
                .with_style_type(
                    TestStyleVariant {
                        activated: true,
                        ..Default::default()
                    }
                ),
            ctx
        );

        renderer.fill_border_inner(
            &render_props
                .inside_spacing_border()
                .with_style_border_type(TestStyleBorderType::Component)
                .with_style_color_type(TestStyleColorType::Border),
            ctx
        );
    }

    fn _event_direct<P,Ph,Evt>(
        &self,
        _: &Ph,
        _: &P,
        _: &Evt,
        _: Option<&(dyn PathResolvusDyn<E>+'_)>,
        _: &mut Self::Cache,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> EventResp where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        false
    }

    fn _size<P,Ph>(
        &self,
        path: &Ph,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.size.clone() //TODO shouldn't the borders be added?
    }

    fn childs(&self) -> usize {
        0
    }
    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'www,'ww,'c,'cc> FnOnce(Result<&'www (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        (callback)(Err(()),ctx)
    }

    fn with_resolve_child<'s,F,R>(
        &'s self,
        sub_path: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'a,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'a,E>,E::Error>,&'c mut E::Context<'cc>) -> R
    {
        (callback)(Err(todo!()),ctx)
    }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: usize,
        path: &Ph,
        stack: &P,
        op: TabulateOrigin<E>,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        Err(todo!())
    }
    
    // fn child_bounds<P,Ph>(&self, path: &Ph,
    //     stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
    //     Ok(vec![])
    // }
    fn focusable(&self) -> bool {
        false
    }

    #[inline]
    fn respond_query<'a>(&'a self, mut r: crate::traitcast::WQueryResponder<'_,'a,E>) {
        r.try_respond::<dyn AtomState<E,f32>>(#[inline] || &self.value);
    }
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> (u32,Bounds) {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.clamp(0.0,1.0) ) as u32;

    (w,Bounds::from_ori(x, y, w, h, o))
}

impl<E> AsWidget<E> for ProgressBar<E> where Self: Widget<E>, E: Env {
    type Widget<'v,'z> = Self where 'z: 'v, Self: 'z;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct ProgressBarCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    std_render_cachors: Option<StdRenderCachors<E>>,
    intvalue_cachor: Option<u32>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<E> WidgetCache<E> for ProgressBarCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    fn reset_current(&mut self) {}
}
