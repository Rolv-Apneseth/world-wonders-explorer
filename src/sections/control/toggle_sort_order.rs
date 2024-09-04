use leptos::*;
use leptos_icons::Icon;

use crate::data::WonderParams;

#[component]
pub fn ToggleSortOrder(
    #[prop()] params: Signal<WonderParams>,
    #[prop()] set_params: WriteSignal<WonderParams>,
) -> impl IntoView {
    let is_reversed = create_memo(move |_| params().sort_reverse.is_some_and(|s| s));

    view! {
        <button
            type="button"
            title="Toggle sort order"
            class="p-2.5 text-sm rounded-xl border border-gray-300 transition-colors outline-none dark:text-gray-400 dark:border-gray-600 dark:border-gray-700 focus:border-amber-500 enabled:hover:bg-gray-300 enabled:hover:text-gray-800 dark:focus:border-amber-600 dark:enabled:hover:bg-gray-700 dark:enabled:hover:text-white"
            on:click=move |_| set_params.update(|p| p.sort_reverse = p.sort_reverse.map(|s| !s))
        >
            <span class=move || {
                format!(
                    "block transition-transform {}",
                    if is_reversed() { "rotate-180" } else { "" },
                )
            }>
                <Icon icon=icondata::TbSortAscending />
                <span class="sr-only">
                    Sort order: {move || if is_reversed() { "Descending" } else { "Ascending" }}
                </span>
            </span>
        </button>
    }
}
