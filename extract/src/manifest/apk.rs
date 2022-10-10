use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use crate::{
    error::{Error, ExtResult},
};
use super::Manifest;

// pub const APK_META_PATH: &str = "AndroidManifest.xml";
pub const APK_EXT: &str = "apk";
// pub const APK_META_PATH: Regex = Regex::new(r"AndroidManifest\.xml").unwrap();

pub struct ApkManifest {

}

impl ApkManifest {
    pub fn from_path(path: &PathBuf) -> ExtResult<Manifest> {
        Ok(
            Manifest::default()
        )
    }
}