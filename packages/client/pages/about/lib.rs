#[macro_use]
extern crate client_globals;

use client_components::{BaseAnimatedFor, BaseButton};
use client_composables::{
    id,
    toast::{self, Severity},
};
use client_globals::prelude::*;
use client_i18n::use_i18n;
use client_utils::future::{next_tick, spawn_local_owned};
use leptos_i18n::t;
use rand::{seq::SliceRandom, thread_rng, Rng};

mod components {
    flatten_pub_mod!(feature_brick);
}
use components::FeatureBrick;

#[derive(Clone, PartialEq)]
struct Feature {
    icon_class: &'static str,
    title: &'static str,
}

const FEATURES: [Feature; 11] = [
    Feature {
        title: "Leptos",
        icon_class: "icon-simple-icons-leptos",
    },
    Feature {
        title: "Rust",
        icon_class: "icon-simple-icons-rust",
    },
    Feature {
        title: "Axum",
        icon_class: "icon-fluent-box-16-filled",
    },
    Feature {
        title: "Leptos-Use",
        icon_class: "icon-fluent-box-16-filled",
    },
    Feature {
        title: "Leptos i18n",
        icon_class: "icon-fluent-box-16-filled",
    },
    Feature {
        title: "UnoCSS",
        icon_class: "icon-simple-icons-unocss",
    },
    Feature {
        title: "Iconify",
        icon_class: "icon-simple-icons-iconify",
    },
    Feature {
        title: "Vite",
        icon_class: "icon-simple-icons-vite",
    },
    Feature {
        title: "PWA",
        icon_class: "icon-ion-cloud-offline",
    },
    Feature {
        title: "Webfonts",
        icon_class: "icon-mingcute-font-size-fill",
    },
    Feature {
        title: "Playwright",
        icon_class: "icon-simple-icons-playwright",
    },
];

#[derive(Clone)]
struct FeatureBrick {
    id: usize,
    active: RwSignal<bool>,
    feature: &'static Feature,
}

impl FeatureBrick {
    pub fn new(feature: &'static Feature) -> Self {
        Self {
            id: id::usize(),
            feature,
            active: RwSignal::new(false),
        }
    }
}

#[component]
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();
    let rng = StoredValue::new(thread_rng());

    let feature_breaks = RwSignal::new({
        let mut bricks =
            FEATURES.iter().map(FeatureBrick::new).collect::<Vec<_>>();

        rng.update_value(|rng| bricks.shuffle(&mut *rng));

        bricks
    });

    let gen_random_feature_brick_index = Callback::new(move |()| {
        if with!(|feature_breaks| feature_breaks.is_empty()) {
            return 0;
        }

        let mut i = None;
        update!(|rng| {
            i = Some(
                rng.gen_range(0..with!(|feature_breaks| feature_breaks.len())),
            );
        });
        i.unwrap()
    });

    let push_bricks_updated_toast_on_next_tick = move || {
        spawn_local_owned(async move {
            next_tick().await;
            toast::push(Severity::Success, t!(i18n, about.features.updated));
        });
    };

    let add_random_feature_brick = move |_| {
        let index = gen_random_feature_brick_index(());
        update!(|rng, feature_breaks| {
            let new_brick =
                FeatureBrick::new(&FEATURES[rng.gen_range(0..FEATURES.len())]);

            feature_breaks.insert(index, new_brick);
        });

        push_bricks_updated_toast_on_next_tick();
    };

    let remove_random_feature_break = move |_| {
        if with!(|feature_breaks| feature_breaks.is_empty()) {
            toast::push(
                Severity::Error,
                t!(i18n, about.features.nothing_to_remove),
            );
            return;
        }

        let index = gen_random_feature_brick_index(());
        update!(|feature_breaks| {
            feature_breaks.remove(index);
        });
        push_bricks_updated_toast_on_next_tick();
    };

    let shuffle_feature_bricks = {
        move |_| {
            let previous_feature_and_active_pairs =
                with!(|feature_breaks| feature_breaks
                    .iter()
                    .map(|brick| (brick.feature, (brick.active)()))
                    .collect::<Vec<_>>());

            update!(|feature_breaks| {
                rng.update_value(|rng| feature_breaks.shuffle(&mut *rng));

                for (i, brick) in feature_breaks.iter().enumerate() {
                    let (previous_feature, previous_active) =
                        previous_feature_and_active_pairs[i];

                    if brick.feature != previous_feature
                        || (brick.active)() != previous_active
                    {
                        push_bricks_updated_toast_on_next_tick();
                        return;
                    }
                }

                toast::push(
                    Severity::Warning,
                    t!(i18n, about.features.shuffle_changed_nothing),
                );
            });
        }
    };

    view! {
        <div>
            <div class="flex flex-wrap gap2 justify-center">
                <BaseButton on:click=add_random_feature_brick>
                    {t!(i18n, about.features.add_random)}
                </BaseButton>

                <BaseButton on:click=shuffle_feature_bricks>
                    {t!(i18n, about.features.shuffle)}
                </BaseButton>

                <BaseButton on:click=remove_random_feature_break>
                    {t!(i18n, about.features.remove_random)}
                </BaseButton>
            </div>

            <div class="mt2 grid grid-cols-[repeat(3,auto)] justify-center gap2">
                <BaseAnimatedFor
                    each=feature_breaks
                    key=|brick| brick.id
                    children=move |brick| {
                        view! {
                            <FeatureBrick
                                title=brick.feature.title
                                icon_class=brick.feature.icon_class
                                active=brick.active
                            />
                        }
                    }
                />

            </div>
        </div>
    }
}
