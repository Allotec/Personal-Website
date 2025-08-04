use crate::app::footer::CopyrightFooter;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::tachys::view;
use leptos::*;

// TODO: Tools to callout
// Neovim ->  https://neovim.io/
// Rust -> https://www.rust-lang.org/
// latex -> https://www.latex-project.org/
// Copilot -> https://github.com/features/copilot
// Tailscale -> https://tailscale.com/
// Arch Linux -> https://archlinux.org/

#[component]
pub(crate) fn ToolsPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <div>
                <ToolsPageMainContent />
                <ToolsGrid />
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ToolsPageMainContent() -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-4 max-w-2xl w-full mx-auto">
            <div class="flex text-white-100 text-base sm:text-3xl md:text-4xl lg:text-5xl font-sans font-semibold justify-start w-full">
                "Tools"
            </div>
            <div class="flex text-grey-100 text-base justify-start w-full">
                "Tools I frequently use to make life easier"
            </div>
        </div>
    }
}

#[component]
fn ToolsGrid() -> impl IntoView {
    let tools = vec![
        (
            "images/Neovim.svg",
            "Neovim",
            "Editor",
            "https://neovim.io/",
        ),
        (
            "images/Rust.svg",
            "Rust",
            "Language",
            "https://www.rust-lang.org/",
        ),
        (
            "images/Latex.svg",
            "LaTeX",
            "Typesetting",
            "https://www.latex-project.org/",
        ),
        (
            "images/Copilot.svg",
            "Copilot",
            "AI",
            "https://github.com/features/copilot",
        ),
        (
            "images/Tailscale.svg",
            "Tailscale",
            "Networking",
            "https://tailscale.com/",
        ),
        (
            "images/Arch.svg",
            "Arch Linux",
            "OS",
            "https://archlinux.org/",
        ),
    ];

    view! {
        <div class="grid grid-cols-2 grid-rows-3 gap-6 mt-8">
            {tools.into_iter().map(|(logo, name, cat, url)| view! {
                <a href=url target="_blank" rel="noopener noreferrer">
                    <ToolButton logo_path=logo.to_string() logo_name=name.to_string() logo_catagory=cat.to_string() />
                </a>
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn ToolButton(logo_path: String, logo_name: String, logo_catagory: String) -> impl IntoView {
    view! {
        <div class="flex flex-row border hover:bg-[rgb(30,32,34)] rounded-lg pl-4 pt-4 pb-4 pr-10" style="border-color:rgba(255,248,225,0.3);">
            <img src=logo_path.clone() alt=logo_name.clone() class="bg-white h-12 w-12 mb-2" style="width:48px;" />
            <div class="flex flex-col ml-3">
                <div class="flex">{logo_name}</div>
                <div class="flex">{logo_catagory}</div>
            </div>
        </div>
    }
}
