use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn ContactPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <ContactPageMainContent />
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ContactPageMainContent() -> impl IntoView {
    view! {
        <p> "Contact Content goes here." </p>
    }
}
