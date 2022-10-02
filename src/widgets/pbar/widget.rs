use crate::queron::Queron;
use crate::widget::cache::{StdRenderCachors, WidgetCache};
use crate::widget::dyn_tunnel::WidgetDyn;

use super::*;
use super::super::util::state::*;

impl<'w,E> Widget<E> for ProgressBar<'w,E> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
{
    type Cache = ProgressBarCache<E>;

    fn id(&self) -> E::WidgetID {
        self.id.clone()
    }
    
    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
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
                .with_vartype(
                    true,
                    true,
                    true,
                    false,
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

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> EventResp where P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        false
    }

    fn _size<P>(
        &self,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
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
    
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        Ok(vec![])
    }
    fn focusable(&self) -> bool {
        false
    }

    impl_traitcast!( dyn WidgetDyn<E>:
        dyn AtomState<E,f32> => |s| &s.value;
    );
}

pub fn crop(i: &Bounds, v: f32, o: Orientation) -> (u32,Bounds) {
    let (x, w) = i.par(o);
    let (y, h) = i.unpar(o);

    let w = ((w as f32) * v.clamp(0.0,1.0) ) as u32;

    (w,Bounds::from_ori(x, y, w, h, o))
}

impl<'z,E> AsWidget<'z,E> for ProgressBar<'z,E> where Self: Widget<E>, E: Env {
    type Widget<'v> = Self where 'z: 'v;
    type WidgetCache = <Self as Widget<E>>::Cache;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: dispatchor::AsWidgetDispatch<'z,Self,R,E>
    {
        f.call(self, root, ctx)
    }
}

#[derive(Default)]
pub struct ProgressBarCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>{
    std_render_cachors: Option<StdRenderCachors<E>>,
    intvalue_cachor: Option<u32>,
    _p: PhantomData<E>,
    //TODO cachor borders and colors
}

impl<E> WidgetCache<E> for ProgressBarCache<E> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E> {
    fn reset_current(&mut self) {}
}
