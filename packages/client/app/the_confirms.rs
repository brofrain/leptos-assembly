use crate::app::{
    components::base::{AnimatedFor, Button, Modal},
    composables::confirm,
    prelude::*,
};

#[component]
pub fn TheConfirms() -> impl IntoView {
    let queue = confirm::use_queue();

    let reversed_queue = Memo::new(move |_| {
        queue.get().iter().rev().copied().collect::<Vec<_>>()
    });

    view! {
        <AnimatedFor
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
                                    <Button on:click=cancel attr:test="confirm-cancel-btn">
                                        {cancel_msg}
                                    </Button>
                                }
                            })
                    });
                view! {
                    <Modal on_overlay_click=cancel>
                        <div class=uno!["text-center"] test="confirm-body">
                            {confirm.with_value(|v| *v.body())}
                        </div>

                        <div class=uno![
                            "mt4", "flex justify-center gap4"
                        ]>
                            {cancel_btn_view}
                            <Button on:click=accept attr:test="confirm-accept-btn">
                                {confirm.with_value(|v| *v.accept())}
                            </Button>
                        </div>
                    </Modal>
                }
            }
        />
    }
}
