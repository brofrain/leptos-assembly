use std::fmt;

use common::{
    prelude::*,
    vendor::leptos_router::{NavigateOptions, Params},
};
use leptos::Params;

#[derive(Params, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct HiParams {
    name: Option<String>,
}

impl HiParams {
    pub const fn new(name: Option<String>) -> Self {
        Self { name }
    }
}

#[derive(Params, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct NotFoundParams {
    path: String,
}

pub enum Route {
    Home,
    About,
    Hi(HiParams),
    NotFound(NotFoundParams),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "/"),
            Self::About => write!(f, "/about"),
            Self::Hi(HiParams { name }) => {
                if let Some(name) = name {
                    write!(f, "/hi/{name}")
                } else {
                    write!(f, "/hi")
                }
            }
            Self::NotFound(NotFoundParams { path }) => write!(f, "/{path}"),
        }
    }
}

pub const HI_ORIGINAL_PATH: &str = "/hi";
pub const HI_NAME_ORIGINAL_PATH: &str = "/hi/:name";
pub const NOT_FOUND_ORIGINAL_PATH: &str = "/*path";

pub fn use_navigate() -> Callback<Route> {
    #[allow(clippy::disallowed_methods)]
    let navigate = leptos_router::use_navigate();
    Callback::new(move |route: Route| {
        navigate(&route.to_string(), NavigateOptions::default());
    })
}

pub fn use_navigate_with_options(options: NavigateOptions) -> Callback<Route> {
    #[allow(clippy::disallowed_methods)]
    let navigate = leptos_router::use_navigate();
    Callback::new(move |route: Route| {
        navigate(&route.to_string(), options.clone());
    })
}
