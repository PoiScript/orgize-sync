use app_dirs::{app_root, AppDataType, AppInfo};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::error::Result;

const APP_INFO: AppInfo = AppInfo {
    name: "orgize-sync",
    author: "PoiScript",
};

fn user_conf_path() -> PathBuf {
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

fn user_cache_path() -> PathBuf {
    app_root(AppDataType::UserCache, &APP_INFO).unwrap()
}

fn default_conf_path() -> PathBuf {
    user_conf_path().join("config.json")
}

fn default_env_path() -> PathBuf {
    user_cache_path().join(".env")
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Conf {
    #[cfg(feature = "dotenv")]
    pub env_path: PathBuf,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarGlobalConf>,
    #[cfg(feature = "toggl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggl: Option<TogglGlobalConf>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            #[cfg(feature = "dotenv")]
            env_path: default_env_path(),
            #[cfg(feature = "google_calendar")]
            google_calendar: None,
            #[cfg(feature = "toggl")]
            toggl: None,
            files: Vec::new(),
        }
    }
}

impl Conf {
    pub fn init() -> Result<()> {
        fs::create_dir_all(user_conf_path())?;
        fs::create_dir_all(user_cache_path())?;

        let conf_path = default_conf_path();
        let env_path = default_env_path();

        if cfg!(feature = "dotenv") {
            let env_path = env_path.as_path();
            if env_path.exists() {
                info!(
                    "Dotenv file \"{}\" has already existed. Skipping.",
                    env_path.display()
                );
            } else {
                info!("Creating dotenv file {}.", env_path.display());
                fs::write(&env_path, "")?;
            }
        }

        let conf_path = conf_path.as_path();
        if conf_path.exists() {
            info!(
                "Config file \"{}\" has already existed. Skipping.",
                conf_path.display()
            );
        } else {
            info!("Creating config file \"{}\".", conf_path.display());

            if cfg!(feature = "dotenv") {
                fs::write(
                    conf_path,
                    serde_json::to_string_pretty(&EnvConf { env_path })?,
                )?;
            } else {
                fs::write(conf_path, "")?;
            }
        }

        Ok(())
    }

    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let conf_path = path.unwrap_or_else(|| {
            let path = default_conf_path();
            debug!("Config file is not specified");
            debug!(
                "Using the default config path \"{}\".",
                path.as_path().display()
            );
            path
        });

        trace!(
            "Reading content from \"{}\".",
            conf_path.as_path().display()
        );

        let content = fs::read(&conf_path)?;

        if cfg!(feature = "dotenv") {
            trace!("Serializing content as EnvConf struct.",);

            let env_conf: EnvConf = serde_json::from_slice(&content)?;
            let env_path = env_conf.env_path.as_path();
            debug!(
                "Loading environment variables from \"{}\".",
                env_path.display()
            );
            dotenv::from_path(env_path)?;
        }

        trace!("Serializing content as Conf struct.",);

        Ok(serde_json::from_slice(&content)?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EnvConf {
    pub env_path: PathBuf,
}

impl Default for EnvConf {
    fn default() -> Self {
        EnvConf {
            env_path: default_env_path(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConf {
    pub name: Option<String>,
    pub path: String,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarConf>,
    #[cfg(feature = "toggl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggl: Option<TogglConf>,
}

#[cfg(feature = "google_calendar")]
mod google_calendar {
    use serde::{Deserialize, Serialize};
    use std::{env, path::PathBuf};

    use super::user_cache_path;

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct GoogleCalendarGlobalConf {
        pub client_id: String,
        pub client_secret: String,
        pub token_dir: PathBuf,
        pub token_filename: String,
        pub redirect_uri: String,
    }

    impl Default for GoogleCalendarGlobalConf {
        fn default() -> Self {
            GoogleCalendarGlobalConf {
                client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
                client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
                token_dir: user_cache_path(),
                token_filename: "google-token.json".into(),
                redirect_uri: "http://localhost".into(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct GoogleCalendarConf {
        pub calendar: String,
        pub append_new: bool,
        pub append_headline: String,
        pub property: String,
        pub up_days: u8,
        pub down_days: u8,
    }

    impl Default for GoogleCalendarConf {
        fn default() -> Self {
            GoogleCalendarConf {
                calendar: String::new(),
                append_new: false,
                append_headline: "Sync".into(),
                property: "EVENT_ID".into(),
                up_days: 7,
                down_days: 7,
            }
        }
    }
}

#[cfg(feature = "google_calendar")]
pub use google_calendar::{GoogleCalendarConf, GoogleCalendarGlobalConf};

#[cfg(feature = "toggl")]
mod toggl {
    use serde::{Deserialize, Serialize};
    use std::env;

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct TogglGlobalConf {
        pub api_token: String,
    }

    impl Default for TogglGlobalConf {
        fn default() -> Self {
            TogglGlobalConf {
                api_token: env::var("TOGGL_API_TOKEN").unwrap(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    pub struct TogglConf {
        pub up_days: u8,
        pub down_days: u8,
    }

    impl Default for TogglConf {
        fn default() -> Self {
            TogglConf {
                down_days: 7,
                up_days: 7,
            }
        }
    }
}

#[cfg(feature = "toggl")]
pub use toggl::{TogglConf, TogglGlobalConf};
