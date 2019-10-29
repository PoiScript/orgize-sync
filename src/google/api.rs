use chrono::{DateTime, Utc};
use isahc::prelude::{Request, RequestExt, ResponseExt};
use serde::Deserialize;

use crate::{conf::GoogleCalendarGlobalConf, error::Result, google::models::Event};

#[derive(Deserialize)]
pub struct ConfirmCodeResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

pub async fn confirm_code(
    code: &str,
    conf: &GoogleCalendarGlobalConf,
) -> Result<ConfirmCodeResponse> {
    let body = format!(
        "code={}&client_id={}&client_secret={}&\
         redirect_uri={}&grant_type=authorization_code",
        code, conf.client_id, conf.client_secret, conf.redirect_uri
    );

    let res = Request::post("https://www.googleapis.com/oauth2/v4/token")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)?
        .send_async()
        .await?
        .json::<ConfirmCodeResponse>()?;

    Ok(res)
}

#[derive(Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

pub async fn refresh_token(
    refresh_token: &str,
    conf: &GoogleCalendarGlobalConf,
) -> Result<RefreshTokenResponse> {
    let body = format!(
        "client_id={}&client_secret={}&refresh_token={}&grant_type=refresh_token",
        conf.client_id, conf.client_secret, refresh_token,
    );

    let res = Request::post("https://www.googleapis.com/oauth2/v4/token")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)?
        .send_async()
        .await?
        .json::<RefreshTokenResponse>()?;

    Ok(res)
}

#[derive(Deserialize)]
pub struct ListEventResponse {
    updated: DateTime<Utc>,
    items: Vec<Event>,
}

pub async fn list_events(
    calendar_id: &str,
    time_min: DateTime<Utc>,
    time_max: DateTime<Utc>,
    updated_min: Option<DateTime<Utc>>,
) -> Result<ListEventResponse> {
    let url = if let Some(updated_min) = updated_min {
        format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events\
             ?orderBy=startTime&timeMin={}&timeMax={}&updatedMin={}",
            calendar_id,
            time_min.to_rfc3339(),
            time_max.to_rfc3339(),
            updated_min.to_rfc3339(),
        )
    } else {
        format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events\
             ?orderBy=startTime&timeMin={}&timeMax={}",
            calendar_id,
            time_min.to_rfc3339(),
            time_max.to_rfc3339(),
        )
    };

    let res = isahc::get_async(url).await?.json::<ListEventResponse>()?;

    Ok(res)
}
