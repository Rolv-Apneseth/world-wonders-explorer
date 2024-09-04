use leptos::*;
use leptos_darkmode::Darkmode;
use leptos_icons::Icon;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let mut darkmode = expect_context::<Darkmode>();

    view! {
        <button
            title="Toggle theme"
            aria-label="Toggles between light and dark themes"
            class="absolute top-3 right-3 py-2 px-3 w-max font-semibold leading-5 text-amber-600 transition-colors duration-200 pointer-events-auto dark:text-gray-300 hover:text-amber-700 dark:hover:text-white"
            on:click=move |_| darkmode.toggle()
        >
            // Regenerate icon element so that the `fade` class's animation triggers between each
            // theme toggle
            {move || {
                let darkmode = expect_context::<Darkmode>();
                view! {
                    <Icon
                        class="text-[1.7rem] fade-fast"
                        icon=if darkmode.is_dark() {
                            icondata::BiMoonSolid
                        } else {
                            icondata::BiSunSolid
                        }
                    />
                }
            }}
        </button>
    }
}
