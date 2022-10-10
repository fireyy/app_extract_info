use std::{path::PathBuf, ffi::OsStr};

use error::{Error, ExtResult};
use manifest::{
    Manifest,
    IPA_EXT, IpaManifest,
    APK_EXT, ApkManifest,
};

pub mod error;
pub mod manifest;

pub fn get_loaders(path: &PathBuf) -> ExtResult<Manifest> {
    // let mut loader_vec = Vec::new();
    let ext = path
        .extension()
        .and_then(OsStr::to_str);
    
    match ext {
        Some(str) => {
            if str == IPA_EXT {
                IpaManifest::from_path(path)
            } else if str == APK_EXT {
                ApkManifest::from_path(path)
            } else {
                Err(Error::InvalidFile)
            }
        },
        None => Err(Error::InvalidFile)
    }
}