use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn ToolsPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <ToolsPageMainContent />
            <CopyrightFooter />
        </div>
    }
}

// TODO: Tools to callout
// Neovim
// Copilot
// latex
// Rust
// Tailscale
// Arch Linux

#[component]
fn ToolsPageMainContent() -> impl IntoView {
    view! {
        <p> "Tools Content goes here." </p>
    }
}
