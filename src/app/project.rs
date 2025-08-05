use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;

#[component]
pub(crate) fn ProjectPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <ProjectPageMainContent />
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ProjectPageMainContent() -> impl IntoView {
    view! {
        <p> "Project Content goes here." </p>
    }
}
