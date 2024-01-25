use client_components::{BaseAnimatedFor, BaseButton};
use client_composables::toast::{self, Severity};
use client_i18n::use_i18n;
use client_utils::future::{next_tick, spawn_local_owned};
use exports::client::{
    icondata::{self as i, Icon},
    prelude::*,
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use utils::id;

mod components {
    flatten_pub_mod!(feature_brick);
}
use components::FeatureBrick;

#[derive(Clone, PartialEq)]
struct Feature {
    title: &'static str,
    icon: Icon,
}

static FEATURES: [Feature; 11] = [
    Feature {
        title: "Leptos",
        icon: i::SiLeptos,
    },
    Feature {
        title: "Rust",
        icon: i::SiRust,
    },
    Feature {
        title: "Axum",
        icon: i::BsBoxSeamFill,
    },
    Feature {
        title: "Leptos-Use",
        icon: i::BsBoxSeamFill,
    },
    Feature {
        title: "Leptos i18n",
        icon: i::IoEarth,
    },
    Feature {
        title: "UnoCSS",
        icon: i::SiUnocss,
    },
    Feature {
        title: "Leptos Icons",
        icon: i::FaFaceSmileRegular,
    },
    Feature {
        title: "Vite",
        icon: i::SiVite,
    },
    Feature {
        title: "PWA",
        icon: i::IoCloudOffline,
    },
    Feature {
        title: "Webfonts",
        icon: i::RiFontSizeEditor,
    },
    Feature {
        title: "Playwright",
        icon: i::SiPlaywright,
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
pub fn About() -> impl IntoView {
    let i18n = use_i18n();
    let push_toast = toast::use_push();
    let rng = StoredValue::new(thread_rng());

    let feature_bricks = RwSignal::new({
        let mut bricks =
            FEATURES.iter().map(FeatureBrick::new).collect::<Vec<_>>();

        // BUG: randomized bricks don't match during hydration and SVGs are
        // broken as the result
        rng.update_value(|rng| bricks.shuffle(&mut *rng));

        bricks
    });

    let gen_random_feature_brick_index = Callback::new(move |()| {
        if with!(|feature_bricks| feature_bricks.is_empty()) {
            return 0;
        }

        let mut i = None;
        update!(|rng| {
            i = Some(
                rng.gen_range(0..with!(|feature_bricks| feature_bricks.len())),
            );
        });
        i.unwrap()
    });

    let push_bricks_updated_toast_on_next_tick = move || {
        spawn_local_owned(async move {
            next_tick().await;
            push_toast(Severity::Success, t!(i18n, about.features.updated));
        });
    };

    let add_random_feature_brick = move |_| {
        let index = gen_random_feature_brick_index(());
        update!(|rng, feature_bricks| {
            let new_brick =
                FeatureBrick::new(&FEATURES[rng.gen_range(0..FEATURES.len())]);

            feature_bricks.insert(index, new_brick);
        });

        push_bricks_updated_toast_on_next_tick();
    };

    let remove_random_feature_brick = move |_| {
        if with!(|feature_bricks| feature_bricks.is_empty()) {
            push_toast(
                Severity::Error,
                t!(i18n, about.features.nothing_to_remove),
            );
            return;
        }

        let index = gen_random_feature_brick_index(());
        update!(|feature_bricks| {
            feature_bricks.remove(index);
        });
        push_bricks_updated_toast_on_next_tick();
    };

    let shuffle_feature_bricks = {
        move |_| {
            let previous_feature_and_active_pairs =
                with!(|feature_bricks| feature_bricks
                    .iter()
                    .map(|brick| (brick.feature, (brick.active)()))
                    .collect::<Vec<_>>());

            update!(|feature_bricks| {
                rng.update_value(|rng| feature_bricks.shuffle(&mut *rng));

                for (i, brick) in feature_bricks.iter().enumerate() {
                    let (previous_feature, previous_active) =
                        previous_feature_and_active_pairs[i];

                    if brick.feature != previous_feature
                        || (brick.active)() != previous_active
                    {
                        push_bricks_updated_toast_on_next_tick();
                        return;
                    }
                }

                push_toast(
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

                <BaseButton on:click=remove_random_feature_brick>
                    {t!(i18n, about.features.remove_random)}
                </BaseButton>
            </div>

            <div class="mt2 grid grid-cols-[repeat(3,auto)] justify-center gap2">
                <BaseAnimatedFor
                    each=feature_bricks
                    key=|brick| brick.id
                    children=move |brick| {
                        view! {
                            <FeatureBrick
                                title=brick.feature.title
                                icon=brick.feature.icon
                                active=brick.active
                            />
                        }
                    }
                />

            </div>
        </div>
    }
}
