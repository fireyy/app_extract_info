use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use lazy_static::lazy_static;
use xml::{EventReader, reader::XmlEvent};
use crate::{
    error::{ExtResult},
};
use super::Manifest;

pub const APK_EXT: &str = "apk";
lazy_static! {
    static ref APK_META_PATH: Regex = Regex::new(r"AndroidManifest\.xml").unwrap();
}

pub struct ApkManifest {}

impl ApkManifest {
    pub fn from_buffer(buf: Vec<u8>) -> ExtResult<Manifest> {
        let mut apk_info = Manifest::default();
        apk_info.icon = String::from("res/drawable-xxxhdpi-v4/ic_launcher.png");
        let str = axml::extract_xml(buf);
        let reader = EventReader::from_str(&str);
        for e in reader {
            match e {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                }) => match name.local_name.as_str() {
                    "manifest" => {
                        for attribute in attributes {
                            let attr = attribute.name.to_string();

                            if attr.contains("versionCode") {
                                apk_info.build_number = attribute.value;
                            } else if attr.contains("versionName") {
                                apk_info.version = attribute.value;
                            } else if attr.contains("package") {
                                apk_info.name = attribute.value.clone();
                                apk_info.bundle_id = attribute.value;
                            }
                        }
                        return Ok(apk_info)
                    }
                    _ => {}
                },
                Err(err) => return Err(err.into()),
                _ => {}
            }
        }

        Ok(apk_info)
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
            if APK_META_PATH.is_match(&n) {
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

pub mod axml {
    use axmldecoder::{Cdata, Element, Node};

    pub fn extract_xml(content: Vec<u8>) -> String {
        let xml = axmldecoder::parse(content.as_slice()).unwrap();
        let root = xml.get_root().as_ref().unwrap();
        let mut xml_as_string = String::new();
        format_xml(root, 0_usize, &mut xml_as_string);
        xml_as_string
    }

    fn format_xml(e: &Node, level: usize, output: &mut String) {
        match e {
            Node::Element(e) => {
                output.push_str(&format!(
                    "{:indent$}{}\n",
                    "",
                    &format_start_element(e),
                    indent = level * 2
                ));

                for child in e.get_children() {
                    format_xml(child, level + 1, output)
                }

                if !e.get_children().is_empty() {
                    output.push_str(&format!(
                        "{:indent$}{}\n",
                        "",
                        format_end_element(e),
                        indent = level * 2
                    ));
                }
            }
            Node::Cdata(e) => {
                output.push_str(&format!(
                    "{:indent$}{}\n",
                    "",
                    &format_cdata(e, level),
                    indent = level * 2
                ));
            }
        }
    }

    fn format_cdata(e: &Cdata, level: usize) -> String {
        let indent = format!("{:indent$}", "", indent = level * 2);
        let mut s = String::new();
        s.push_str("<![CDATA[");
        s.push_str(&e.get_data().replace('\n', &format!("\n{}", &indent)));
        s.push_str("]]>");
        s
    }

    fn format_start_element(e: &Element) -> String {
        let mut s = String::new();
        s.push('<');
        s.push_str(e.get_tag());

        if e.get_tag() == "manifest" {
            s.push(' ');
            s.push_str("xmlns:android=\"http://schemas.android.com/apk/res/android\"");
        }

        for (key, val) in e.get_attributes().iter() {
            s.push(' ');
            s.push_str(key);
            s.push('=');
            s.push('"');
            s.push_str(val);
            s.push('"');
        }

        if e.get_children().is_empty() {
            s.push('/');
        }

        s.push('>');

        s
    }

    fn format_end_element(e: &Element) -> String {
        let mut s = String::new();
        s.push('<');
        s.push('/');
        s.push_str(e.get_tag());
        s.push('>');
        s
    }
}