use leptos::*;

use crate::components::external_link::ExternalLink;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col gap-4 mt-10 text-sm leading-tight">
            <p>
                This website is built using
                <ExternalLink href="https://github.com/leptos-rs/leptos" content="Leptos" />
                (Rust + WASM), check out the code
                <ExternalLink
                    href="https://github.com/Rolv-Apneseth/world-wonders-explorer"
                    content="here"
                />!
            </p>

            <p>"© 2024 Rolv Apneseth. All rights reserved."</p>
        </footer>
    }
}
