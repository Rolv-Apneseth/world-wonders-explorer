use leptos::error::Result;
use serde::Deserialize;
use serde_qs as qs;
use std::sync::LazyLock;
use tracing::{
    debug,
    error,
};

use crate::data::{
    Category,
    SortBy,
    TimePeriod,
    Wonder,
    WonderParams,
};

/// The base URL for the API
const API_URL: &str = "https://world-wonders-api.org/v0";

/// Append to the base URL for the API
///
/// # Example
/// ```
/// let wonders_url = build_api_url("/wonders");
/// ```
fn build_api_url(ext: impl AsRef<str>) -> String {
    let mut url = API_URL.to_string();
    if !ext.as_ref().is_empty() {
        url.push_str(ext.as_ref());
    };
    url
}

pub static URL_WONDERS: LazyLock<String> = LazyLock::new(|| build_api_url("/wonders"));
pub static URL_CATEGORIES: LazyLock<String> =
    LazyLock::new(|| build_api_url("/wonders/categories"));
pub static URL_TIME_PERIODS: LazyLock<String> =
    LazyLock::new(|| build_api_url("/wonders/time-periods"));
pub static URL_SORT_BY: LazyLock<String> = LazyLock::new(|| build_api_url("/wonders/sort-by"));

/// Build URL for fetching wonders from given [`WonderParams`]
pub fn build_wonders_url(params: WonderParams) -> String {
    let mut url = URL_WONDERS.clone();
    match qs::to_string(&params) {
        Ok(serialised_params) if !serialised_params.is_empty() => {
            url = format!("{url}?{serialised_params}");
        }
        Err(e) => error!("Error encountered while serialising query parameters: {e}"),
        _ => {}
    }

    url
}

/// Expected response for fetching wonders
#[derive(Debug, Deserialize)]
pub struct WondersResponse(pub Vec<Wonder>);

/// Fetch all wonders
pub async fn fetch_wonders(params: WonderParams) -> Result<Vec<Wonder>> {
    let json = reqwasm::http::Request::get(&build_wonders_url(params))
        .send()
        .await?
        .json::<WondersResponse>()
        .await?;

    debug!("Wonders data fetched: {:?}", &json);

    Ok(json.0)
}

/// Expected response for fetching categories
#[derive(Debug, Deserialize)]
pub struct CategoriesResponse(pub Vec<Category>);

/// Fetch all wonder categories
pub async fn fetch_categories(_: ()) -> Result<Vec<Category>> {
    let json = reqwasm::http::Request::get(&URL_CATEGORIES)
        .send()
        .await?
        .json::<CategoriesResponse>()
        .await?;

    debug!("Categories data fetched: {:?}", &json);

    Ok(json.0)
}

/// Expected response for fetching human history time periods
#[derive(Debug, Deserialize)]
pub struct TimePeriodsResponse(pub Vec<TimePeriod>);

/// Fetch all human history time periods
pub async fn fetch_time_periods(_: ()) -> Result<Vec<TimePeriod>> {
    let json = reqwasm::http::Request::get(&URL_TIME_PERIODS)
        .send()
        .await?
        .json::<TimePeriodsResponse>()
        .await?;

    debug!("Time periods data fetched: {:?}", &json);

    Ok(json.0)
}

/// Expected response for fetching wonder sort by options
#[derive(Debug, Deserialize)]
pub struct SortByResponse(pub Vec<SortBy>);

/// Fetch all wonder sort by options
pub async fn fetch_sort_by(_: ()) -> Result<Vec<SortBy>> {
    let json = reqwasm::http::Request::get(&URL_SORT_BY)
        .send()
        .await?
        .json::<SortByResponse>()
        .await?;

    debug!("Time periods data fetched: {:?}", &json);

    Ok(json.0)
}
