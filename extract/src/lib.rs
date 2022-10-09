use core::fmt;
use std::{fs::File, io::BufReader};

use error::ExtResult;
use ipa::IPA_META_PATH;
use apk::APK_META_PATH;

pub mod error;
pub mod ipa;
pub mod apk;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum AppLoader {
    Ipa,
    Apk,
}

impl fmt::Display for AppLoader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(self, f) }
}

pub fn get_loaders(file: &File) -> ExtResult<Vec<AppLoader>> {
    let mut loader_vec = Vec::new();

    let reader = BufReader::new(file);
    
    let archive = zip::ZipArchive::new(reader)?;

    let names: Vec<String> = archive.file_names().map(ToString::to_string).collect();

    if names.contains(&IPA_META_PATH.to_string()) {
        loader_vec.push(AppLoader::Ipa);
    }

    if names.contains(&APK_META_PATH.to_string()) {
        loader_vec.push(AppLoader::Apk);
    }

    Ok(loader_vec)
}