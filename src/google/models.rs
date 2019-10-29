use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Event {
    /// Type of the resource ("calendar#event").
    pub kind: String,
    /// Opaque identifier of the event.
    pub id: Option<String>,
    /// Whether this is a locked. The default is False. Read-Only.
    pub locked: bool,
    /// Whether the end time is actually unspecified.
    #[serde(rename = "endTimeUnspecified")]
    pub end_time_unspecified: bool,
    /// The id of the recurring event to which this instance belongs. Immutable.
    #[serde(rename = "recurringEventId")]
    pub recurring_event_id: Option<String>,

    /// The (inclusive) start time of the event.
    pub start: EventDateTime,
    /// The (exclusive) end time of the event.
    pub end: EventDateTime,
    /// Last modification time of the event (as a RFC3339 timestamp). Read-only.
    pub updated: String,
    /// Creation time of the event (as a RFC3339 timestamp). Read-only.
    pub created: String,

    /// Title of the event.
    pub summary: String,
    /// Description of the event. Optional.
    pub description: Option<String>,
    /// Geographic location of this event. Optional.
    pub location: Option<String>,
    /// The color of the event. Optional.
    #[serde(rename = "colorId")]
    pub color_id: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct EventDateTime {
    /// The date, in the format "yyyy-mm-dd", if this is an all-day event.
    pub date: Option<String>,
    /// The time zone in which the time is specified.
    /// (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".)
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    /// The time, as a combined date-time value (formatted according to
    /// RFC3339). A time zone offset is required unless a time zone is
    /// explicitly specified in timeZone.
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
}

impl From<DateTime<Utc>> for EventDateTime {
    fn from(date_time: DateTime<Utc>) -> Self {
        EventDateTime {
            date_time: Some(date_time.to_rfc3339()),
            ..Default::default()
        }
    }
}
