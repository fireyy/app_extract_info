use core::fmt;

mod ipa;
mod apk;

#[derive(Clone, Debug, Default)]
pub struct Manifest {
    pub name: String,
    pub icon: String,
    pub bundle_id: String,
    pub version: String,
    pub build_number: String,
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(self, f) }
}

pub use ipa::{IPA_EXT, IpaManifest};
pub use apk::{APK_EXT, ApkManifest};