use crate::aliases::{EStyle, ESize, ERenderer, EEvent, ETextLayout};
use crate::cachor::AsCachor;
use crate::ctx::Context;
use crate::env::Env;
use crate::event::imp::StdVarSup;
use crate::invalidation::Invalidation;
use crate::layout::Gonstraints;
use crate::newpath::{PathStack, PathResolvusDyn};
use crate::render::widgets::RenderStdWidgets;
use crate::text::layout::TxtLayoutFromStor;
use crate::text::stor::TextStor;
use crate::widget::dyn_tunnel::WidgetDyn;
use crate::widget_decl::WidgetDecl;
use crate::widget_decl::route::UpdateRoute;

pub struct Label<E,Text> where
    E: Env,
{
    pub(super) size: Option<ESize<E>>,
    pub(super) style: Option<EStyle<E>>,
    pub(super) text: Text,
    pub(super) align: Option<(f32,f32)>,
}

impl<E,Text> WidgetDecl<E> for Label<E,Text> where
    E: Env,
    for<'r> ERenderer<'r,E>: RenderStdWidgets<E>,
    EEvent<E>: StdVarSup<E>,
    Text: TextStor<E> + AsCachor<E>,
    ETextLayout<E>: TxtLayoutFromStor<Text,E>,
{
    type Widget = super::widget::Label<E,String>;

    fn send_mutation<Ph>(
        &self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn std::any::Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {}

    fn instantiate<Ph>(&self, path: &Ph, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Self::Widget where Ph: PathStack<E> + ?Sized {
        super::widget::Label {
            id: ctx.retained_id(),
            size: self.size.clone().unwrap_or(ESize::<E>::zero()),
            style: self.style.clone().unwrap_or_default(),
            text: self.text.caption().into_owned(),
            align: self.align.unwrap_or((0.5,0.5)),
            text_cache: None,
            rendered_dims: None,
        }
    }

    fn update<Ph>(
        &self,
        w: &mut Self::Widget,
        path: &Ph,
        route: UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        let mut vali = Invalidation::valid();

        if self.text.caption() != w.text {
            w.text = self.text.caption().into_owned();
            w.text_cache = None; w.rendered_dims = None; vali.relayout();
        }
        if self.align.as_ref().map_or(false, |&v| align_compare(v, w.align) ) {
            w.align = self.align.unwrap();
            w.text_cache = None; w.rendered_dims = None; vali.relayout();
        }

        vali
    }

    fn update_restore<Ph>(
        &self,
        prev: &mut dyn WidgetDyn<E>,
        path: &Ph,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> (Self::Widget,Invalidation) where Ph: PathStack<E> + ?Sized {
        (self.instantiate(path, root, ctx), Invalidation::new())
    }
}

fn align_compare(a: (f32,f32), b: (f32,f32)) -> bool {
    sub_float_cmp(a.0, b.0)
}

fn sub_float_cmp(a: f32, b: f32) -> bool {
    ((a * 65536.).round() as isize) == ((b * 65536.).round() as isize)
}
