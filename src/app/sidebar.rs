use crate::icons::*;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::hooks::use_location;

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
    let titles = [
        "Systems Programmer",
        "Problem Solver",
        "Rust Enthusiast",
        "Embedded Systems",
    ];

    view! {
        <div class="flex flex-col text-gray-100 text-sm">
            <div class="flex">
                "Matthew"
            </div>
            <div class="flex">
                <TypingAnimation titles=titles.to_vec() />
            </div>
        </div>
    }
}

#[component]
fn TypingAnimation(titles: Vec<&'static str>) -> impl IntoView {
    use leptos::*;
    use std::time::Duration;

    let index = RwSignal::new(0);
    let char_count = RwSignal::new(0);
    let typing = RwSignal::new(true);
    let value = RwSignal::new(titles);

    set_interval(
        {
            move || {
                let current_title = value.get()[index.get()];
                let chars = current_title.len();

                if typing.get() {
                    if char_count.get() < chars {
                        char_count.update(|c| *c += 1);
                    } else {
                        typing.set(false);
                    }
                } else if char_count.get() > 0 {
                    char_count.update(|c| *c -= 1);
                } else {
                    typing.set(true);
                    index.update(|i| *i = (*i + 1) % value.get().len());
                }
            }
        },
        Duration::from_millis(100),
    );

    view! {
        <span>
            {move || {
                let current = value.get()[index.get()];
                let chars = char_count.get();
                current.chars().take(chars).collect::<String>()
            }}
            <span class="blinking-cursor">|</span>
        </span>
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
    view! {
        <div class="flex flex-col mt-8 font-sans items-start justify-top text-gray-100 text-base w-full">
            <div class="flex ml-4"> Connect </div>
            <ConnectWithItem text="Twitter".into() link="https://twitter.com/alllotec".into() icon=TwitterIcon />
            <ConnectWithItem text="LinkedIn".into() link="https://www.linkedin.com/in/allotec/".into() icon=LinkedinIcon />
            <ConnectWithItem text="Github".into() link="https://github.com/Allotec".into() icon=ProjectIcon />
            <ConnectWithItem text="BlueSky".into() link="https://bsky.app/profile/allotec.bsky.social".into() icon=BlueSkyIcon />
            <ConnectWithItem text="Instagram".into() link="https://www.instagram.com/alllotec/".into() icon=InstagramIcon />
        </div>
    }
}

#[component]
fn ConnectWithItem<F: IntoView>(text: String, link: String, icon: F) -> impl IntoView {
    let attributes = String::from(
        "flex mt-2 pl-4 pt-2 pb-2 pr-8 text-left rounded-lg text-base align-middle items-center hover:underline",
    );

    view! {
    <a href={link} class="ml-2 w-full">
        <div class=attributes>
            <span class="pr-2">
                {icon}
            </span>
            {text}
            <span class="ml-auto">
                <BoxLinkIcon />
            </span>
        </div>
    </a>
    }
}

#[component]
fn SideBarNavigation() -> impl IntoView {
    view! {
        <div class="flex flex-col mt-10 font-sans items-start justify-top text-gray-100 text-base w-full">
            <SideBarItem text="Home".into() path="/".into() icon=HouseIcon />
            <SideBarItem text="Experience".into() path="/experience".into() icon=OfficeIcon />
            <SideBarItem text="Projects".into() path="/projects".into() icon=ProjectIcon />
            <SideBarItem text="About".into() path="/about".into() icon=UserIcon />
            <SideBarItem text="Contact".into() path="/contact".into() icon=MailIcon />
            <SideBarItem text="Tools".into() path="/tools".into() icon=WrenchIcon />
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
