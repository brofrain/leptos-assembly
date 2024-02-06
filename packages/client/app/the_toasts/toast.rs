use client_components::BaseIcon;
use client_hooks::toast;
use common::prelude::*;

#[component]
pub fn Toast(children: Children, severity: toast::Severity) -> impl IntoView {
    let icon = match severity {
        toast::Severity::Info => view! { <BaseIcon icon=icon::CgInfo/> },
        toast::Severity::Success => {
            view! { <BaseIcon icon=icon::AiCheckCircleOutlined/> }
        }
        toast::Severity::Warning => {
            view! { <BaseIcon icon=icon::IoWarningOutline/> }
        }
        toast::Severity::Error => {
            view! { <BaseIcon icon=icon::BiErrorCircleRegular/> }
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
