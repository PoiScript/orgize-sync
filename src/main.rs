//! Sync your Org with your favorite applications.
//!
//! > This project is still in *alpha stage*. Don't forget to backup your
//! > orgmode files before trying!
//!
//! # Installation
//!
//! ```text
//! $ cargo install orgize-sync
//! ```
//!
//! # Subcommand
//!
//! ## `init`
//!
//! Initializes a new configuration file
//!
//! ```text
//! USAGE:
//!     orgize-sync init [FLAGS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!     -v, --verbose    Increases verbosity
//! ```
//!
//! ## `conf`
//!
//! Prints your configuration file
//!
//! ```text
//! USAGE:
//!     orgize-sync conf [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!     -v, --verbose    Increases verbosity
//!
//! OPTIONS:
//!     -c, --conf <conf-path>    Path to configuration file
//! ```
//!
//! ## `sync`
//!
//! Synchronizes org files
//!
//! ```text
//! USAGE:
//!     orgize-sync sync [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help                    Prints help information
//!         --skip-google-calendar    Skips Google Calendar synchronization
//!         --skip-toggl              Skips Toggl synchronization
//!     -V, --version                 Prints version information
//!     -v, --verbose                 Increases verbosity
//!
//! OPTIONS:
//!     -c, --conf <conf-path>    Path to configuration file
//! ```
//!
//! # Configuration
//!
//! + [General](#general)
//!   + [Global](#global)
//!   + [Pre-file](#pre-file)
//! + [Google Calendar](#google-calendar)
//!   + [Global](#global-1)
//!   + [Pre-file](#pre-file-1)
//! + [Toggl](#toggl)
//!   + [Global](#global-2)
//!   + [Pre-file](#pre-file-2)
//!
//! ## General
//!
//! ### Global
//!
//! ```javascript
//! {
//!     // Path to dotenv file.
//!     // The default is `${UserCacheDir}/orgize-sync/.env`.
//!     "env_path": "./.env",
//!     // Number of days to filter headline before today.
//!     // The default is 7.
//!     "up_days": 1,
//!     // Number of days to filter headline after today.
//!     // The default is 7.
//!     "down_days": 1
//! }
//! ```
//!
//! ### Pre-file
//!
//! ```javascript
//! {
//!     "files": [{
//!         // Specifies the name for this orgmode file. Optional.
//!         "name": "note",
//!         // Specifies the path to orgmode file. Required.
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
//!         // Google OAuth client id. Required.
//!         // Sepcifying here or by setting the `GOOGLE_CLIENT_ID` environment variable.
//!         "client_id": "xxx",
//!         // Google OAuth client secret. Required.
//!         // Sepcifying here or by setting the `GOOGLE_CLIENT_SECRET` environment variable.
//!         "client_secret": "xxx",
//!         // Redirect url after authorizing.
//!         // The default is `http://localhost`
//!         "redirect_uri": "",
//!         // Path to store the access token and refresh token.
//!         // The default is `${UserCacheDir}/orgize-sync`.
//!         "token_dir": "",
//!         // The default is `google-token.json`.
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
//!             // Which calendar to sync. Required.
//!             "calendar": "",
//!             // Whether to append new calendar event to the org mode.
//!             // The default is true.
//!             "append_new": false,
//!             // Where to append new calendar event.
//!             // The default is `Sync`.
//!             "append_headline": "",
//!             // Which property to store event id.
//!             // The default is "EVENT_ID`.
//!             "property": ""
//!         }
//!     }]
//! }
//! ```
//!
//! ## Toggl
//!
//! ### Global
//!
//! ```javascript
//! {}
//! ```
//!
//! ### Pre-file
//!
//! ```javascript
//! {}
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
    /// Initializes a new configuration file
    #[structopt(name = "init")]
    Init {
        /// Increases verbosity
        #[structopt(short, long)]
        verbose: bool,
    },
    /// Synchronizes org files
    #[structopt(name = "sync")]
    Sync {
        /// Skips Google Calendar synchronization
        #[cfg(feature = "google_calendar")]
        #[structopt(long = "skip-google-calendar")]
        skip_google_calendar: bool,
        /// Skips Toggl synchronization
        #[cfg(feature = "toggl")]
        #[structopt(long = "skip-toggl")]
        skip_toggl: bool,
        /// Increases verbosity
        #[structopt(short, long)]
        verbose: bool,
        /// Path to configuration file
        #[structopt(short = "c", long = "conf", parse(from_os_str))]
        conf_path: Option<PathBuf>,
    },
    /// Prints your configuration file
    #[structopt(name = "conf")]
    Conf {
        /// Increases verbosity
        #[structopt(short, long)]
        verbose: bool,
        /// Path to configuration file
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
