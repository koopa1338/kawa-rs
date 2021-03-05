use druid::{ExtEventError, PlatformError};
use reqwest::Error as ReqError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Connection was interrupted.")]
    DisconnectError,
    #[error("Request returned an error not a response.")]
    RequestError(#[from] ReqError),
}

#[derive(Error, Debug)]
pub enum GuiError {
    #[error("GUI encountered externel event error.")]
    EventError(#[from] ExtEventError),
    #[error("Platform encountered a shell error.")]
    ShellError(#[from] PlatformError),
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Error while extracting package.")]
    ExtractError,
    #[error("Error while writing data.")]
    WriteError,
    #[error("Error while reading data.")]
    ReadError,
}
