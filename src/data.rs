use std::sync::LazyLock;

use chrono::{
    Datelike,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use strum_macros::{
    Display,
    EnumIter,
    EnumString,
};

pub static MIN_YEAR: i16 = -10000;
pub static CURRENT_YEAR: LazyLock<i16> = LazyLock::new(|| Utc::now().year_ce().1 as i16);

// SHAPE OF WONDERS DATA - these structs and enums are mostly just copied from world-wonders-api
#[derive(
    Clone,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumIter,
    Display,
    EnumString,
)]
pub enum TimePeriod {
    Prehistoric,
    Ancient,
    Classical,
    #[strum(to_string = "Post-Classical")]
    PostClassical,
    #[strum(to_string = "Early Modern")]
    EarlyModern,
    Modern,
}
impl From<TimePeriod> for (i16, i16) {
    fn from(value: TimePeriod) -> Self {
        match value {
            TimePeriod::Prehistoric => (MIN_YEAR, -3000),
            TimePeriod::Ancient => (-2999, -800),
            TimePeriod::Classical => (-799, 500),
            TimePeriod::PostClassical => (501, 1500),
            TimePeriod::EarlyModern => (1501, 1800),
            TimePeriod::Modern => (1801, *CURRENT_YEAR),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Links {
    pub wiki: String,
    pub britannica: Option<String>,
    pub google_maps: Option<String>,
    pub trip_advisor: Option<String>,
    pub images: Vec<String>,
}
#[derive(
    Clone,
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumIter,
    Display,
    EnumString,
)]
pub enum Category {
    #[strum(to_string = "7 Wonders of the Ancient World")]
    SevenWonders,
    #[strum(to_string = "7 Modern Wonders of the World")]
    SevenModernWonders,
    #[strum(to_string = "7 New Wonders of the World")]
    SevenNewWonders,
    #[strum(to_string = "Game - Civilisation 5")]
    Civ5,
    #[strum(to_string = "Game - Civilisation 6")]
    Civ6,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Wonder {
    pub name: String,
    pub summary: String,
    pub location: String,
    pub build_year: i16,
    pub time_period: TimePeriod,
    pub links: Links,
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq)]
pub struct WonderParams {
    pub name: Option<String>,
    pub location: Option<String>,
    pub lower_limit: Option<i16>,
    pub upper_limit: Option<i16>,
    pub category: Option<Category>,
    pub sort_by: Option<SortBy>,
    pub sort_reverse: Option<bool>,
}
impl WonderParams {
    pub fn new() -> Self {
        Self {
            sort_by: Some(SortBy::BuildYear),
            sort_reverse: Some(false),
            lower_limit: Some(MIN_YEAR),
            // FIXME: avoid the need to mod 10 the current year to allow the slider to reach the end
            // (due to step size vs maximum value)
            upper_limit: Some(*CURRENT_YEAR - *CURRENT_YEAR % 10),
            ..Default::default()
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, EnumString, EnumIter, Display)]
pub enum SortBy {
    Alphabetical,
    #[strum(to_string = "Build year")]
    BuildYear,
}

// PAGINATION DATA
pub const PER_PAGE_OPTS: [usize; 3] = [15, 45, 90];

#[derive(Debug, Clone)]
pub struct PaginationState {
    pub per_page: usize,
    pub current_page: usize,
    pub total_pages: usize,
}

impl PaginationState {
    pub fn new() -> Self {
        Self {
            per_page: PER_PAGE_OPTS[0],
            current_page: 0,
            total_pages: 1,
        }
    }

    pub fn reset_current_page(&mut self) {
        self.current_page = 0
    }

    pub fn next_page(&mut self) {
        if self.current_page != self.total_pages {
            self.current_page += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn is_at_start(&self) -> bool {
        self.current_page == 0
    }

    pub fn is_at_end(&self) -> bool {
        self.current_page >= self.total_pages - 1
    }
}
