use chrono::{TimeZone, Utc};
use chrono_tz::America::New_York;
use leptos::prelude::*;
use leptos::*;
use leptos_use::use_timestamp;

#[component]
pub(crate) fn CopyrightFooter() -> impl IntoView {
    view! {
        <footer class="flex w-full py-4 mt-auto bg-[rgb(0, 0, 9)] border-t border-t-amber-100 border-t-[1px]">
            <div class="flex w-full text-[#B8B8B1] text-center items-center justify-between px-4">
                <a class="hover:underline" href="/contact">
                    "Reach out →"
                </a>
                <div>
                    "Made by Matt | © 2025"
                </div>
                <div>
                    <CurrentTime />
                </div>
            </div>
        </footer>
    }
}

#[component]
fn CurrentTime() -> impl IntoView {
    let timestamp = use_timestamp();

    view! {
        <div>
            {move || format_time(timestamp.get())}
        </div>
    }
}

fn format_time(unix_time: f64) -> String {
    let datetime_utc = Utc.timestamp_millis_opt(unix_time as i64).unwrap();
    let datetime_est = datetime_utc.with_timezone(&New_York);
    datetime_est.format("%I:%M:%S %p").to_string()
}
