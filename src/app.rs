use leptos::*;
use leptos_darkmode::Darkmode;
use leptos_meta::*;
use leptos_use::{
    signal_debounced_with_options,
    DebounceOptions,
};

use crate::{
    components::{
        external_link::ExternalLink,
        theme_toggle::ThemeToggle,
    },
    data::{
        PaginationState,
        WonderParams,
    },
    sections::{
        control::Control,
        footer::Footer,
        wonders::Wonders,
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let darkmode = Darkmode::init();

    let (wonder_params, set_wonder_params) = create_signal(WonderParams::new());
    let wonder_params = signal_debounced_with_options(
        wonder_params,
        100.0,
        DebounceOptions::default().max_wait(Some(1000.0)),
    );

    let (pagination, set_pagination) = create_signal(PaginationState::new());

    let wonders_link_ref: NodeRef<html::A> = create_node_ref();

    // Scroll back to the top of the page on any pagination state update
    create_effect(move |prev: Option<PaginationState>| {
        let curr = pagination();

        if prev.is_some_and(|p| p.current_page == curr.current_page) {
            return curr;
        };

        // HACK: using an anchor tag which points to the `wonders` list to navigate back to the top
        // of the list
        if let Some(e) = wonders_link_ref.get() {
            e.click();
        }
        // TODO: figure out why the below doesn't work specifically for the bottom page picking
        // element (top one works just fine)
        //
        // if let Some(e) = leptos::document().document_element() {
        //     e.set_scroll_top(0);
        // }

        curr
    });

    view! {
        <Html
            lang="en"
            class=move || format!("scroll-smooth {}", if darkmode.is_dark() { "dark" } else { "" })
        />

        <Body class="font-sans antialiased leading-relaxed transition-colors duration-200 bg-slate-200 motion-reduce:transition-none dark:selection:text-cyan-900 dark:bg-gray-950 dark:text-slate-400 dark:selection:bg-cyan-300" />

        <a class="hidden" _ref=wonders_link_ref href="#wonders" />

        <div class="relative py-5 px-6 mx-auto max-w-screen-xl min-h-screen md:py-5 md:px-12 lg:px-24">
            <ThemeToggle />

            <header class="flex flex-col py-6 mb-6 border-b border-gray-700 dark:border-gray-300">
                <h1 class="text-4xl font-bold tracking-tight sm:text-5xl text-slate-900 dark:text-slate-200">
                    World Wonders Explorer
                </h1>
                <h2 class="flex gap-1.5 mt-4 italic leading-normal max-w-m">
                    Discover famous
                    <ExternalLink
                        href="https://en.wikipedia.org/wiki/Wonders_of_the_World"
                        content="Wonders of the World"
                    />
                </h2>
            </header>

            <main class="flex flex-col">
                <Control
                    params=wonder_params
                    set_params=set_wonder_params
                    pagination=pagination
                    set_pagination=set_pagination
                />

                <Wonders
                    wonder_params=wonder_params
                    pagination=pagination
                    set_pagination=set_pagination
                />
            </main>

            <Footer />
        </div>
    }
}
