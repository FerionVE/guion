use crate::core::render::widgets::RenderStdWidgets;
use crate::core::ctx::aliases::ECStateful;
use crate::core::ctx::handler::access::AsHandler;
use crate::core::state::handler::AsHandlerStateful;
use crate::core::ctx::aliases::ECHLink;
use crate::core::ctx::Env;

pub mod handler;

pub trait EnvStateful: Env where ECHLink<Self>: AsHandlerStateful<Self,Self::Context>, Self::Renderer: RenderStdWidgets<Self> {

}

impl<T> EnvStateful for T where Self: Env, ECHLink<Self>: AsHandlerStateful<Self,Self::Context>, Self::Renderer: RenderStdWidgets<Self> {

}