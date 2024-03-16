use client_components::{BaseAnimatedFor, BaseButton, BaseModal};
use client_hooks::confirm;
use client_macros::pin_test_selector;
use common::prelude::*;

#[component]
pub fn Confirms() -> impl IntoView {
    let queue = confirm::use_queue();

    let reversed_queue =
        Memo::new(move |_| queue.get().iter().rev().copied().collect::<Vec<_>>());

    view! {
        <BaseAnimatedFor
            each=reversed_queue
            key=|confirm| confirm.with_value(|v| *v.id())
            children=move |confirm| {
                let id = with!(| confirm | * confirm.id());
                let accept = move |_| {
                    queue.resolve_by_id(id, confirm::ResolutionStatus::Accepted);
                };
                let cancel = Callback::new(move |_| {
                    if with!(| confirm | confirm.cancel().is_some()) {
                        queue.resolve_by_id(id, confirm::ResolutionStatus::Cancelled);
                    }
                });
                let cancel_btn_view = confirm
                    .with_value(|v| {
                        v.cancel()
                            .map(move |cancel_msg| {
                                view! {
                                    <BaseButton
                                        on:click=cancel
                                        attr:test=pin_test_selector!(cancel)
                                    >
                                        {cancel_msg}
                                    </BaseButton>
                                }
                            })
                    });
                view! {
                    <BaseModal on_overlay_click=cancel>
                        <div class=uno!["text-center"] test=pin_test_selector!(body)>
                            {confirm.with_value(|v| *v.body())}
                        </div>

                        <div class=uno![
                            "mt4", "flex justify-center gap4"
                        ]>
                            {cancel_btn_view}
                            <BaseButton on:click=accept attr:test=pin_test_selector!(confirm)>
                                {confirm.with_value(|v| *v.accept())}
                            </BaseButton>
                        </div>
                    </BaseModal>
                }
            }
        />
    }
}
