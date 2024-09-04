use leptos::*;

use crate::data::{
    PaginationState,
    PER_PAGE_OPTS,
};

#[component]
pub fn PerPage(
    #[prop()] pagination: ReadSignal<PaginationState>,
    #[prop()] set_pagination: WriteSignal<PaginationState>,
) -> impl IntoView {
    view! {
        <label>
            <select
                title="Number of results to show per page"
                class="p-2.5 text-sm bg-transparent rounded-xl border border-gray-300 transition-colors cursor-pointer outline-none dark:placeholder-gray-400 dark:text-gray-400 dark:border-gray-600 focus:border-amber-500 dark:focus:border-amber-600"
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    let curr_per_page = pagination().per_page.to_string();
                    if !val.is_empty() && val != curr_per_page {
                        set_pagination
                            .update(|p| {
                                p.reset_current_page();
                                p.per_page = val.parse().unwrap_or(PER_PAGE_OPTS[0]);
                            })
                    }
                }
                prop:value=move || pagination.get().per_page
            >
                {PER_PAGE_OPTS
                    .iter()
                    .map(|v| {
                        view! { <option value=v.to_string()>{v.to_string()}</option> }
                    })
                    .collect_view()}
            </select>
            wonders per page
        </label>
    }
}
