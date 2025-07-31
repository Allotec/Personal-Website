use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn HomePageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <BigNameTitle />
            <HomePageMainContent />
            <CopyrightFooter />
        </div>
    }
}

#[component]
pub(crate) fn BigNameTitle() -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-2 items-center w-9/10 max-w-2xl">
            <div class="flex text-white-100 text-base sm:text-3xl md:text-4xl lg:text-5xl font-sans font-semibold justify-start w-full mx-auto">
                "Hey, I'm Matt"
            </div>
            <div class="flex text-[#A8A891] text-base sm:text-2xl md:text-3xl lg:text-4xl justify-start w-full">
                "Systems Programmer"
            </div>
        </div>
    }
}

#[component]
fn HomePageMainContent() -> impl IntoView {
    view! {
        <p> "Main Content of the homepage goes here." </p>
    }
}
