use std::fmt;

use leptos_router::{NavigateOptions, Route as RouteView, Routes};

use crate::app::{
    components::{
        layouts,
        pages::{self, HiParams, NotFoundParams},
    },
    prelude::*,
};

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
            Self::Hi(HiParams { name }) => write!(f, "/hi/{name}"),
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

#[component]
pub fn View() -> impl IntoView {
    view! {
        <Routes>
            <RouteView path="" view=layouts::Home>
                <RouteView path=Route::Home view=pages::Home/>
            </RouteView>

            <RouteView path="" view=layouts::Default>
                <RouteView
                    path=Route::Hi(HiParams {
                        name: ":name".to_owned(),
                    })

                    view=pages::Hi
                />
                <RouteView path=Route::About view=pages::About/>
            </RouteView>

            <RouteView path="" view=layouts::Blank>
                <RouteView
                    path=Route::NotFound(NotFoundParams {
                        path: "*path".to_owned(),
                    })

                    view=pages::NotFound
                />
            </RouteView>

        </Routes>
    }
}
