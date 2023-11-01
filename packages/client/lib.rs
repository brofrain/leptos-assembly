#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(lazy_cell)]

#[macro_use]
extern crate common_macros;

mod env;
mod prelude;

mod app;
mod utils;

pub use crate::{app::App, utils::logger};