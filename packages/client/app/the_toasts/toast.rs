use client_composables::toast;
use common_exports::client::prelude::*;

#[component]
pub fn Toast(children: Children, severity: toast::Severity) -> impl IntoView {
    let icon_class = match severity {
        toast::Severity::Info => "icon-material-symbols-info-outline-rounded",
        toast::Severity::Success => {
            "icon-material-symbols-check-circle-outline-rounded"
        }
        toast::Severity::Warning => {
            "icon-material-symbols-warning-outline-rounded"
        }
        toast::Severity::Error => "icon-material-symbols-error-outline-rounded",
    };

    view! {
        <div class=uno![
            "max-w-9/10", "rounded", "p-(y1 x4)", "shadow", "bg-accent/80",
            "text-(accent-contrast sm)", "flex items-center gap2",
        ]>

            <div class=icon_class></div>

            {children()}
        </div>
    }
}
