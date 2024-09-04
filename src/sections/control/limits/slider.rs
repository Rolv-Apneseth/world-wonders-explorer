use html::Input;
use leptos::*;

#[component]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
pub fn DoubleSlider(
    #[prop()] lower: (ReadSignal<i16>, WriteSignal<i16>),
    #[prop()] upper: (ReadSignal<i16>, WriteSignal<i16>),
    #[prop(into)] format_label: Callback<i16, String>,
) -> impl IntoView {
    let (lower, set_lower) = lower;
    let (upper, set_upper) = upper;
    let min = lower.get_untracked();
    let max = upper.get_untracked();

    let on_change_lower = move |ev| {
        if let Ok(v) = event_target_value(&ev).parse::<i16>() {
            set_lower(v.min(upper()).max(min));
        }
    };

    let on_change_upper = move |ev| {
        if let Ok(v) = event_target_value(&ev).parse::<i16>() {
            set_upper(v.max(lower()).min(max));
        };
    };

    let slider_lower = create_node_ref::<Input>();
    let slider_upper = create_node_ref::<Input>();

    // Required to make the sliders behave as intended - sliders should not move past each other
    create_effect(move |_| {
        if let Some(e) = slider_lower.get() {
            e.set_value(&lower().to_string());
        }
    });
    create_effect(move |_| {
        if let Some(e) = slider_upper.get() {
            e.set_value(&upper().to_string());
        }
    });

    let range = (max - min) as f64;
    let percent_lower = move || ((lower() - min).abs() as f64 / range * 100f64).round();
    let percent_upper = move || ((upper() - min).abs() as f64 / range * 100f64).round();

    view! {
        <div
            class="flex flex-col gap-1 w-full double-slider"
            // Pass variables to CSS for defining a linear gradient using Tailwind colours
            style=move || {
                let percent_lower = percent_lower();
                let percent_upper = percent_upper();
                format!("--percent_lower: {percent_lower}%; --percent_upper: {percent_upper}%;")
            }
        >
            <div class="relative py-3">
                <input
                    _ref=slider_lower
                    id="slider-lower"
                    // Prevent deadlock of sliders by lowering the z-index if the upper slider is
                    // brought all the way down to the minimum
                    class=move || {
                        format!(
                            "absolute w-full h-0 pointer-events-none {}",
                            if upper() == min { "z-0" } else { "z-10" },
                        )
                    }
                    type="range"
                    min=min
                    max=max
                    step=10
                    default_value=min
                    on:input=on_change_lower
                />
                <input
                    class="absolute w-full h-0.5 pointer-events-none"
                    _ref=slider_upper
                    id="slider-upper"
                    type="range"
                    min=min
                    max=max
                    step=10
                    default_value=max
                    on:input=on_change_upper
                />
            </div>

            <div class="flex gap-4 justify-between">
                <label for="slider_lower">{move || format_label(lower())}</label>
                <label for="slider_upper">{move || format_label(upper())}</label>
            </div>
        </div>
    }
}
