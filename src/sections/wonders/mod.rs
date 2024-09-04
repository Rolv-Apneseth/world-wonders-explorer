mod carousel;
mod wonder;

use leptos::*;

use self::wonder::Wonder;
use crate::{
    api::fetch_wonders,
    components::{
        external_link::ExternalLink,
        page_picker::PagePicker,
    },
    data::{
        PaginationState,
        Wonder as WonderData,
        WonderParams,
    },
};

#[component]
pub fn Wonders(
    #[prop()] wonder_params: Signal<WonderParams>,
    #[prop()] pagination: ReadSignal<PaginationState>,
    #[prop()] set_pagination: WriteSignal<PaginationState>,
) -> impl IntoView {
    let resource = create_local_resource(
        move || wonder_params.get(),
        move |params| {
            set_pagination.update(|p| p.reset_current_page());
            fetch_wonders(params)
        },
    );

    let fallback_loading = move || {
        view! { <div aria-role="status">"Loading..."</div> }
    };
    let fallback_error = move |_errors: RwSignal<Errors>| {
        view! {
            <section class="ps-5">
                <h6 class="text-red-300">Error - Failed to fetch Wonders from the API.</h6>
                <p class="text-xs">
                    Please try refreshing the page. If the error persists, please open an issue
                    <ExternalLink
                        href="https://github.com/Rolv-Apneseth/world-wonders-explorer/issues"
                        content="here."
                    />
                </p>
            </section>
        }
    };

    let wonders_on_page = move || {
        resource.and_then(|wonders| {
            let PaginationState {
                per_page,
                current_page,
                ..
            } = pagination.get();

            // Update page parameters with total number of pages based on number of wonders
            set_pagination.update(|p| p.total_pages = wonders.len().div_ceil(p.per_page).max(1));

            wonders
                .iter()
                .skip(per_page * current_page)
                .take(per_page)
                .cloned()
                .collect::<Vec<WonderData>>()
        })
    };

    view! {
        <section class="flex flex-col gap-6 py-6">
            <Transition fallback=fallback_loading>
                <ErrorBoundary fallback=fallback_error>
                    <ul
                        id="wonders"
                        class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 group/list"
                    >
                        {wonders_on_page()
                            .map(|r| {
                                r.map(|_| {
                                    view! {
                                        <For
                                            each=move || {
                                                wonders_on_page()
                                                    .transpose()
                                                    .map(|w| w.unwrap_or_default())
                                                    .unwrap()
                                            }
                                            key=|wonder| wonder.name.clone()
                                            children=|wonder| view! { <Wonder wonder=&wonder /> }
                                        />
                                    }
                                })
                            })}
                    </ul>

                    <section class="flex flex-col gap-4 justify-between items-center lg:flex-row">
                        <aside class="text-sm italic shrink">
                            Data provided by the
                            <ExternalLink
                                href="https://github.com/Rolv-Apneseth/world-wonders-api"
                                content="world-wonders-api"
                            />.
                            
                            If you notice any mistakes, or just want more wonders, feel free to contribute or
                            open up an issue
                            <ExternalLink
                                href="https://github.com/Rolv-Apneseth/world-wonders-api/issues"
                                content="here"
                            />.
                        </aside>
                        <PagePicker params=pagination set_params=set_pagination />
                    </section>
                </ErrorBoundary>
            </Transition>
        </section>
    }
}
