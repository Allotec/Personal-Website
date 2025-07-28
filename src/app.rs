mod sidebar;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use sidebar::SideBar;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| PageNotFound>
                <Route path=path!("/") view=HomePage/>
                // <Route path=path!("/experience") view=ExperiencePage/>
                // <Route path=path!("/projects") view=ProjectPage/>
                // <Route path=path!("/about") view=AboutPage/>
                // <Route path=path!("/contact") view=ContactPage/>
                // <Route path=path!("/tools") view=ToolsPage/>
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
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ExperiencePage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ProjectPage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ContactPage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ToolsPage() -> impl IntoView {
    view! {
        <div class="flex h-screen">
            <SideBar />
            <div class="flex flex-1 flex-col items-center bg-[rgb(20,22,23)] text-gray-200">
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}
