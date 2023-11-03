use client_components::{BaseAnimatedFor, BaseLink};
use client_composables::i18n::{t, use_i18n};
use client_globals::prelude::*;
use client_router::{HiParams, Route};
use client_stores::{use_store, Names};
use leptos_router::use_params;
use pct_str::PctStr;

#[component]
pub fn Index() -> impl IntoView {
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

                    <div class="flex flex-col">
                        <BaseAnimatedFor
                            each=other_names_sorted
                            key=String::clone
                            children=move |name| {
                                view! {
                                    <BaseLink
                                        class="text-sm"
                                        to=Route::Hi(HiParams { name: name.clone() })
                                    >
                                        {name}
                                    </BaseLink>
                                }
                            }
                        />

                    </div>
                </div>
            </Show>
        </div>
    }
}
