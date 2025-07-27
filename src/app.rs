use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| PageNotFound>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("/experience") view=ExperiencePage/>

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
        <div class="flex min-h-screen flex-col bg-stone-950 text-center font-fira-mono text-base text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <SideBar />
                <p class="m-auto w-3/4 md:text-xl">
                    "This is a client-side rendering website template made with "
                </p>
            </div>
        </div>
    }
}

#[component]
fn SideBar() -> impl IntoView {
    view! {
        <div
            class="inset-x-0 bottom-auto top-0 mt-4 flex items-center justify-center tracking-widest text-amber-200 sm:text-lg">
            "Bye, Matt!"
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
