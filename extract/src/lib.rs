use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    ffi::OsStr
};
use regex::Regex;
use lazy_static::lazy_static;

use error::{Error, ExtResult};
use manifest::{
    Manifest,
    IPA_EXT, IpaManifest,
    APK_EXT, ApkManifest,
};

pub mod error;
pub mod manifest;

pub const APK_META_PATH: &str = "AndroidManifest.xml";
pub const APK_ARSC_PATH: &str = "resources.arsc";

lazy_static! {
    static ref IPA_META_PATH: Regex = Regex::new(r"Payload/[^/]+\.app/Info\.plist").unwrap();
}

pub fn get_loaders(path: &PathBuf) -> ExtResult<Manifest> {
    let ext = path
        .extension()
        .and_then(OsStr::to_str);
    
    match ext {
        Some(str) => {
            if str == IPA_EXT || str == APK_EXT {
                get_from_path(path, str)
            } else {
                Err(Error::InvalidFile)
            }
        },
        None => Err(Error::InvalidFile)
    }
}

pub fn get_from_path (path: &PathBuf, ext: &str) -> ExtResult<Manifest> {
    let file = File::open(path)?;
    let mut name = String::new();
    let mut manifest = Manifest::default();
    let reader = BufReader::new(file);
    let mut arsc_buf: Vec<u8> = Vec::new();

    let mut archive = zip::ZipArchive::new(reader)?;

    if ext == APK_EXT {
        name = APK_META_PATH.to_string();
        let mut arsc_file = archive.by_name(&APK_ARSC_PATH)?;
        arsc_file.read_to_end(&mut arsc_buf)?;
    } else {
        let names: Vec<String> = archive.file_names().map(ToString::to_string).collect();
        for n in names {
            if ext == IPA_EXT && IPA_META_PATH.is_match(&n) {
                name = n;
                break;
            }
        }
    }

    let mut buf: Vec<u8> = Vec::new();
    archive.by_name(&name)?
        .read_to_end(&mut buf)?;

    if ext == APK_EXT {        
        manifest = ApkManifest::from_buffer(buf, arsc_buf)?;
        let mut icon_buf: Vec<u8> = Vec::new();
        archive.by_name(&manifest.icon)?
            .read_to_end(&mut icon_buf)?;
        manifest.icon = base64::encode(icon_buf);
    } else {
        manifest = IpaManifest::from_buffer(buf)?;
        let mut icon_buf: Vec<u8> = Vec::new();
        let names = archive.file_names()
                        .map(ToString::to_string)
                        .filter(|f| f.contains(&manifest.icon))
                        .collect::<Vec<String>>();
        let name = names.first().unwrap();
        println!("----{:?}", names);
        archive.by_name(&name)?
            .read_to_end(&mut icon_buf)?;
        manifest.icon = base64::encode(icon_buf);
    }

    Ok(manifest)
}