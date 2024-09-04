use leptos::*;
use strum::IntoEnumIterator;

use crate::{
    api::fetch_time_periods,
    data::TimePeriod,
};

#[component]
pub fn TimePeriodCombo(
    #[prop()] lower: (ReadSignal<i16>, WriteSignal<i16>),
    #[prop()] upper: (ReadSignal<i16>, WriteSignal<i16>),
) -> impl IntoView {
    let (lower, set_lower) = lower;
    let (upper, set_upper) = upper;
    let min = lower.get_untracked();
    let max = upper.get_untracked();

    let selected = move || {
        let limits = (lower(), upper());
        TimePeriod::iter()
            .find(|t| {
                let l: (i16, i16) = t.clone().into();
                l == limits
            })
            .map(|t| t.to_string())
            .unwrap_or(String::new())
    };

    let resource = create_local_resource(|| (), fetch_time_periods);

    let options_views = move || {
        resource.and_then(|items| {
            items
                .iter()
                .map(|c| {
                    let val = c.to_string();
                    view! { <option value=&val>{val}</option> }
                })
                .collect_view()
        })
    };

    let fallback_error = move |_errors: RwSignal<Errors>| {
        view! {
            <h6 class="text-red-500 dark:text-red-300">
                Error - Failed to fetch available time periods
            </h6>
        }
    };

    view! {
        <ErrorBoundary fallback=fallback_error>
            <select
                title=format!("Select a {}", "Time period")
                class="p-2.5 text-sm bg-transparent rounded-xl border border-gray-300 cursor-pointer dark:placeholder-gray-400 dark:border-gray-600 focus:border-amber-500 dark:focus:border-amber-600"
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    if val.is_empty() {
                        set_lower(min);
                        set_upper(max);
                    } else if let Some((lower, upper)) = TimePeriod::try_from(val.as_str())
                        .ok()
                        .map(|t| t.into())
                    {
                        set_lower(lower);
                        set_upper(upper);
                    }
                }
                prop:value=selected
            >
                <option value="" selected>
                    Custom time period
                </option>

                {options_views}
            </select>
        </ErrorBoundary>
    }
}
