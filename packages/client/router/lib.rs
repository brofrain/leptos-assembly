use std::fmt;

use common::{
    prelude::*,
    vendor::leptos_router::{NavigateOptions, Params},
};
use leptos::Params;

#[derive(Params, PartialEq)]
pub struct HiParams {
    pub name: String,
}

#[derive(Params, PartialEq)]
pub struct NotFoundParams {
    pub path: String,
}

pub enum Route {
    Home,
    About,
    Hi(Option<HiParams>),
    NotFound(NotFoundParams),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "/"),
            Self::About => write!(f, "/about"),
            Self::Hi(None) => write!(f, "/hi"),
            Self::Hi(Some(HiParams { name })) => write!(f, "/hi/{name}"),
            Self::NotFound(NotFoundParams { path }) => write!(f, "/{path}"),
        }
    }
}

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
