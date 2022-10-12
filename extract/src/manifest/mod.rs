use core::fmt;

mod ipa;
mod apk;

pub const RESOURCE_VALUE_TYPE: &str = "ResourceValueType::Reference/";

#[derive(Clone, Debug, Default)]
pub struct Manifest {
    pub name: String,
    // TODO: to base64 string
    pub icon: String,
    pub bundle_id: String,
    pub version: String,
    pub build_number: String,
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(self, f) }
}

impl Manifest {
    fn check (&self) -> Vec<String> {
        let res: Vec<String> = self.keys()
                                .iter()
                                .filter(|k| self.get(k).contains(RESOURCE_VALUE_TYPE))
                                .cloned()
                                .collect();

        res
    }

    fn keys (&self) -> Vec<String> {
        vec![
            "name".into(),
            "icon".into(),
            "bundle_id".into(),
            "version".into(),
            "build_number".into(),
        ]
    }

    fn get (&self, key: &str) -> String {
        match key {
            "name" => self.name.clone(),
            "icon" => self.icon.clone(),
            "bundle_id" => self.bundle_id.clone(),
            "version" => self.version.clone(),
            "build_number" => self.build_number.clone(),
            _ => String::default()
        }
    }
    fn set (&mut self, key: &str, val: String) {
        match key {
            "name" => self.name = val,
            "icon" => self.icon = val,
            "bundle_id" => self.bundle_id = val,
            "version" => self.version = val,
            "build_number" => self.build_number = val,
            _ => {}
        }
    }
}

pub use ipa::{IPA_EXT, IpaManifest};
pub use apk::{APK_EXT, ApkManifest};