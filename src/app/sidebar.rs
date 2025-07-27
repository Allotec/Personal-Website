use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-60 bg-[rgb(28,30,32)] overflow-auto">
            <ProfileWTitle />
            <SideBarNavigation />
            <ConnectWith />
        </div>
    }
}

#[component]
fn ProfileWTitle() -> impl IntoView {
    view! {
        <div class="flex flex-row mt-12 space-x-2 align-start w-full ml-12">
            <ProfilePic />
            <JobTitleAnimation />
        </div>
    }
}

#[component]
fn JobTitleAnimation() -> impl IntoView {
    view! {
        <div class="flex flex-col text-gray-100 text-sm">
            <div class="flex">
                "Matthew"
            </div>
            <div class="flex">
                "Systems Programming"
            </div>
        </div>
    }
}

#[component]
fn ProfilePic() -> impl IntoView {
    view! {
        <div class="flex w-10">
            <img class="rounded-full" src="/images/profile.jpg" alt="Profile picture"/>
        </div>
    }
}

#[component]
fn ConnectWith() -> impl IntoView {
    view! {}
}

#[component]
fn SideBarNavigation() -> impl IntoView {
    view! {
        <div class="flex flex-col mt-12 font-sans items-center justify-top tracking-widest text-gray-100 sm:text-lg">
            <SideBarItem text="Home".into() path="/".into() />
            <SideBarItem text="Experience".into() path="/experience".into() />
            <SideBarItem text="Projects".into() path="/projects".into() />
            <SideBarItem text="About".into() path="/about".into() />
            <SideBarItem text="Contact".into() path="/contact".into() />
            <SideBarItem text="Tools".into() path="/tools".into() />
        </div>
    }
}

#[component]
fn SideBarItem(text: String, path: String) -> impl IntoView {
    view! {
    <div class="flex">
        <a href={path} class="hover:underline">
            {text}
        </a>
    </div>
    }
}
