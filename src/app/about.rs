use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::app::reused_buttons::{ContactButton, EmailButton, LabelTag};
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;

struct Paragraph {
    title: &'static str,
    content: &'static str,
}

static PARAGRAPHS: &[Paragraph] = &[
    Paragraph {
        title: "Who I Am",
        content: "I'm Matthew, an embedded systems engineer focused on low-level software, performance, and building practical tools. I’ve been working professionally close to the metal since 2023.",
    },
    Paragraph {
        title: "What I Do",
        content: "I write firmware, drivers, and automation tools often working solo to bring order to complex hardware-software systems.",
    },
    Paragraph {
        title: "My Journey",
        content: "From bootloaders to graphics pipelines and homelab scripts, I build whatever’s needed especially when it doesn’t exist yet.",
    },
    Paragraph {
        title: "Vision",
        content: "I’m driven by clean, reliable systems. Rust excites me as a way to modernize low-level development without compromising control.",
    },
    Paragraph {
        title: "Beyond Code",
        content: "Into self-hosting, streamlining workflows, and solving tough problems. I like tools that just work and building them when they don’t.",
    },
];

#[component]
pub(crate) fn AboutPageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
            <div class="ml-10 mr-10 mb-5">
                <PageHeading
                    main_title="Matthew"
                    main_color="text-white-100"
                    subtitle="Systems Programmer • Computer Engineer • Stony Brook Alumni"
                    sub_color="text-[#A8A891]"
                />
                <LabelRow />
                {PARAGRAPHS.iter().map(|p| view! {
                    <AboutParagraph title=p.title contens=p.content />
                }).collect::<Vec<_>>()}
                <div class="flex flex-row space-x-3 justify-start w-full mt-4 mb-6">
                    <ContactButton />
                    <EmailButton />
                </div>
            </div>
            <CopyrightFooter />
        </div>
    }
}

#[component]
fn AboutParagraph(title: &'static str, contens: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-2 w-full max-w-2xl text-base mt-5">
            <div class="flex text-white-100 text-lg font-semibold">
                {title}
            </div>
            <div class="flex text-white-100">
                {contens}
            </div>
        </div>
    }
}

#[component]
fn LabelRow() -> impl IntoView {
    view! {
        <div class="flex flex-row w-full space-x-3 mt-5">
            <LabelTag label="Rust" />
            <LabelTag label="STM32" />
            <LabelTag label="Bash" />
            <LabelTag label="Lua" />
            <LabelTag label="Linux" />
            <LabelTag label="AVR" />
            <LabelTag label="Networking" />
        </div>
    }
}

#[component]
fn AboutPageMainContent() -> impl IntoView {
    view! {
        <p> "About Content goes here." </p>
    }
}
