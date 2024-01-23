use client_components::BaseIcon;
use client_composables::toast;
use exports::client::{icondata as i, prelude::*};

#[component]
pub fn Toast(children: Children, severity: toast::Severity) -> impl IntoView {
    let icon = match severity {
        toast::Severity::Info => view! { <BaseIcon icon=i::CgInfo/> },
        toast::Severity::Success => {
            view! { <BaseIcon icon=i::AiCheckCircleOutlined/> }
        }
        toast::Severity::Warning => {
            view! { <BaseIcon icon=i::IoWarningOutline/> }
        }
        toast::Severity::Error => {
            view! { <BaseIcon icon=i::BiErrorCircleRegular/> }
        }
    };

    view! {
        <div class=uno![
            "max-w-9/10", "rounded", "p-(y1 x4)", "shadow", "bg-accent/80",
            "text-(accent-contrast sm)", "flex items-center gap2",
        ]>

            {icon} {children()}
        </div>
    }
}
