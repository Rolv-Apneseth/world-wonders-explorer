mod slider;
mod time_period_combo;

use leptos::*;

use self::{
    slider::DoubleSlider,
    time_period_combo::TimePeriodCombo,
};

use crate::data::WonderParams;

#[component]
pub fn Limits(
    #[prop()] params: Signal<WonderParams>,
    #[prop()] set_params: WriteSignal<WonderParams>,
) -> impl IntoView {
    let (min, max) = {
        let p = params.get_untracked();
        (p.lower_limit.unwrap(), p.upper_limit.unwrap())
    };
    let lower = create_signal(min);
    let upper = create_signal(max);

    // Keep the `params` synced with `lower` and `upper`, while also ensuring they do not go beyond
    // their defined limits.
    create_effect(move |opt: Option<(i16, i16)>| {
        let (lower, upper) = (lower.0(), upper.0());

        if opt.is_some_and(|(l, _)| lower != l && lower >= min) {
            set_params.update(|p| {
                p.lower_limit = Some(lower);
            });
        };
        if opt.is_some_and(|(_, u)| upper != u && upper <= max) {
            set_params.update(|p| {
                p.upper_limit = Some(upper);
            });
        };

        (lower, upper)
    });

    view! {
        <section class="flex flex-col items-center w-80">
            <TimePeriodCombo lower=lower upper=upper />
            <DoubleSlider
                lower=lower
                upper=upper
                format_label=move |l: i16| {
                    if l < 0 { format!("{} BCE", l.abs()) } else { format!("{l} CE") }
                }
            />
        </section>
    }
}
