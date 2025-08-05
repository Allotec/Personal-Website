use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;

#[component]
pub(crate) fn ProjectPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <div class="m-8">
                <PageHeading
                    main_title="Projects"
                    main_color="text-white-100"
                    subtitle="Playground"
                    sub_color="text-white-100"
                />
                // <ProjectGrid />
                <ProjectTile
                    img_path="images/profile.jpg".to_string()
                />
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ProjectGrid() -> impl IntoView {
    view! {
        <p> "Project Content goes here." </p>
    }
}

#[component]
fn ProjectTile(
    img_path: String,
    // title: String,
    // description: String,
    // host_site: String,
    // status: bool,
) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-[rgb(28,30,32)] rounded-lg p-3">
            <img src=img_path class="flex rounded-lg w-full object-cover" />
            <div class="flex text-white-100 text-base font-semibold">
                "Project Title"
            </div>
            <div class="flex text-stone-300 text-sm">
                "This is a project description"
            </div>
            <div class="flex flex-row mt-2">
                <div class="text-white-100 bg-black pl-2 pr-2 border rounded-lg" style="border-color:rgba(255,248,225,0.5);">
                    "Visit"
                </div>
            </div>
        </div>
    }
}
