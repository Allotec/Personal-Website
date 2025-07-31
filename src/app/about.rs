use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn AboutPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <AboutPageMainContent />
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn AboutPageMainContent() -> impl IntoView {
    view! {
        <p> "About Content goes here." </p>
    }
}
