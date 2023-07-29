#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod lyra_app;
mod style;
pub use lyra_app::LyraApp;
