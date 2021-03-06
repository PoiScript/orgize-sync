use app_dirs::AppDirsError;
use chrono::ParseError as ChronoError;
use dotenv::Error as EnvError;
use isahc::http::Error as HttpError;
use isahc::Error as IsahcError;
use serde_json::Error as JsonError;
use std::convert::From;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
    AppDirs(AppDirsError),
    Chrono(ChronoError),
    Env(EnvError),
    Http(IsahcError),
    IO(IOError),
    Json(JsonError),
}

impl From<AppDirsError> for Error {
    fn from(err: AppDirsError) -> Self {
        Error::AppDirs(err)
    }
}

impl From<ChronoError> for Error {
    fn from(err: ChronoError) -> Self {
        Error::Chrono(err)
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        Error::Env(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}

impl From<IsahcError> for Error {
    fn from(err: IsahcError) -> Self {
        Error::Http(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::Http(err.into())
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
