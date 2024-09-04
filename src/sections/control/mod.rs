mod combos;
mod limits;
mod popover;
mod toggle_sort_order;

use leptos::*;
use leptos_icons::Icon;
use popover::Popover;

use self::{
    combos::{
        Category,
        SortBy,
    },
    limits::Limits,
    toggle_sort_order::ToggleSortOrder,
};
use crate::{
    components::per_page::PerPage,
    data::{
        PaginationState,
        WonderParams,
    },
};

#[component]
pub fn Control(
    #[prop()] params: Signal<WonderParams>,
    #[prop()] set_params: WriteSignal<WonderParams>,
    #[prop()] pagination: ReadSignal<PaginationState>,
    #[prop()] set_pagination: WriteSignal<PaginationState>,
) -> impl IntoView {
    // Input box for name and location filtering
    macro_rules! search_input {
        ($field: ident, $label: expr) => {
            view!{
            <div class="overflow-hidden relative rounded-xl shrink" title=$label>
                <input
                    id=$label
                    type="text"
                    placeholder=" "
                    prop:value=move ||  params.get().$field.unwrap_or_default()
                    on:input=move |ev| {
                        set_params.update(|p| {
                            let val = event_target_value(&ev);
                            p.$field = if val.is_empty() { None } else { Some(val) };
                        })
                    }
                    class="block px-2.5 pt-5 pb-2.5 text-sm bg-transparent rounded-xl border border-gray-300 appearance-none outline-none dark:border-gray-600 focus:border-amber-500 peer max-w-40 dark:focus:border-amber-600"
                />

                <label
                    for=$label
                    class="absolute top-4 z-10 text-sm duration-300 transform scale-75 -translate-y-4 origin-[0] start-2.5 peer-focus:text-amber-700 peer-focus:dark:text-amber-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto cursor-text"
                >
                    {$label}
                </label>

                <Show when=move || params().$field.is_some()>
                    <button
                        class="block absolute top-0 right-0 py-1 px-2 text-gray-400 duration-300 outline-none dark:text-gray-400 hover:text-gray-700 motion-safe:transition-colors dark:hover:text-gray-300"
                        on:click=move |_| set_params.update(|p| p.$field = None)
                    >
                        <Icon icon=icondata::IoClose class=""/>
                    </button>
                </Show>
            </div>
            }
        };
    }

    view! {
        <div class="flex relative flex-col gap-4">

            <section class="flex flex-wrap gap-2 justify-between items-end">
                <div class="flex gap-2">
                    <Popover
                        title="Filter"
                        inner={
                            view! { <Icon icon=icondata::BiFilterAltSolid /> }
                        }
                    >
                        <fieldset class="flex flex-col gap-5 items-center">
                            <legend class="sr-only">Filter</legend>

                            <section class="flex flex-col gap-5">
                                <section class="flex flex-nowrap gap-2">
                                    {search_input!(name, "Name")}
                                    {search_input!(location, "Location")}
                                </section>
                                <Category params=params set_params=set_params />
                            </section>

                            <Limits params=params set_params=set_params />
                        </fieldset>
                    </Popover>

                    <fieldset class="flex gap-2">
                        <legend class="sr-only">Sort by</legend>
                        <SortBy params=params set_params=set_params />
                        <ToggleSortOrder params=params set_params=set_params />
                    </fieldset>
                </div>
                <PerPage pagination=pagination set_pagination=set_pagination />
            </section>
        </div>
    }
}
