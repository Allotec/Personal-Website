use crate::icons::*;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::hooks::use_location;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

//TODO: Need logic to look good on mobile
#[component]
pub fn SideBar() -> impl IntoView {
    let zoom = get_zoom_signal();

    view! {
        <Show
            when=move || {zoom.get() <= 1.25}
        >
            <VerticalSideBar />
        </Show>

        <Show
            when=move || {zoom.get() > 1.25}
        >
            <HorizontalSideBar />
        </Show>
    }
}

#[component]
fn VerticalSideBar() -> impl IntoView {
    view! {
        <div
            class="flex flex-col pt-12 items-center w-60 bg-[rgb(28,30,32)] h-full overflow-y-auto overflow-x-hidden"
            style="scrollbar-width: auto; scrollbar-color: #444 #222;"
        >
            <ProfileWTitle margin_class="ml-12"/>
            <SideBarNavigation />
            <ConnectWith />
        </div>
    }
}

#[component]
fn HorizontalSideBar() -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = signal(false);

    view! {
        <div class="fixed top-0 left-0 w-full z-50 flex flex-col pt-3 pb-3 items-start bg-[rgb(28,30,32)]">
            <div class="flex flex-row w-full items-center">
                <ProfileWTitle margin_class="ml-6"/>

                <div
                    class="flex ml-auto mr-5 align-middle hover:bg-[rgb(38,40,42)] pl-2 pr-2 pt-2 pb-2 rounded-lg cursor-pointer"
                    on:click=move |_| set_dropdown_open.update(|open| *open = !*open)
                >
                    <HamburgerIcon />
                </div>


                <Show
                    when=move || {dropdown_open.get()}
                >
                    <div class="fixed top-0 left-0 h-full z-60 bg-[rgb(28,30,32)]">
                        <VerticalSideBar />
                    </div>
                </Show>

            </div>
        </div>
    }
}

#[component]
fn ProfileWTitle(margin_class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("flex flex-row space-x-2 align-start w-full {margin_class}")>
            <a href="/">
                <ProfilePic />
            </a>
            <JobTitleAnimation />
        </div>
    }
}

#[component]
fn JobTitleAnimation() -> impl IntoView {
    let titles = [
        "Systems Programmer",
        "Rust Enthusiast",
        "Problem Solver",
        "Neovim Enjoyer",
        "Embedded Systems",
        "Arch BTW",
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
    let is_paused = RwSignal::new(false);

    set_interval(
        {
            move || {
                if is_paused.get() {
                    return;
                }
                let current_title = value.get()[index.get()];
                let chars = current_title.len();

                if typing.get() {
                    if char_count.get() < chars {
                        char_count.update(|c| *c += 1);
                    } else {
                        typing.set(false);
                        is_paused.set(true);
                        set_timeout(move || is_paused.set(false), Duration::from_millis(1750));
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
            <ConnectWithItem text="Github".into() link="https://github.com/Allotec".into() icon=GithubIcon />
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
        attributes.push_str(" bg-black border-solid border-1");
    }

    view! {
    <a href={path} class="ml-4 w-26/30">
        <div class=attributes style="border-color:rgba(255,248,225,0.6);">
            <span class="pr-2">
                {icon}
            </span>
            {text}
        </div>
    </a>
    }
}

fn get_zoom_signal() -> RwSignal<f64> {
    let zoom = RwSignal::new(window().device_pixel_ratio());
    let closure = Closure::wrap(Box::new(move || {
        let ratio = window().device_pixel_ratio();
        zoom.set(ratio);
    }) as Box<dyn FnMut()>);

    window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    zoom
}

fn screen_width() -> i32 {
    window()
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(1024.0) as i32
}
