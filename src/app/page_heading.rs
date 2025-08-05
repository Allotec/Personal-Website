use leptos::prelude::*;

#[component]
pub fn PageHeading(
    main_title: &'static str,
    main_color: &'static str,
    subtitle: &'static str,
    sub_color: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col font-sans space-y-4 max-w-2xl w-full">
            <div class=format!("flex sm:text-3xl md:text-4xl lg:text-5xl xl:text-6xl font-sans font-semibold justify-start w-full {main_color}")>
                {main_title}
            </div>
            <div class=format!("flex text-grey-100 text-base justify-start w-full {sub_color}")>
                {subtitle}
            </div>
        </div>
    }
}
