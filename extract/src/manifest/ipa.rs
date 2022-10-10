use serde::Deserialize;
use plist::{Value};
use crate::{
    error::{ExtResult},
};
use super::Manifest;

pub const IPA_EXT: &str = "ipa";

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
}

fn find_ipa_icon_path (data: Value) -> String {
    let mut str = String::from(".app/Icon.png");
    if let Some(icon) = data.as_dictionary()
        .and_then(|icon| icon.get("CFBundlePrimaryIcon"))
        .and_then(|icons| icons.as_dictionary())
        .and_then(|icons| icons.get("CFBundleIconFiles"))
        .and_then(|icons| icons.as_array())
        .and_then(|icons| icons.last())
        .and_then(|icon| icon.as_string())
        .and_then(|f| Some(f.to_string()))
    {
        str = icon
    }

    str
}