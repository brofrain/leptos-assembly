use client_router::Route;
use client_utils::reactivity::{MaybeTextProp, MaybeTextPropExt};
use common::prelude::*;

pub enum BaseLinkTo {
    Internal(Route),
    External(String),
}

impl From<Route> for BaseLinkTo {
    fn from(route: Route) -> Self {
        Self::Internal(route)
    }
}

impl From<String> for BaseLinkTo {
    fn from(url: String) -> Self {
        Self::External(url)
    }
}

impl From<&str> for BaseLinkTo {
    fn from(url: &str) -> Self {
        Self::External(url.to_owned())
    }
}

impl BaseLinkTo {
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::External(_))
    }
}

#[component]
pub fn BaseLink(
    children: Children,
    #[prop(optional, into)] to: Option<BaseLinkTo>,
    #[prop(optional, into)] title: MaybeTextProp,
    #[prop(optional, into)] class: MaybeTextProp,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let external = to.as_ref().is_some_and(BaseLinkTo::is_external);

    view! {
        <a
            {..attrs}
            class=move || {
                uno![
                    "inline-block", "underline text-secondary-interactive", "transition-colors",
                    "cursor-pointer", "select-none", class.get_string()
                ]
            }

            title=title
            external=external
            target=external.then_some("_blank")
            href=to
                .map(|v| match v {
                    BaseLinkTo::Internal(route) => route.to_string(),
                    BaseLinkTo::External(url) => url,
                })
        >

            {children()}
        </a>
    }
}
