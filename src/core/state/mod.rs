use crate::core::render::widgets::RenderStdWidgets;
use crate::core::state::handler::AsHandlerStateful;
use crate::core::ctx::aliases::ECHandler;
use crate::core::ctx::Env;

pub mod handler;

pub trait EnvStateful: Env where ECHandler<Self>: AsHandlerStateful<Self>, Self::Renderer: RenderStdWidgets<Self> {

}

impl<T> EnvStateful for T where Self: Env, ECHandler<Self>: AsHandlerStateful<Self>, Self::Renderer: RenderStdWidgets<Self> {

}