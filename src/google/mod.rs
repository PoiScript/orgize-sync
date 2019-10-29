pub mod api;
pub mod auth;
pub mod models;

use chrono::{DateTime, Utc};
use orgize::Org;

pub async fn sync() {}

fn filter_headlines_by_scheduled(
    org: &Org,
    time_min: DateTime<Utc>,
    time_max: DateTime<Utc>,
    updated_min: Option<DateTime<Utc>>,
) {
}
