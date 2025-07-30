mod homepage;
mod sidebar;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use sidebar::SideBar;

const MAIN_PAGE_STYLES: &str = "flex flex-1 flex-col items-center bg-[rgb(00,00,09)] text-gray-200";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| PageNotFound>
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/experience") view=ExperiencePage/>
                <Route path=path!("/projects") view=ProjectPage/>
                <Route path=path!("/about") view=AboutPage/>
                <Route path=path!("/contact") view=ContactPage/>
                <Route path=path!("/tools") view=ToolsPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn PageNotFound() -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col items-center justify-center min-h-screen bg-[rgb(00,00,09)] text-gray-200">
            <svg width="120" height="120" viewBox="0 0 120 120" fill="none" class="mb-6">
                <circle cx="60" cy="60" r="55" stroke="#e53e3e" stroke-width="8" fill="#1c1e20"/>
                <text x="50%" y="54%" text-anchor="middle" fill="#e53e3e" font-size="48" font-weight="bold" dy=".3em">404</text>
            </svg>
            <h1 class="text-4xl font-bold text-red-500 mb-2">Page Not Found</h1>
            <p class="text-white-500 text-lg mb-4 text-center">
                Oops! The page you are looking for does not exist.
            </p>
            <a href="/" class="text-blue-400 hover:underline text-base">Go back home</a>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
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
        <Title text="Experience - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
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
        <Title text="Projects - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
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
        <Title text="About - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
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
        <Title text="Contact - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
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
        <Title text="Tools - Matthew Champagne" />
        <div class="flex h-screen">
            <SideBar />
            <div class=MAIN_PAGE_STYLES>
                <p class="m-auto w-3/4 md:text-xl text-white-500">
                    "Under Construction"
                </p>
            </div>
        </div>
    }
}
