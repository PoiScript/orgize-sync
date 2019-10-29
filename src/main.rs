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

use std::path::PathBuf;
use structopt::StructOpt;

use crate::{conf::Conf, error::Result};

#[derive(StructOpt, Debug)]
#[structopt(name = "orgize-sync")]
struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
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

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Init => Conf::init()?,
        Cmd::Conf { conf_path } => {
            let conf = Conf::new(conf_path)?;
            println!("{}", serde_json::to_string_pretty(&conf)?);
        }
        Cmd::Sync {
            conf_path,
            skip_google_calendar,
            skip_toggl,
        } => {
            let _conf = Conf::new(conf_path)?;

            if cfg!(feature = "google_calendar") && !skip_google_calendar {}

            if cfg!(feature = "toggl") && !skip_toggl {}
        }
    }

    Ok(())
}
