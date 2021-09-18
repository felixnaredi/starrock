#[macro_use]
extern crate derive_builder;

mod background;
mod bullet;
mod bullet_renderer;
mod collision;
mod context;
mod dom;
mod foreground;
mod foreground_renderer;
mod gl;
mod keyboard_event_bus;
mod matrix;
mod rock;
mod run;
mod run_loop;
mod ship;

pub use run::run;
