use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn Popover(
    #[prop()] inner: impl IntoView,
    #[prop()] title: impl AsRef<str>,
    children: Children,
) -> impl IntoView {
    let (visible, set_visible) = create_signal(false);

    let container = create_node_ref::<html::Div>();
    let _ = on_click_outside(container, move |_| {
        set_visible(false);
    });

    view! {
        <div _ref=container class="flex relative max-w-max">
            <button
                title=title.as_ref().to_owned()
                class="p-2.5 text-sm rounded-xl border border-gray-300 transition-colors outline-none dark:text-gray-400 dark:border-gray-600 dark:border-gray-700 hover:text-gray-800 hover:bg-gray-300 focus:border-amber-500 dark:focus:border-amber-600 dark:hover:bg-gray-700 dark:hover:text-white"
                on:click=move |e| {
                    e.prevent_default();
                    set_visible.update(|v| *v = !*v);
                }
            >
                {inner}
            </button>

            <div class=move || {
                format!(
                    "shadow-lg dark:shadow-gray-900 p-4 border rounded-xl absolute left-0 bottom-0 -mb-2
border-gray-300 dark:border-gray-600  bg-slate-200 dark:bg-gray-950 dark:text-slate-400
motion-safe:transition-all {}",
                    if visible() {
                        "opacity-100 z-10 translate-y-full"
                    } else {
                        "z-0 opacity-0 pointer-events-none translate-y-[90%]"
                    },
                )
            }>{children()}</div>
        </div>
    }
}
