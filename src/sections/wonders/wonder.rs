use leptos::*;

use super::carousel::Carousel;
use crate::{
    components::external_link::ExternalLinkIcon,
    data::Wonder,
};

fn format_build_year(wonder: &Wonder) -> String {
    match wonder.build_year {
        ..0 => format!("{} BCE", wonder.build_year.abs()),
        0.. => format!("{} CE", wonder.build_year),
    }
}

fn format_location(wonder: &Wonder) -> String {
    match wonder.location.rsplit_once(',') {
        Some((l, _)) => l.to_string(),
        None => unreachable!(),
    }
}

#[component]
#[allow(clippy::needless_lifetimes)]
pub fn Wonder<'a>(#[prop()] wonder: &'a Wonder) -> impl IntoView {
    macro_rules! link {
        ($field: ident, $title: expr, $icon: expr) => {
            if wonder.links.$field.is_some() {
                Some(view! {
                    <li title=$title>
                        <ExternalLinkIcon
                            href=&wonder.links.$field.clone().unwrap()
                            icon=$icon
                        />
                    </li>
                })
            } else {
                None
            }
        };
    }

    view! {
        <li class="flex flex-col gap-5 pb-5 rounded-xl border border-gray-200 shadow duration-300 dark:border-gray-700 bg-gray-100/50 lg:motion-safe:hover:bg-white/60 lg:motion-safe:group-hover/list:opacity-50 lg:hover:!opacity-100 lg:dark:motion-safe:hover:bg-zinc-900/50 lg:motion-safe:hover:shadow[inset_0_1px_0_0_rgba(148,163,184,0.1)] lg:motion-safe:hover:drop-shadow-lg dark:bg-zinc-900/30">
            <div class="flex overflow-hidden justify-center w-full bg-gray-200 rounded-t-lg shadow-lg dark:bg-gray-900 dark:shadow-gray-800">
                <Carousel wonder=wonder />
            </div>
            <div class="flex flex-col gap-3 justify-between px-5 grow">
                <section>
                    <h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                        {&wonder.name}
                    </h5>
                    <section class="flex justify-between mt-1 w-full text-xs tracking-tight">
                        <span title="Nearest current day location">{format_location(wonder)}</span>
                        <span title="Estimated year of construction">
                            {format_build_year(wonder)}
                        </span>
                    </section>
                    <p class="mt-3 font-normal text-gray-700 dark:text-gray-400">
                        {&wonder.summary}
                    </p>
                </section>

                <section class="flex flex-col gap-3 text-gray-900 dark:text-white">
                    <h6 class="text-xs font-bold uppercase">Learn more</h6>
                    <ul class="flex gap-4">
                        <li title="Wikipedia">
                            <ExternalLinkIcon href=&wonder.links.wiki icon=icondata::BiWikipedia />
                        </li>
                        {link!(britannica, "Britannica", icondata::LuFlower2)}
                        {link!(trip_advisor,"Trip Advisor", icondata::BiTripAdvisor)}
                        {link!(google_maps,"Google Maps", icondata::SiGooglemaps)}
                    </ul>
                </section>
            </div>
        </li>
    }
}
