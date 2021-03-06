//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod app2;
pub use app2::Headlines as TemplateApp2;

#[cfg(feature = "default")]
pub mod iv_platform;

#[cfg(feature = "use_sdl2")]
pub mod sdl2_platform;
