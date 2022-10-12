use std::{io::Cursor};
use xml::{EventReader, reader::XmlEvent};
use crate::{
    error::{ExtResult},
};
use super::Manifest;

pub const APK_EXT: &str = "apk";

pub struct ResourceId {
    id: u32,
}

impl ResourceId {
    pub fn from_parts(package_id: u8, type_id: u8, entry_id: u16) -> ResourceId {
        ResourceId {
            id: ((package_id as u32) << 24) | ((type_id as u32) << 16) | entry_id as u32,
        }
    }

    pub fn from_u32(id: u32) -> ResourceId {
        ResourceId { id }
    }

    pub fn package_id(&self) -> u8 {
        ((self.id & 0xff00_0000) >> 24) as u8
    }

    pub fn type_id(&self) -> u8 {
        ((self.id & 0x00ff_0000) >> 16) as u8
    }

    pub fn entry_id(&self) -> u16 {
        (self.id & 0x0000_ffff) as u16
    }
}

fn find_resource_by_id (table: &arsc::Arsc, key: String) -> Option<String> {
    let mut str = None;
    let mut entry = vec![];
    let id = key.replace("ResourceValueType::Reference/", "").parse::<u32>().unwrap();
    let res_id = ResourceId::from_u32(id);
    let p = table.packages.iter().find(|p| p.id == res_id.package_id().into()).unwrap();
    let t = p.types.iter().find(|t| t.id == res_id.type_id().into()).unwrap();
    
    for e in &t.configs {
        for a in &e.resources.resources {
            if a.spec_id == res_id.entry_id().into() {
                entry.push(&a.value);
            }
        }
    }
    // name_index, data_index, spec_id
    if !entry.is_empty() {
        let a = entry.get(0).unwrap();
        if let arsc::ResourceValue::Plain(b) = &a {
            match table.global_string_pool.strings.get(b.data_index) {
                Some(v) => {
                    str = Some(v.clone())
                },
                None => {}
            }
        }
    }

    str
}

fn parse_resource (buf: Vec<u8>, info: Manifest) -> ExtResult<Manifest> {
    let mut info = info.clone();
    let res = info.check();
    let cursor = Cursor::new(&buf);
    let table = arsc::parse_from(cursor)?;
    for key in res {
        match find_resource_by_id(&table, info.get(&key)) {
            Some(v) => {
                info.set(&key, v);
            }
            None => {}
        }
    }

    Ok(
        info
    )
}

pub struct ApkManifest {}

impl ApkManifest {
    pub fn from_buffer(buf: Vec<u8>, arsc_buf: Vec<u8>) -> ExtResult<Manifest> {
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
                    }
                    _ => {}
                },
                Err(err) => return Err(err.into()),
                _ => {}
            }
        }

        parse_resource(arsc_buf, apk_info)
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