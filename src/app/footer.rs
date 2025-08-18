use std::str::FromStr;

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use js_sys::Reflect;
use js_sys::{Array, Intl, Object};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_use::use_timestamp;

#[component]
pub(crate) fn CopyrightFooter() -> impl IntoView {
    view! {
        <footer class="flex w-full py-4 mt-auto bg-[rgb(0, 0, 9)] border-t border-t-[1px]" style="border-color:rgba(255,248,225,0.5);">
            <div class="flex w-full text-[#B8B8B1] text-center items-center justify-between px-4 text-sm sm:text-base md:text-base lg:text-xs xl:text-xs">
                <a class="hover:underline text-sm sm:text-base md:text-base lg:text-xs xl:text-xs" href="/contact">
                    "Reach out →"
                </a>
                <div class="text-sm sm:text-xs md:text-base lg:text-xs">
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
    let timezone = Tz::from_str(get_browser_timezone().as_str()).unwrap();
    let timestamp = use_timestamp();

    view! {
        <div class="sm:text-base md:text-base lg:text-xs">
            {move || format_time(timestamp.get(), timezone)}
        </div>
    }
}

fn format_time(unix_time: f64, timezone: Tz) -> String {
    let datetime_utc = Utc.timestamp_millis_opt(unix_time as i64).unwrap();
    let datetime_est = datetime_utc.with_timezone(&timezone);
    datetime_est.format("%I:%M:%S %p").to_string()
}

fn get_browser_timezone() -> String {
    let locales = Array::new();
    let options = Object::new();
    let intl = Intl::DateTimeFormat::new(&locales, &options);
    let resolved = intl.resolved_options();
    let tz =
        Reflect::get(&resolved, &JsValue::from_str("timeZone")).unwrap_or(JsValue::from_str("UTC"));
    tz.as_string().unwrap_or_else(|| "UTC".to_string())
}
