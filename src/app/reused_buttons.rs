use crate::app::{BUTTON_CLASS, BUTTON_STYLE};
use crate::icons::ClipboardDocument;
use crate::icons::*;
use leptos::prelude::*;
use leptos_use::{UseClipboardReturn, use_clipboard};

#[component]
pub fn LabelTag(label: &'static str) -> impl IntoView {
    view! {
        <div class="flex rounded-full text-sm bg-[rgb(28,30,32)] pl-3 pr-3 pt-1 pb-1">
            {label}
        </div>
    }
}

#[component]
pub fn ContactButton() -> impl IntoView {
    view! {
        <a href="/contact">
            <button class=BUTTON_CLASS style=BUTTON_STYLE>
                "Contact"
            </button>
        </a>
    }
}

#[component]
pub fn ResumeDownload() -> impl IntoView {
    view! {
        <a href="docs/Matthew_Champagne_Resume.pdf" download>
            <button class=BUTTON_CLASS style=BUTTON_STYLE>
                "Resume"
            </button>
        </a>
    }
}

#[component]
pub fn EmailButton() -> impl IntoView {
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
                    format!("{BUTTON_CLASS} border-green-500")
                } else {
                    BUTTON_CLASS.to_string()
                }
            }
            style=move || {
                if copied_state.get() {
                    "".to_string()
                } else {
                    BUTTON_STYLE.to_string()
                }
            }
            on:click=on_click
        >

            <Show
                when=move || {!copied_state.get()}
            >
                <ClipboardDocument />
                <span class="ml-1">"E-Mail"</span>
            </Show>

            <Show
                when=move || {copied_state.get()}
            >
                <ClipboardDocumentCheck />
                <span class="ml-1">"Copied!"</span>
            </Show>

        </button>
    }
}
