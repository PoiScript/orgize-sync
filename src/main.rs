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
mod logger;

use log::LevelFilter;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::{conf::Conf, error::Result};

#[derive(StructOpt, Debug)]
#[structopt(name = "orgize-sync")]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    /// Initializes a new configuration file.
    #[structopt(name = "init")]
    Init {
        /// Increases verbosity.
        #[structopt(short, long)]
        verbose: bool,
    },
    /// Synchronizes org files.
    #[structopt(name = "sync")]
    Sync {
        /// Skips Google Calendar synchronization.
        #[cfg(feature = "google_calendar")]
        #[structopt(long = "skip-google-calendar")]
        skip_google_calendar: bool,
        /// Skips Toggl synchronization.
        #[cfg(feature = "toggl")]
        #[structopt(long = "skip-toggl")]
        skip_toggl: bool,
        /// Increases verbosity.
        #[structopt(short, long)]
        verbose: bool,
        /// Path to configuration file.
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
    /// Prints your configuration file.
    #[structopt(name = "conf")]
    Conf {
        /// Increases verbosity.
        #[structopt(short, long)]
        verbose: bool,
        /// Path to configuration file.
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Init { verbose } => {
            init_logger(verbose);

            Conf::init()?;
        }
        Cmd::Conf { verbose, conf_path } => {
            init_logger(verbose);

            let conf = Conf::new(conf_path)?;
            println!("{}", serde_json::to_string_pretty(&conf)?);
        }
        Cmd::Sync {
            verbose,
            conf_path,
            skip_google_calendar,
            skip_toggl,
        } => {
            init_logger(verbose);

            let _conf = Conf::new(conf_path)?;

            if cfg!(feature = "google_calendar") && !skip_google_calendar {}

            if cfg!(feature = "toggl") && !skip_toggl {}
        }
    }

    Ok(())
}

fn init_logger(verbose: bool) {
    log::set_logger(&logger::LOGGER).unwrap();
    if verbose {
        log::set_max_level(LevelFilter::Info);
    } else {
        log::set_max_level(LevelFilter::Trace);
    }
}
