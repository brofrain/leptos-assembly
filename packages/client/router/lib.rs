use std::fmt;

use leptos::Params;
use leptos_router::{NavigateOptions, Params};

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

pub fn use_navigate() -> impl Fn(&Route, NavigateOptions) {
    #[allow(clippy::disallowed_methods)]
    let navigate = leptos_router::use_navigate();
    move |route, options| {
        navigate(&route.to_string(), options);
    }
}
