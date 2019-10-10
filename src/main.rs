//! Sync your Org with your favorite applications.
//!
//! **Note**: This project is still in *alpha stage*. Don't forget to backup
//! your orgmode files before trying!
//!
//! # Commands
//!
//! ## `Init`
//!
//! // TODO
//!
//! ## `Sync`
//!
//! // TODO
//!
//! ## `Conf`
//!
//! // TODO
//!
//! # Configuration
//!
//! ## General
//!
//! ### Global
//!
//! ```javascript
//! {
//!     // path to dotenv file
//!     // default is "${UserCacheDir}/orgize-sync/.env"
//!     "env_path": "./.env",
//!     // number of days to filter headline before today
//!     // default is 7
//!     "up_days": 1,
//!     // number of days to filter headline after today
//!     // default is 7
//!     "down_days": 1
//! }
//! ```
//!
//! ### Pre-file
//!
//! ```javascript
//! {
//!     "files": [{
//!         // specify a name for this file, optional
//!         "name": "note",
//!         // path to this orgmode file, required
//!         "path": "./notes.org"
//!     }]
//! }
//! ```
//!
//! ## Google Calendar
//!
//! ### Global
//!
//! ```javascript
//! {
//!     "google-calendar": {
//!         // google oauth client id, required
//!         // specifying here or by setting the GOOGLE_CLIENT_ID environment variable
//!         "client_id": "xxx",
//!         // google oauth client_secret
//!         // sepcifying here or by setting the GOOGLE_CLIENT_SECRET environment variable
//!         "client_secret": "xxx",
//!         // redirect url after authorizing
//!         // default is "http://localhost"
//!         "redirect_uri": "",
//!         // where to store the access token and refresh token
//!         // default is "${UserCacheDir}/orgize-sync"
//!         "token_dir": "",
//!         // default is "google-token.json"
//!         "token_filename": ""
//!     }
//! }
//! ```
//!
//! ### Pre-file
//!
//! ```javascript
//! {
//!     "files": [{
//!         "google-calendar": {
//!             // which calendar to sync, required
//!             "calendar": "",
//!             // whether to append new calendar event to the org mode
//!             // default is true
//!             "append_new": false,
//!             // where to append new calendar event
//!             // default is "Sync"
//!             "append_headline": "",
//!             // which property to store event id
//!             // default is "EVENT_ID"
//!             "property": ""
//!         }
//!     }]
//! }
//! ```

mod conf;
mod error;
#[cfg(feature = "google_calendar")]
mod google;

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::{
    conf::{
        default_config_path, default_env_path, user_cache_path, user_config_path, Conf, EnvConf,
    },
    error::Result,
    google::auth::Auth,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "orgize-sync")]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "sync")]
    Sync {
        #[cfg(feature = "google_calendar")]
        #[structopt(long = "skip-google-calendar")]
        skip_google_calendar: bool,
        #[cfg(feature = "toggl")]
        #[structopt(long = "skip-toggl")]
        skip_toggl: bool,
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
    #[structopt(name = "conf")]
    Conf {
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Init => {
            fs::create_dir_all(user_config_path())?;
            fs::create_dir_all(user_cache_path())?;

            let default_env_path = default_env_path();
            let default_config_path = default_config_path();

            if default_env_path.as_path().exists() {
                println!(
                    "{} already existed, skipping ...",
                    default_env_path.as_path().display()
                );
            } else {
                println!("Creating {} ...", default_env_path.as_path().display());
                fs::write(default_env_path.clone(), "")?;
            }

            if default_config_path.as_path().exists() {
                println!(
                    "{} already existed, skipping ...",
                    default_config_path.as_path().display()
                );
            } else {
                println!("Creating {} ...", default_config_path.as_path().display());
                fs::write(
                    default_config_path,
                    serde_json::to_string_pretty(&EnvConf {
                        env_path: default_env_path,
                    })?,
                )?;
            }
        }
        Cmd::Sync {
            conf_path,
            skip_google_calendar,
            skip_toggl,
        } => {
            let conf = Conf::new(conf_path)?;

            if cfg!(feature = "google_calendar") && !skip_google_calendar {
                if let Some(google_calendar) = conf.google_calendar {
                    let _auth = Auth::new(&google_calendar).await;
                }
            }

            if cfg!(feature = "toggl") && !skip_toggl {}
        }
        Cmd::Conf { conf_path } => {
            let conf = Conf::new(conf_path)?;

            println!("{}", serde_json::to_string_pretty(&conf)?);
        }
    }

    Ok(())
}
