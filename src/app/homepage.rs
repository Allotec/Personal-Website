use crate::app::BUTTON_STYLE;
use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;
use leptos_use::{UseClipboardReturn, use_clipboard};

#[component]
pub(crate) fn HomePageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <BigNameTitle />
            <HomePageMainParagraph />
            <CopyrightFooter />
        </div>
    }
}

#[component]
pub(crate) fn BigNameTitle() -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-2 items-center w-9/10 max-w-2xl">
            <div class="flex text-white-100 text-base sm:text-3xl md:text-4xl lg:text-5xl font-sans font-semibold justify-start w-full mx-auto">
                "Hey, I'm Matt"
            </div>
            <div class="flex text-[#A8A891] text-base sm:text-2xl md:text-3xl lg:text-4xl justify-start w-full">
                "Embedded Systems Programmer"
            </div>
        </div>
    }
}

#[component]
fn HomePageMainParagraph() -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-4 items-center w-9/10 max-w-2xl text-base mt-5">
            <HomePageParagraph>
                {"I turn low-level hardware challenges into reliable embedded systems fast, practical, and built to ship."}
            </HomePageParagraph>

            <HomePageParagraph flex=false>
                <span>"Currently I am working as an Embedded Systems Design Engineer at "</span>
                <a class="underline" href="https://orbitintl.com/">
                    "Orbit International Corporation"
                </a>
                <span>" building military HMI devices."</span>
            </HomePageParagraph>

            <HomePageParagraph flex=false>
                <span>
                    "You can talk to me about embedded systems, Rust, hardware quirks, or anything else you’re building."
                </span>
            </HomePageParagraph>

            <HomePageParagraph flex=false>
                <span>"Come Say Hi on "</span>
                <a class="underline font-bold" href="https://orbitintl.com/">
                    "X"
                </a>
            </HomePageParagraph>

            <div class="flex flex-row space-x-3 justify-start w-full mt-4">
                <ContactButton />
                <ResumeDownload />
                <EmailButton />
            </div>
        </div>
    }
}

#[component]
fn ContactButton() -> impl IntoView {
    view! {
        <a href="/contact">
            <button class= BUTTON_STYLE>
                "Contact"
            </button>
        </a>
    }
}

#[component]
fn ResumeDownload() -> impl IntoView {
    view! {
        <a href="/docs/Matthew_Champagne_Resume.pdf" download>
            <button class= BUTTON_STYLE>
                "Resume"
            </button>
        </a>
    }
}

#[component]
fn EmailButton() -> impl IntoView {
    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard();

    let (copied_state, set_copied_state) = signal(false);

    let on_click = {
        let copy = copy.clone();
        move |_| {
            if is_supported.get() {
                copy("champagne7103@gmail.com");
                set_copied_state.set(true);
                set_timeout(
                    move || set_copied_state.set(false),
                    std::time::Duration::from_secs(3),
                );
            }
        }
    };

    view! {
        <button
            class=move || {
                if copied_state.get() {
                    format!("{BUTTON_STYLE} border-green-500")
                } else {
                    BUTTON_STYLE.to_string()
                }
            }
            on:click=on_click
        >
            {move || if copied_state.get() { "Copied!" } else { "E-Mail" }}
        </button>
    }
}

#[component]
fn HomePageParagraph(children: Children, #[prop(default = true)] flex: bool) -> impl IntoView {
    let class = if flex {
        "flex text-grey-100 text-base text-lg justify-start w-full mx-auto"
    } else {
        "text-grey-100 text-base text-lg justify-start w-full mx-auto"
    };

    view! {
        <div class={class}>
            {children()}
        </div>
    }
}
