use std::{fs, path::PathBuf};

use app_dirs::{app_root, AppDataType, AppInfo};
use serde::{Deserialize, Serialize};

use crate::error::Result;

const APP_INFO: AppInfo = AppInfo {
    name: "orgize-sync",
    author: "PoiScript",
};

pub fn user_conf_path() -> PathBuf {
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

pub fn user_cache_path() -> PathBuf {
    app_root(AppDataType::UserCache, &APP_INFO).unwrap()
}

pub fn default_conf_path() -> PathBuf {
    user_conf_path().join("config.json")
}

pub fn default_env_path() -> PathBuf {
    user_cache_path().join(".env")
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Conf {
    #[cfg(feature = "dotenv")]
    pub env_path: PathBuf,
    pub up_days: i64,
    pub down_days: i64,
    #[cfg(feature = "google_calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarGlobalConf>,
    pub files: Vec<FileConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            #[cfg(feature = "dotenv")]
            env_path: default_env_path(),
            up_days: 7,
            down_days: 7,
            #[cfg(feature = "google_calendar")]
            google_calendar: None,
            files: Vec::new(),
        }
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

impl Conf {
    pub fn init() -> Result<()> {
        fs::create_dir_all(user_conf_path())?;
        fs::create_dir_all(user_cache_path())?;

        let conf_path = default_conf_path();
        let env_path = default_env_path();

        if cfg!(feature = "dotenv") {
            if env_path.as_path().exists() {
                println!(
                    "{} already existed, skipping ...",
                    env_path.as_path().display()
                );
            } else {
                println!("Creating {} ...", env_path.as_path().display());
                fs::write(&env_path, "")?;
            }
        }

        if conf_path.as_path().exists() {
            println!(
                "{} already existed, skipping ...",
                conf_path.as_path().display()
            );
        } else {
            println!("Creating {} ...", conf_path.as_path().display());
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
        let conf_path = path.unwrap_or_else(default_conf_path);

        let content = fs::read(&conf_path).expect(&format!(
            "Failed to read config file: {}",
            conf_path.as_path().display()
        ));

        if cfg!(feature = "dotenv") {
            let env_conf: EnvConf = serde_json::from_slice(&content)?;
            if env_conf.env_path.as_path().exists() {
                dotenv::from_path(env_conf.env_path.as_path())?;
            }
        }

        Ok(serde_json::from_slice(&content)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConf {
    pub name: Option<String>,
    pub path: String,
    #[cfg(feature = "google_calendar")]
    #[serde(rename = "google-calendar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar: Option<GoogleCalendarConf>,
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
    }

    impl Default for GoogleCalendarConf {
        fn default() -> Self {
            GoogleCalendarConf {
                calendar: String::new(),
                append_new: false,
                append_headline: "Sync".into(),
                property: "EVENT_ID".into(),
            }
        }
    }
}

#[cfg(feature = "google_calendar")]
pub use google_calendar::{GoogleCalendarConf, GoogleCalendarGlobalConf};
