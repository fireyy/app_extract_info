use xml::{EventReader, reader::XmlEvent};
use crate::{
    error::{ExtResult},
};
use super::Manifest;

pub const APK_EXT: &str = "apk";

pub struct ApkManifest {}

impl ApkManifest {
    pub fn from_buffer(buf: Vec<u8>) -> ExtResult<Manifest> {
        let mut apk_info = Manifest::default();
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
                    }
                    "application" => {
                        for attribute in attributes {
                            let attr = attribute.name.to_string();

                            if attr.contains("label") {
                                apk_info.name = attribute.value;
                            } else if attr.contains("icon") {
                                apk_info.icon = attribute.value;
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