use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="flex flex-col bg-[rgb(28,30,32)]">
            <ProfilePicture />
            <SideBarNavigation />
            <ConnectWith />
        </div>
    }
}

#[component]
fn ProfilePicture() -> impl IntoView {
    view! {
        <img src="/images/profile.jpg" alt="Profile picture"/>
    }
}

#[component]
fn ConnectWith() -> impl IntoView {
    view! {}
}

#[component]
fn SideBarNavigation() -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans items-center justify-top tracking-widest text-gray-100 sm:text-lg">
            <SideBarItem text="Home".into() />
            <SideBarItem text="Experience".into() />
            <SideBarItem text="Projects".into() />
            <SideBarItem text="About".into() />
            <SideBarItem text="Contact".into() />
            <SideBarItem text="Tools".into() />
        </div>
    }
}

#[component]
fn SideBarItem(text: String) -> impl IntoView {
    view! {
    <div class="flex">
        <a href="/" class="hover:underline">
            {text}
        </a>
    </div>
    }
}
