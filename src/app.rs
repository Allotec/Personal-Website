mod sidebar;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use sidebar::SideBar;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| PageNotFound>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("/experience") view=PageNotFound/>

                //TODO: Implement these pages
                <Route path=StaticSegment("/projects") view=PageNotFound/>
                <Route path=StaticSegment("/about") view=PageNotFound/>
                <Route path=StaticSegment("/contact") view=PageNotFound/>
                <Route path=StaticSegment("/tools") view=PageNotFound/>
            </Routes>
        </Router>
    }
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
        <p> "Page not Found" </p>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Make a description about yourself"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ExperiencePage() -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col bg-stone-950 text-center font-fira-mono text-base text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <SideBar />
                <p class="m-auto w-3/4 md:text-xl">
                    "This is an experience page"
                </p>
            </div>
        </div>
    }
}
