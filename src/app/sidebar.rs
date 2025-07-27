use crate::icons::*;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::hooks::use_location;
use log::Record;
use log::info;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-60 bg-[rgb(28,30,32)] overflow-y-auto">
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
            //TODO: Animate this with a typing animation
            <div class="flex">
                "Systems Programmer"
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
        <div class="flex flex-col mt-10 font-sans items-start justify-top text-gray-100 text-base w-full">
            <SideBarItem text="Home".into() path="/".into() icon=HouseIcon />
            <SideBarItem text="Experience".into() path="/experience".into() icon=HouseIcon />
            <SideBarItem text="Projects".into() path="/projects".into() icon=HouseIcon />
            <SideBarItem text="About".into() path="/about".into() icon=HouseIcon />
            <SideBarItem text="Contact".into() path="/contact".into() icon=HouseIcon />
            <SideBarItem text="Tools".into() path="/tools".into() icon=HouseIcon />
        </div>
    }
}

#[component]
fn SideBarItem<F: IntoView>(text: String, path: String, icon: F) -> impl IntoView {
    let current_path = use_location().pathname.get();
    let mut attributes = String::from(
        "flex mt-2 pl-4 pt-2 pb-2 pr-6 text-left rounded-lg text-base align-middle items-center hover:bg-[rgb(38,40,42)]",
    );

    if path == current_path {
        attributes.push_str(" bg-black border-solid border-1 border-amber-100");
    }

    view! {
    <a href={path} class="ml-4 w-26/30">
        <div class=attributes>
            <span class="pr-2">
                {icon}
            </span>
            {text}
        </div>
    </a>
    }
}
