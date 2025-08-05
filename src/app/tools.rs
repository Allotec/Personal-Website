use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::tachys::view;
use leptos::*;

#[derive(Clone)]
struct Tool {
    logo_path: &'static str,
    logo_name: &'static str,
    logo_category: &'static str,
    url: &'static str,
}

// Matches grid size
const TOOLS: [Tool; 6] = [
    Tool {
        logo_path: "images/Neovim.svg",
        logo_name: "Neovim",
        logo_category: "Editor",
        url: "https://neovim.io/",
    },
    Tool {
        logo_path: "images/Rust.svg",
        logo_name: "Rust",
        logo_category: "Language",
        url: "https://www.rust-lang.org/",
    },
    Tool {
        logo_path: "images/Latex.svg",
        logo_name: "LaTeX",
        logo_category: "Typesetting",
        url: "https://www.latex-project.org/",
    },
    Tool {
        logo_path: "images/Git.svg",
        logo_name: "Git",
        logo_category: "Source Control",
        url: "https://git-scm.com/",
    },
    Tool {
        logo_path: "images/Gitlab.svg",
        logo_name: "Gitlab",
        logo_category: "Self-Hosted Git",
        url: "https://gitlab.com/",
    },
    Tool {
        logo_path: "images/Arch.svg",
        logo_name: "Arch Linux",
        logo_category: "Operating System",
        url: "https://archlinux.org/",
    },
];

#[component]
pub(crate) fn ToolsPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <div class="m-4">
                <PageHeading
                    main_title="Tools"
                    main_color="text-white-100"
                    subtitle="Tools I frequently use to make life easier"
                    sub_color="text-grey-100"
                />
                <ToolsGrid />
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn ToolsGrid() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 grid-rows-3 gap-6 mt-8 mb-10">
            {TOOLS.iter().map(|tool| view! {
                <a href=tool.url target="_blank" rel="noopener noreferrer">
                    <ToolButton
                        logo_path=tool.logo_path.to_string()
                        logo_name=tool.logo_name.to_string()
                        logo_catagory=tool.logo_category.to_string()
                    />
                </a>
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn ToolButton(logo_path: String, logo_name: String, logo_catagory: String) -> impl IntoView {
    view! {
        <div class="flex flex-row border hover:bg-[rgb(28,30,32)] hover:scale-105 rounded-lg pl-4 pt-4 pb-4 pr-10" style="border-color:rgba(255,248,225,0.3);">
            <img src=logo_path.clone() alt=logo_name.clone() class="h-12 w-12 mb-2" style="width:48px;" />
            <div class="flex flex-col ml-3">
                <div class="flex">{logo_name}</div>
                <div class="flex">{logo_catagory}</div>
            </div>
        </div>
    }
}
