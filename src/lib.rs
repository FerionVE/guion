#![doc(html_logo_url="https://raw.githubusercontent.com/FerionVE/guion/430c18e7/res/icon.svg")]
#![doc(html_favicon_url="https://raw.githubusercontent.com/FerionVE/guion/8cc44b2e/res/favicon.svg")]

#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

//#![warn(clippy::all)]

pub mod widget;
pub mod backend;
pub mod env;
pub mod render;
pub mod event;
pub mod event_new;
pub mod style;
pub mod layout;
pub mod ctx;
pub mod handler;
pub mod state;
pub mod util;
pub mod aliases;
pub mod text;
pub mod widgets;

pub mod root;
pub mod traitcast;
pub mod view;
pub mod error;

pub mod compat;

pub mod dispatchor;
pub mod cachialize;
pub mod queron;
pub mod newpath;
pub mod cachor;

pub type EventResp = bool;

#[doc(hidden)]
pub struct ProtectedReturn(std::marker::PhantomData<()>);
