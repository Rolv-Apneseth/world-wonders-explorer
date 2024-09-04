use leptos::*;
use leptos_icons::Icon;

use crate::data::PaginationState;

#[component]
pub fn PagePicker(
    #[prop()] params: ReadSignal<PaginationState>,
    #[prop()] set_params: WriteSignal<PaginationState>,
) -> impl IntoView {
    let button_class = "p-2.5 text-sm border-e border-gray-300 enabled:hover:bg-gray-300 \
                        enabled:hover:text-gray-800 dark:border-gray-700 dark:text-gray-400 \
                        dark:enabled:hover:bg-gray-700 dark:enabled:hover:text-white outline-none \
                        dark:focus:bg-gray-700 focus:bg-gray-300 transition-colors";

    let button_views = move || {
        let p = params();

        (0..p.total_pages)
            .map(|i| {
                view! {
                    <li>
                        <button
                            type="button"
                            on:click=move |_| {
                                if let Some(e) = leptos::document().document_element() {
                                    e.set_scroll_top(0);
                                }
                                params();
                                if params().current_page != i {
                                    set_params.update(|p| p.current_page = i);
                                }
                            }
                            disabled=move || params().current_page == i
                            class=move || {
                                format!(
                                    "{} {}",
                                    button_class,
                                    if params().current_page == i {
                                        "text-black bg-slate-300/50 dark:text-white dark:bg-gray-800"
                                    } else {
                                        ""
                                    },
                                )
                            }
                        >
                            {(i + 1).to_string()}
                        </button>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <ul class="flex overflow-hidden min-w-max rounded-xl border border-gray-300 dark:border-gray-700 no-wrap border-e">
            <li>
                <button
                    type="button"
                    class=format!("{} {}", button_class, "px-1")
                    on:click=move |_| {
                        if !params().is_at_start() {
                            set_params.update(|p| p.prev_page());
                            if let Some(e) = leptos::document().document_element() {
                                e.set_scroll_top(0);
                            }
                        }
                    }
                >
                    <Icon class="text-xl" icon=icondata::BiChevronLeftRegular />
                </button>
            </li>

            {button_views}

            <li>
                <button
                    type="button"
                    class=format!("{} {}", button_class, "px-1 border-e-0")
                    on:click=move |_| {
                        if !params().is_at_end() {
                            set_params.update(|p| p.next_page());
                            if let Some(e) = leptos::document().document_element() {
                                e.set_scroll_top(0);
                            }
                        }
                    }
                >
                    <Icon class="text-xl" icon=icondata::BiChevronRightRegular />
                </button>
            </li>
        </ul>
    }
}
