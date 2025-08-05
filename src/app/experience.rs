use crate::app::footer::CopyrightFooter;
use crate::app::page_heading::PageHeading;
use crate::icons::BoxLinkIcon;
use crate::{MAIN_PAGE_CLASS, app::MAIN_PAGE_STYLES};
use leptos::prelude::*;
use leptos::*;

#[component]
pub(crate) fn ExperiencePageContent() -> impl IntoView {
    view! {
        <div
            class=MAIN_PAGE_CLASS
            style=MAIN_PAGE_STYLES
        >
        <div class="m-8" >
            <PageHeading
                main_title="Changelog from my journey"
                main_color="text-white-100"
                subtitle="I'b been working at Orbit for the past 2 years."
                sub_color="text-stone-300"
            />
            <div class="text-stone-300">"Here's a timeline of my journey."</div>

            <ExperiencePageMainContent />
        </div>

        <CopyrightFooter />
        </div>
    }
}

#[component]
fn ExperiencePageMainContent() -> impl IntoView {
    view! {
        <div class="mt-20 space-y-10">
            <ExperienceTile
                company="Orbit International Corporation".to_string()
                title="Junior Embedded Systems Design Engineer".to_string()
                date="May 2023 - Present".to_string()
                bullets={vec![
                    "Lead the software development effort for multiple projects including new and legacy products.".to_string(),
                    "Spearheaded modernization efforts by integrating advanced software development workflows into engineering projects.".to_string(),
                    "Setup, documented, and maintained a self-hosted GitLab server for Orbit Instrument’s Engineering department.".to_string()
                ]}
                link="https://orbitintl.com/".to_string()
            />

            <ExperienceTile
                company="Orbit International Corporation".to_string()
                title="Embedded Systems Design Intern".to_string()
                date="Jan 2023 - May 2023".to_string()
                bullets={vec![
                    "Constructed a trackball demo unit to test the feasibility of using an optical sensor for our trackball products.".to_string(),
                    "Learned how to use Orbit’s product lifecycle software, schematic software, and product release process.".to_string()
                ]}
                link="https://orbitintl.com/".to_string()
            />

            <div class="flex flex-row items-center space-x-2">
                <a class="hover:underline flex items-center space-x-2" href="docs/Matthew_Champagne_Resume.pdf" download>
                    <div class="text-white-100 text-lg">
                        "View Full Resume"
                    </div>
                    <BoxLinkIcon />
                </a>
            </div>
        </div>
    }
}

#[component]
fn ExperienceTile(
    company: String,
    title: String,
    date: String,
    bullets: Vec<String>,
    link: String,
) -> impl IntoView {
    view! {
        <a href={link} class="block no-underline">
            <div class="experience-tile shadow-lg rounded-xl p-6 from-gray to-gray-100 border border-amber-100 transition hover:scale-105 hover:shadow-2xl duration-300">
                <div class="flex items-center justify-between mb-2">
                    <h3 class="text-xl font-bold text-white-100">{company}</h3>
                    <span class="text-sm text-white-100 font-medium">{date}</span>
                </div>
                <h4 class="text-lg text-indigo-400 font-semibold mb-2">{title}</h4>
                <ul class="list-disc list-inside space-y-2 text-white-100">
                    {move || bullets.iter().map(|bullet| view! { <li>{bullet.clone()}</li> }).collect::<Vec<_>>()}
                </ul>
            </div>
        </a>
    }
}
