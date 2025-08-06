use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::app::reused_buttons::ReachOutForCall;
use crate::icons::LinkedinIconBig;
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
                class="m-8 bg-gradient-to-br from-gray-900 via-gray-800 to-black backdrop-blur-md rounded-xl shadow-lg p-2"
                style="background-image: url('data:image/svg+xml;utf8,<svg width=\"40\" height=\"40\" viewBox=\"0 0 40 40\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><rect x=\"0\" y=\"0\" width=\"40\" height=\"40\" fill=\"none\" stroke=\"%23333\" stroke-width=\"1\"/></svg>'); background-size: 40px 40px;"
            >
                <PageHeading
                    main_title="Contact"
                    main_color="text-white-100"
                    subtitle="If you'are building in / excited about embedded systems or just wanna chat, say hi on Linkedin."
                    sub_color="text-white-100"
                />

                <div class="space-y-10">
                    <LinkedinWideButton />
                    <ReachOutForCall />
                </div>
                <div class="mb-40"> </div>
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn LinkedinWideButton() -> impl IntoView {
    view! {
        <a class="flex" href="https://www.linkedin.com/in/allotec/">
        <div class="group flex flex-row justify-between items-center p-3 w-full border rounded-lg mt-10 space-x-3" style="border-color:rgba(255,248,225,0.3);">
            <div class="flex">
                <LinkedinIconBig />
            </div>
            <div class="flex text-white-100">
                "Connect with me on Linkedin to keep up with my work"
            </div>

            <div class="flex bg-black rounded-lg p-2 text-sm transform transition-transform duration-200 group-hover:scale-110">
                "Connect"
            </div>
        </div>
        </a>
    }
}
