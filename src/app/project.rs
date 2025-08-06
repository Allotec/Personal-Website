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
                    subtitle="Playground - Small Scripts to Big Apps"
                    sub_color="text-white-100"
                />
                <ProjectGrid />
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ProjectGrid() -> impl IntoView {
    view! {
        <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 gap-4">
            <ExampleTile />
            <ExampleTile />
            <ExampleTile />
            <ExampleTile />
            <ExampleTile />
        </div>
    }
}

#[component]
fn ExampleTile() -> impl IntoView {
    view! {
        <ProjectTile
            img_path="images/screenshot.jpg".to_string()
            title="Project Title".to_string()
            description="This is a project description".to_string()
            tags=vec!["Visit".to_string(), "Github".to_string()]
            host_site="https://google.com".to_string()
            is_active=true
        />
    }
}

#[component]
fn ProjectTile(
    img_path: String,
    title: String,
    description: String,
    tags: Vec<String>,
    host_site: String,
    is_active: bool,
) -> impl IntoView {
    view! {
        <a href={host_site}>
        <div class="flex flex-col bg-[rgb(28,30,32)] rounded-lg p-3 hover:scale-103">
            <img src=img_path class="flex rounded-lg w-full object-cover mb-2" />
            <div class="flex text-white-100 text-base font-semibold">
                {title}
            </div>
            <div class="flex text-stone-300 text-sm">
                {description}
            </div>
            <div class="flex flex-row mt-2 space-x-2 justify-between items-center">
                <div class="flex space-x-2">
                    {tags.iter().map(|text| view! {
                        <TileTag text=text.clone() />
                    }).collect::<Vec<_>>()}
                </div>

            <StatusTag is_active=is_active />
            </div>
        </div>
        </a>
    }
}

#[component]
fn TileTag(text: String) -> impl IntoView {
    view! {
        <div class="text-white-100 bg-black pl-2 pr-2 border rounded-lg" style="border-color:rgba(255,248,225,0.5);">
            {text}
        </div>
    }
}

#[component]
fn StatusTag(is_active: bool) -> impl IntoView {
    view! {
        <Show
            when=move || {is_active}
        >
            <ActiveTag />
        </Show>

        <Show
            when=move || {!is_active}
        >
            <ArchivedTag />
        </Show>
    }
}

#[component]
fn ActiveTag() -> impl IntoView {
    view! {
        <div class="text-white-100 bg-green-900 pl-2 pr-2 rounded-full">
            "Active"
        </div>
    }
}

#[component]
fn ArchivedTag() -> impl IntoView {
    view! {
        <div class="text-white-100 bg-[rgb(38,40,42)] pl-2 pr-2 rounded-full">
            "Archived"
        </div>
    }
}
