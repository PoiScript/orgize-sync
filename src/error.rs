use app_dirs::AppDirsError;
use dotenv::Error as EnvError;
use isahc::Error as IsahcError;
use isahc::http::Error as HttpError;
use serde_json::Error as JsonError;
use std::convert::From;
use std::io::Error as IOError;
use url::ParseError as UrlError;

#[derive(Debug)]
pub enum Error {
    AppDirs(AppDirsError),
    Env(EnvError),
    Http(IsahcError),
    IO(IOError),
    Json(JsonError),
    Url(UrlError),
}

impl From<AppDirsError> for Error {
    fn from(err: AppDirsError) -> Self {
        Error::AppDirs(err)
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

impl From<UrlError> for Error {
    fn from(err: UrlError) -> Self {
        Error::Url(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
