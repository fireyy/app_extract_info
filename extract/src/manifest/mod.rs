use std::{
    fs::File,
};
use crate::{
    error::{ExtResult},
};

mod ipa;
mod apk;

#[derive(Clone, Debug, Default)]
pub struct Manifest {
    name: String,
    icon: String,
    bundle_id: String,
    version: String,
    build_number: String,
}

pub use ipa::{IPA_EXT, IpaManifest};
pub use apk::{APK_EXT, ApkManifest};