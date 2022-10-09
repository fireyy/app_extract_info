use std::io::Error as IoError;

use axmldecoder::ParseError as XmlError;
use thiserror::Error;
use plist::Error as PlistError;
use zip::result::ZipError;

pub type ExtResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("IoError: {}", .0)]
    IoError(#[from] IoError),
    #[error("Error while parsing xml")]
    XmlError(#[from] XmlError),
    #[error("PlistError: {}", .0)]
    PlistError(#[from] PlistError),
    #[error("ZipError: {}", .0)]
    ZipError(#[from] ZipError),
    #[error("The file provided is not a valid app file")]
    InvalidFile,
    #[error("The file does not correspond to this type")]
    IncorrectLoader,
}
