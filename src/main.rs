use leptos::*;
use tracing_subscriber::{
    fmt,
    prelude::*,
    EnvFilter,
};

mod api;
mod app;
mod components;
mod data;
mod sections;

use crate::app::App;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().without_time().with_line_number(true))
        .with(EnvFilter::from_default_env())
        .init();

    mount_to_body(move || {
        view! { <App /> }
    })
}
