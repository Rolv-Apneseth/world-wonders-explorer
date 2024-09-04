use icondata::Icon as IconData;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn ExternalLink(
    #[prop()] href: impl AsRef<str>,
    #[prop()] content: impl IntoView,
) -> impl IntoView {
    view! {
        <a
            class="font-bold underline duration-300 hover:text-black hover:underline group motion-safe:transition-all hover:dark:text-slate-200"
            href=href.as_ref().to_string()
            target="_blank"
            rel="noreferrer noopener"
        >
            {content.into_view()}
        </a>
    }
}

#[component]
pub fn ExternalLinkIcon(
    #[prop()] href: impl AsRef<str>,
    #[prop()] icon: IconData,
) -> impl IntoView {
    view! {
        <ExternalLink
            href=href
            content=view! {
                <Icon
                    class="text-2xl text-gray-900 dark:text-gray-300 motion-safe:transition-all motion-safe:group-hover:text-gray-800 motion-safe:group-hover:-translate-y-1 motion-safe:dark:group-hover:text-white"
                    icon=icon
                />
            }
        />
    }
}
