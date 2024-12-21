#![warn(clippy::all, rust_2018_idioms)]

mod game_app;
mod game;
mod sparse;

mod template_app;

//pub use template_app::TemplateApp;
pub use game_app::GameApp as TemplateApp;

pub use rand_xoshiro::Xoshiro128PlusPlus as Rand;