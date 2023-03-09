use std::any::Any;
use std::ops::Range;

use crate::aliases::{ERenderer, EEvent, ETextLayout, ESize, EStyle};
use crate::cachor::AsCachor;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::event_new;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::queron::Queron;
use crate::render::{StdRenderProps, TestStyleColorType};
use crate::render::widgets::RenderStdWidgets;
use crate::text::layout::{TxtLayoutFromStor, TxtLayout};
use crate::text::stor::TextStor;
use crate::util::bounds::{Bounds, Dims};
use crate::util::tabulate::{TabulateOrigin, TabulateDirection, TabulateResponse};
use crate::widget::Widget;
use crate::widget::cache::{StdRenderCachors, ValidationStat};
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget::id::WidgetID;

pub struct Label<E,Text> where
    E: Env,
{
    pub(super) id: WidgetID,
    pub(super) size: ESize<E>,
    pub(super) style: EStyle<E>,
    pub(super) text: Text,
    pub(super) text_cache: Option<ETextLayout<E>>,
    pub(super) rendered_dims: Option<Dims>,
    pub(super) align: (f32,f32),
}

impl<E,Text> Widget<E> for Label<E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E> + AsCachor<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    type Cache = ();

    
    
    fn _render<P,Ph>(
        &mut self,
        _path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        mut force_render: bool,
        cache: &mut Self::Cache,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        let mut need_render = force_render;

        // StdRenderCachors::current(stack)
        //     .validate(&mut cache.std_render_cachors, &mut need_render, &mut force_render);

        //TODO cachor align and style stuff e.g. bg color
        //TODO text layout cachors
        self.validate(stack, ctx);

        let render_props = StdRenderProps::new(&stack);

        need_render |= self.rendered_dims != Some(render_props.absolute_bounds.size);

        if !need_render {return;}

        let text_layout = self.text_cache.as_ref().unwrap();

        renderer.fill_rect(
            &render_props
                .with_style_color_type(TestStyleColorType::Bg),
            ctx
        );

        //TODO way to inject props/style to widget
        renderer.render_preprocessed_text(
            text_layout,
            Default::default(),
            &StdRenderProps::new(&stack)
                .inner_aligned(text_layout.display_size(),self.align),
            ctx
        );

        self.rendered_dims = Some(render_props.absolute_bounds.size);
    }

    fn _event_direct<P,Ph,Evt>(
        &mut self,
        _: &Ph,
        _: &P,
        _: &Evt,
        _: Option<&(dyn PathResolvusDyn<E>+'_)>,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized {
        Invalidation::valid()
    }

    fn _size<P,Ph>(
        &mut self,
        _path: &Ph,
        stack: &P,
        _root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        self.validate(stack, ctx);

        let ms = self.text_cache.as_ref().unwrap().display_size();
        let ms = ESize::<E>::fixed(ms.w, ms.h);
        ms.max( &self.size )
    }

    fn childs(&self) -> Range<isize> {
        0..0
    }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        _: isize,
        _: &Ph,
        _: &P,
        _: TabulateOrigin<E>,
        _: TabulateDirection,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
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
        //r.try_respond::<dyn AsCachor<E>>(#[inline] || &self.test) ||
        r.try_respond::<dyn TextStor<E>>(#[inline] || &self.text);
    }

    fn id(&self) -> WidgetID {
        self.id
    }

    fn update<Ph>(
        &mut self,
        _: &Ph,
        _: crate::widget_decl::route::UpdateRoute<'_,E>,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        Invalidation::valid()
    }

    fn child_dyn(&self, _: isize) -> Option<crate::widget::WidgetChildDynResult<'_,E>> {
        None
    }

    fn child_dyn_mut(&mut self, _: isize) -> Option<crate::widget::WidgetChildDynResultMut<'_,E>> {
        None
    }

    fn childs_dyn<'a,F>(&'a self, _: Range<isize>, _: F) where F: FnMut(crate::widget::WidgetChildDynResult<'a,E>) {}

    fn childs_dyn_mut<'a,F>(&'a mut self, _: Range<isize>, _: F) where F: FnMut(crate::widget::WidgetChildDynResultMut<'a,E>) {}

    fn resolve_child_dyn<'a,'b>(&'a self, _: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<crate::widget::WidgetChildResolveDynResult<'a,'b,E>> {
        None
    }

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, _: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<crate::widget::WidgetChildResolveDynResultMut<'a,'b,E>> {
        None
    }

    fn send_mutation<Ph>(
        &self,
        _: &Ph,
        _: &(dyn PathResolvusDyn<E>+'_),
        _: &dyn Any,
        _: E::RootRef<'_>,
        _: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {}

    #[inline]
    fn invalidate_recursive(&mut self, vali: Invalidation) {
        if vali.render | vali.layout {
            self.rendered_dims = None;
        }
    }

    fn respond_query_mut<'a>(&'a mut self, _: crate::traitcast::WQueryResponder<'_,'a,E>) {}
}

impl<E,Text> Label<E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E> + AsCachor<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    fn validate(&mut self, stack: &(impl Queron<E> + ?Sized), ctx: &mut E::Context<'_>) {
        //TODO also cachor e.g. style that affects text
        // if cache.text_cachor.is_none() || cache.text_cache.is_none() || !self.text.valid(cache.text_cachor.as_ref().unwrap()) { //TODO old Validation trait bad coercion
        //     cache.text_cachor = Some(self.text.cachor());
        //     cache.text_cache = Some(TxtLayoutFromStor::from(&self.text,ctx));
        //     cache.text_rendered = false;
        // }
        // ValidationStat::from_valid(cache.text_rendered)

        if self.text_cache.is_none() {
            self.text_cache = Some(TxtLayoutFromStor::from(&self.text,ctx));
            self.rendered_dims = None;
        }
    }
}

// impl<E,Text> AsWidget<E> for Label<E,Text> where Self: Widget<E>, E: Env {
//     type Widget<'v,'z> = Self where 'z: 'v, Self: 'z;
//     type WidgetCache = <Self as Widget<E>>::Cache;

//     #[inline]
//     fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
//     where
//         Self: 'w
//     {
//         f.call(self, root, ctx)
//     }
// }

// pub struct LabelCache<E,TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
//     text_cache: Option<ETextLayout<E>>,
//     text_cachor: Option<TC>,
//     text_rendered: bool,
//     std_render_cachors: Option<StdRenderCachors<E>>,
//     align_cachor: Option<(f32,f32)>,
//     //render_style_cachor: Option<<ERenderer<'_,E> as RenderStdWidgets<E>>::RenderPreprocessedTextStyleCachors>,
// }

// impl<E, TC> Default for LabelCache<E, TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
//     #[inline]
//     fn default() -> Self {
//         Self {
//             text_cache: None,
//             text_cachor: None,
//             text_rendered: false,
//             std_render_cachors: None,
//             align_cachor: None
//         }
//     }
// }

// impl<E,TC> RenderCache<E> for LabelCache<E,TC> where E: Env, for<'r> ERenderer<'r,E>: RenderStdWidgets<E>, TC: Clone + PartialEq + 'static {
//     fn reset_current(&mut self) {}
// }
