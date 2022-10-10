use core::fmt;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use regex::Regex;
use lazy_static::lazy_static;

use serde::Deserialize;
use plist::{Value};
use crate::{
    error::{Error, ExtResult},
};
use super::Manifest;

pub const IPA_EXT: &str = "ipa";
lazy_static! {
    static ref IPA_META_PATH: Regex = Regex::new(r"Payload/[^/]+\.app/Info\.plist").unwrap();
}

#[derive(Deserialize, Clone, Debug)]
pub struct IpaManifest {
    #[serde(rename = "CFBundleDisplayName")]
    name: String,
    #[serde(rename = "CFBundleIcons")]
    icon: Value,
    #[serde(rename = "CFBundleIdentifier")]
    bundle_id: String,
    #[serde(rename = "CFBundleShortVersionString")]
    version: String,
    #[serde(rename = "CFBundleVersion")]
    build_number: String,
}

impl fmt::Display for IpaManifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(self, f) }
}

impl IpaManifest {
    pub fn from_buffer(buf: Vec<u8>) -> ExtResult<Manifest> {
        match plist::from_bytes::<IpaManifest>(&buf[..]) {
            Ok(metadata) => {
                let Self {
                    name,
                    icon,
                    bundle_id,
                    version,
                    build_number,
                } = metadata;

                let icon = find_ipa_icon_path(icon);

                Ok(
                    Manifest {
                        name,
                        icon,
                        bundle_id,
                        version,
                        build_number,
                    }
                )
            },
            Err(err) => Err(err.into()),
        }
    }

    pub fn from_path(path: &PathBuf) -> ExtResult<Manifest> {
        let mut file = File::open(path)?;
        Self::from_file(&mut file)
    }

    pub fn from_file(file: &mut File) -> ExtResult<Manifest> {
        let mut name = String::new();
        let reader = BufReader::new(file);

        let mut archive = zip::ZipArchive::new(reader)?;

        let names: Vec<String> = archive.file_names().map(ToString::to_string).collect();

        for n in names {
            if IPA_META_PATH.is_match(&n) {
                name = n;
            }
        }

        let file = archive.by_name(&name);

        match file {
            Ok(mut zip_file) => {
                // let mut buf = String::new();
                // let mut buf: [u8; 10] = Default::default();
                let mut buf: Vec<u8> = Vec::new();
                // zip_file.read_to_string(&mut buf)?;
                zip_file.read_to_end(&mut buf)?;

                Ok(Self::from_buffer(buf)?)
            }
            Err(err) => Err(err.into()),
        }
    }
}

fn find_ipa_icon_path (data: Value) -> String {
    let mut str = String::from(".app/Icon.png");
    if let Some(icon) = data.as_dictionary() {
        if let Some(icons) = icon.get("CFBundlePrimaryIcon") {
            if let Some(icons) = icons.as_dictionary() {
                if let Some(icons) = icons.get("CFBundleIconFiles") {
                    if let Some(icons) = icons.as_array() {
                        str = icons.last().unwrap().as_string().unwrap().to_string();
                    }
                }
            }
        }
    }

    str
}