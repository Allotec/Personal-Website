use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn ExperiencePageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <ExperiencePageMainContent />
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ExperiencePageMainContent() -> impl IntoView {
    view! {
        <p> "Experience Content goes here." </p>
    }
}
