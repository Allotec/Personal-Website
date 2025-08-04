use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::icons::LinkedinIcon;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;

#[component]
pub(crate) fn ContactPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <div
                class="bg-gradient-to-br from-gray-900 via-gray-800 to-black backdrop-blur-md rounded-xl shadow-lg p-2"
                style="background-image: url('data:image/svg+xml;utf8,<svg width=\"40\" height=\"40\" viewBox=\"0 0 40 40\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><rect x=\"0\" y=\"0\" width=\"40\" height=\"40\" fill=\"none\" stroke=\"%23333\" stroke-width=\"1\"/></svg>'); background-size: 40px 40px;"
            >
                <PageHeading
                    main_title="Contact"
                    main_color="text-white-100"
                    subtitle="If you'are building in / excited about embedded systems or just wanna chat, say hi on Linkedin."
                    sub_color="text-white-100"
                />

                <LinkedinWideButton />
                <ReachOutForCall />
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn LinkedinWideButton() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-between items-center p-3 w-full border rounded-lg mt-10 space-x-3" style="border-color:rgba(255,248,225,0.3);">
            <div class="flex">
                <LinkedinIcon />
            </div>
            <div class="flex text-white-100">
                "Connect with me on Linkedin to keep up with me"
            </div>
            <div class="flex bg-black rounded-lg p-2 text-sm">
                "Follow"
            </div>
        </div>
    }
}

#[component]
fn ReachOutForCall() -> impl IntoView {
    view! {}
}
