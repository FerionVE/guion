#![doc(html_logo_url="https://raw.githubusercontent.com/FerionVE/guion/430c18e7/res/icon.svg")]
#![doc(html_favicon_url="https://raw.githubusercontent.com/FerionVE/guion/8cc44b2e/res/favicon.svg")]

#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

//#![warn(clippy::all)]

pub mod widget_decl;

pub mod widget;
pub mod backend;
pub mod env;
pub mod render;
pub mod event; //TODO get rid of the old event
pub mod event_new;
pub mod style;
pub mod layout;
pub mod ctx;
pub mod intercept;
pub mod state;
pub mod util;
pub mod aliases;
pub mod text;
pub mod widgets;

pub mod root;
pub mod traitcast;
pub mod error;

pub mod compat;

pub mod cachialize;
pub mod queron;
pub mod newpath;
pub mod cachor;

pub mod invalidation;

pub type EventResp = bool;

#[cfg(feature = "qcell")]
pub use qcell;

#[doc(hidden)]
pub struct ProtectedReturn(std::marker::PhantomData<()>);
