use leptos_router::{use_params, IntoParam, Params};
use pct_str::PctStr;

use crate::app::{
    components::base::Link,
    prelude::*,
    router::Route,
    stores::{use_store, Names},
};

#[derive(Params, PartialEq)]
pub struct HiParams {
    pub name: String,
}

#[component]
pub fn Hi() -> impl IntoView {
    let params = use_params::<HiParams>();
    let i18n = use_i18n();

    let names_store = use_store::<Names>();

    let name = Memo::new(move |_| {
        with!(|params| {
            PctStr::new(&params.as_ref().unwrap().name)
                .unwrap()
                .decode()
        })
    });

    Effect::new(move |_| {
        names_store.push_name(&name());
    });

    let other_names_sorted = names_store.other_names_sorted();

    view! {
        <div class="text-center">
            <div class="inline-block text-4xl mb1 icon-mdi-human-greeting"></div>

            <div>
                <p>{t!(i18n, name.hi, name)}</p>
            </div>

            <p>
                <em class="text-xs op60">{t!(i18n, name.description)}</em>
            </p>

            <Show when=move || !other_names_sorted().is_empty() fallback=|| {}>
                <div class="text-sm mt4">
                    <span class="op75">{t!(i18n, name.aka)} ":"</span>

                    // TODO: use FLIP animations
                    <div class="flex flex-col">
                        <For
                            each=other_names_sorted
                            key=String::clone
                            children=move |name| {
                                view! {
                                    <Link
                                        class="text-sm"
                                        to=Route::Hi(HiParams { name: name.clone() })
                                    >
                                        {name}
                                    </Link>
                                }
                            }
                        />

                    </div>
                </div>
            </Show>
        </div>
    }
}
