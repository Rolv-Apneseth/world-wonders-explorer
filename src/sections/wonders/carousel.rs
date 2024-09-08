use std::{
    borrow::Cow,
    ops::Deref,
};

use html::Div;
use leptos::*;
use leptos_icons::Icon;
use leptos_use::{
    breakpoints_tailwind,
    use_breakpoints,
    use_element_hover,
    BreakpointsTailwind,
};

use crate::data::Wonder;

/// Number of images to load ahead in the carousel
const LOAD_AHEAD_IMAGES: usize = 1;

/// Utility function to format an image URL from a known provider to the same image with a given
/// width.
fn get_image_link_with_width(src: &String, width: u16) -> Cow<String> {
    if src.starts_with("https://upload.wikimedia.org/") {
        if let Some((_, filename)) = src.rsplit_once('/') {
            return Cow::Owned(format!(
                "{}/{width}px-{filename}",
                src.replace("commons", "commons/thumb")
            ));
        }
    } else if src.starts_with("https://cdn.britannica.com")
        || src.starts_with("https://dynamic-media-cdn.tripadvisor.com")
    {
        return Cow::Owned(format!("{src}?w={width}"));
    }

    Cow::Borrowed(src)
}

#[derive(Debug, Clone)]
struct CarouselState {
    /// Number of images available to be loaded
    max: usize,
    /// Number of images to load
    load: usize,
    /// Current index of the image being shown
    index: usize,
}

impl CarouselState {
    /// Whether the current index is pointing to the first image
    fn is_at_start(&self) -> bool {
        self.index == 0
    }

    /// Whether the current index is pointing to the last image
    fn is_at_end(&self) -> bool {
        self.index == self.max - 1
    }

    /// Decrement the current index
    fn prev(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        }
    }

    /// Increment the current index
    fn next(&mut self) {
        if self.index == self.max - 1 {
            return;
        }

        self.index += 1;
        if self.index >= self.load - LOAD_AHEAD_IMAGES {
            self.load += 1;
        }
    }
}

#[component]
#[allow(clippy::needless_lifetimes)]
pub fn Carousel<'a>(#[prop()] wonder: &'a Wonder) -> impl IntoView {
    let images = store_value(wonder.links.images.clone());
    let carousel_ref: NodeRef<Div> = create_node_ref();

    let is_hovered = use_element_hover(carousel_ref);
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let is_small_screen = screen_width.le(BreakpointsTailwind::Md);

    let (state, set_state) = create_signal(CarouselState {
        max: wonder.links.images.len(),
        load: 1 + LOAD_AHEAD_IMAGES,
        index: 0,
    });

    view! {
        <div class="relative w-full" _ref=carousel_ref>
            <ul class="w-full">
                {move || {
                    images()
                        .iter()
                        .take(state().load)
                        .enumerate()
                        .map(|(i, src)| {
                            view! {
                                <li class=move || {
                                    format!(
                                        "relative fade w-full flex justify-center shrink-0 {}",
                                        if i == state().index { "block" } else { "hidden" },
                                    )
                                }>
                                    <img
                                        class="object-cover h-56"
                                        src=get_image_link_with_width(src, 600).deref()
                                        loading="lazy"
                                    />

                                    // Avoid misclicks when using the carousel controls by setting `inset-x`
                                    <a
                                        class="absolute inset-y-0 inset-x-8"
                                        href=src
                                        target="_blank"
                                        rel="noreferrer noopener"
                                    >
                                        <span class="sr-only">Link to full-size image</span>
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>

            <div class=move || {
                if is_small_screen() || is_hovered() {
                    "transition-opacity duration-300 opacity-100"
                } else {
                    "transition-opacity duration-300 opacity-0"
                }
            }>
                <button
                    type="button"
                    disabled=move || state().is_at_start()
                    on:click=move |_| { set_state.update(|s| s.prev()) }
                    class="flex absolute top-0 z-30 justify-center items-center px-1 h-full cursor-pointer outline-none disabled:hidden start-0 group motion-safe:transition-colors dark:hover:bg-gray-900/60 dark:hover:bg-gray-900/80 hover:bg-gray-900/60"
                >
                    <span class="text-gray-700 dark:text-gray-300 group-hover:text-gray-300 motion-safe:transition-colors dark:group-hover:text-gray-200">
                        <Icon class="text-2xl drop-shadow-md" icon=icondata::BiChevronLeftSolid />
                        <span class="sr-only">Previous</span>
                    </span>
                </button>
                <button
                    type="button"
                    disabled=move || state().is_at_end()
                    on:click=move |_| { set_state.update(|s| s.next()) }
                    class="flex absolute top-0 z-30 justify-center items-center px-1 h-full cursor-pointer outline-none disabled:hidden end-0 group motion-safe:transition-colors dark:hover:bg-gray-950/60 dark:hover:bg-gray-900/80 hover:bg-gray-900/60"
                >
                    <span class="text-gray-700 dark:text-gray-300 group-hover:text-gray-300 motion-safe:transition-colors dark:group-hover:text-gray-200">
                        <Icon class="text-2xl drop-shadow-lg" icon=icondata::BiChevronRightSolid />
                        <span class="sr-only">Next</span>
                    </span>
                </button>
            </div>
        </div>
    }
}
