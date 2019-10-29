use log::{debug, info, trace};
use std::fs;
use std::io::{stdin, BufRead};
use std::process;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    conf::GoogleCalendarGlobalConf,
    error::Result,
    google::api::{confirm_code, refresh_token},
};

#[derive(Serialize, Deserialize)]
struct OAuthToken {
    access_token: String,
    expires_at: DateTime<Utc>,
    refresh_token: String,
}

pub async fn access_token(conf: &GoogleCalendarGlobalConf) -> Result<String> {
    let token_path = conf.token_dir.clone().join(&conf.token_filename);

    debug!("Google OAuth token path: {}", token_path.display());

    if let Ok(json) = fs::read_to_string(&token_path) {
        let mut auth: OAuthToken = serde_json::from_str(&json)?;

        if auth.expires_at > Utc::now() {
            Ok(auth.access_token)
        } else {
            info!("Google OAuth token expired. Refreshing.");

            let res = refresh_token(&auth.refresh_token, conf).await?;
            auth.access_token = res.access_token;
            auth.expires_at = Utc::now() + Duration::seconds(res.expires_in);

            trace!("Saving Google OAuth token.");

            fs::write(token_path, serde_json::to_string(&auth)?)?;

            Ok(auth.access_token)
        }
    } else {
        info!(
            "Please visit: https://accounts.google.com/o/oauth2/v2/auth\
             ?client_id={}&redirect_uri={}&scope=https://www.googleapis.com/auth/calendar\
             &response_type=code&access_type=offline",
            conf.client_id, conf.redirect_uri,
        );
        info!("Follow the instructions and paste the code here (press q to quit):");

        for line in stdin().lock().lines() {
            let line = line?;
            let code = line.trim();

            if code.is_empty() {
                continue;
            } else if code == "q" {
                process::exit(1);
            }

            info!("Confirming code.");

            let res = confirm_code(code, conf).await?;

            let auth = OAuthToken {
                access_token: res.access_token,
                expires_at: Utc::now() + Duration::seconds(res.expires_in),
                refresh_token: res.refresh_token,
            };

            trace!("Saving Google OAuth token.");

            fs::write(token_path, serde_json::to_string(&auth)?)?;

            return Ok(auth.access_token);
        }
        process::exit(1);
    }
}
