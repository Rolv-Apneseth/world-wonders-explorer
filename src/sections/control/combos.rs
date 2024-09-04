use leptos::*;

use crate::{
    api::{
        fetch_categories,
        fetch_sort_by,
    },
    data::{
        Category,
        SortBy,
        WonderParams,
    },
};

/// Macro used to setup combo boxes, e.g. categories and time periods
macro_rules! combo_box {
    ($enum: ident, $fetch_fn: ident, $field: ident, $name: expr) => {
        #[component]
        pub fn $enum(
            #[prop()] params: Signal<WonderParams>,
            #[prop()] set_params: WriteSignal<WonderParams>,
        ) -> impl IntoView {
            // Yes, we could just iterate over the enum, but we want to make use of the different
            // endpoints of the API so instead we fetch them
            let resource = create_local_resource(move || (), $fetch_fn);

            let options_views = move || {
                resource.and_then(|items| {
                    items
                        .iter()
                        .map(|c| {
                            let val = c.to_string();
                            view! { <option value=&val selected={params().$field.as_ref() == Some(c)}>{val}</option> }
                        })
                        .collect_view()
                })
            };

            let fallback_error = move |_errors: RwSignal<Errors>| {
                view! { <h6 class="text-red-500 dark:text-red-300">Error - Failed to fetch data</h6> }
            };

            view! {
                <ErrorBoundary fallback=fallback_error>
                    <select
                        title=format!("Select a {}", $name)
                        class="p-2.5 text-sm bg-transparent rounded-xl border border-gray-300 cursor-pointer dark:placeholder-gray-400 dark:border-gray-600 focus:border-amber-500 dark:focus:border-amber-600"
                        on:change=move |ev| {
                            set_params
                                .update(|p| {
                                    let val = event_target_value(&ev);
                                    p.$field = if val.is_empty() {
                                        None
                                    } else {
                                        $enum::try_from(val.as_str()).ok()
                                    };
                                })
                        }
                        prop:value=move || params.get().$field.map(|c| c.to_string()).unwrap_or_default()
                    >
                        {
                            // Skip default option for "sort by order" as that should just default to the first option
                            if $name != "sort by option" {
                                Some(view!{
                                    <option aria-label=format!( "Default option - any {}", $name) value="" selected>
                                        Any {$name}
                                    </option>
                                })
                            } else { None }
                        }

                        {options_views}
                    </select>
                </ErrorBoundary>
            }
        }
    };
}

combo_box!(Category, fetch_categories, category, "category");
combo_box!(SortBy, fetch_sort_by, sort_by, "sort by option");
