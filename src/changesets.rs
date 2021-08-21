#![allow(warnings)]
use super::*;
use xml_rs::reader::{EventReader, Events, XmlEvent};
use xml_rs::attribute::{OwnedAttribute};
use xml_rs::name::{OwnedName};
use std::io::{BufReader, Read};
use bzip2::Compression;
use bzip2::read::{BzEncoder, BzDecoder};
use std::iter::Peekable;
use crate::xml::{get_xml_attribute, extract_attrs};

#[derive(Debug)]
pub struct Changeset {
    id: ObjId,
    created: TimestampFormat,
    closed: Option<TimestampFormat>,
    open: bool,
    uid: i64,
    user: String,
    tags: HashMap<String, String>,

    num_changes: u64,
    comments_count: u64,
}

pub struct ChangesetReader<R: Read>
{
    parser: Peekable<Events<BufReader<R>>>,
}

impl<R: Read> ChangesetReader<R> {

    fn new(reader: R) -> ChangesetReader<R> {
        ChangesetReader {
            parser: EventReader::new(BufReader::new(reader)).into_iter().peekable(),
        }
    }

    fn next(&mut self) -> Option<Changeset> {
        // move forward until we are at a changeset tag (happens at the start)
        loop {
            let next = self.parser.peek();
            match self.parser.peek() {
                None => { return None; }
                Some(Ok(XmlEvent::StartElement { ref name, .. })) => {
                    if name.local_name.as_str() == "changeset" {
                        break;
                    } else {
                        self.parser.next();
                    }
                },
                _ => { self.parser.next(); }
            }
        }

        let mut changeset_el = self.parser.next().unwrap().unwrap();
        let mut attrs = extract_attrs(&mut changeset_el).unwrap();
        let id: ObjId = get_xml_attribute(&mut attrs, "id").and_then(|x| x.parse().ok())?;
        let created = get_xml_attribute(&mut attrs, "created_at")
                .map(|x| TimestampFormat::ISOString(x.to_owned()));
        let closed = get_xml_attribute(&mut attrs, "closed_at")
                .map(|x| TimestampFormat::ISOString(x.to_owned()));
        let open = get_xml_attribute(&mut attrs, "open").and_then(|x| x.parse().ok());
        let uid = get_xml_attribute(&mut attrs, "uid").and_then(|x| x.parse().ok());
        let user = get_xml_attribute(&mut attrs, "user");
        let num_changes = get_xml_attribute(&mut attrs, "num_changes").and_then(|x| x.parse().ok());
        let comments_count = get_xml_attribute(&mut attrs, "comments_count").and_then(|x| x.parse().ok());

        // tags
        let mut tags = HashMap::new();
        loop {
            let next = self.parser.next().unwrap();
            if let Ok(XmlEvent::EndElement{ ref name }) = next {
                if name.local_name == "changeset" {
                    // all done;
                    break;
                }
            }
            if let Ok(XmlEvent::StartElement{ ref name, ref attributes, .. }) = next {
                if name.local_name == "tag" {
                    let k = attributes.iter().filter_map(|a| {
                          if let &OwnedAttribute{ name: OwnedName{ ref local_name, .. }, ref value } = a { 
                          if local_name == "k" { return Some(value.to_owned()) }
                        }
                        return None;
                    }
                    ).next().unwrap();
                    let v = attributes.iter().filter_map(|a| {
                          if let &OwnedAttribute{ name: OwnedName{ ref local_name, .. }, ref value } = a { 
                          if local_name == "v" { return Some(value.to_owned()) }
                        }
                        return None;
                    }
                    ).next().unwrap();
                    tags.insert(k, v);
                }
            }
        }

        let changeset = Changeset {
            id: id,
            created: created.unwrap(),
            closed: closed,
            open: open.unwrap(),
            uid: uid.unwrap(),
            user: user.unwrap(),
            tags: tags,

            num_changes: num_changes.unwrap(),
            comments_count: comments_count.unwrap(),
        };

        Some(changeset)
    }
}

impl<R: Read> Iterator for ChangesetReader<R> {
    type Item = Changeset;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::*;
    use super::*;

    #[test]
    fn changeset_files() {
        let mut f = File::open("/home/amanda/code/rust/osmio/changeset-examples.osm.bz2").unwrap();
        let mut dec = BzDecoder::new(f);
        let mut osc = ChangesetReader::new(dec);
        let c = osc.next().unwrap();
        assert_eq!(c.id, 2);
        assert_eq!(c.tags.len(), 0);
        let c = osc.next().unwrap();
        assert_eq!(c.id, 98870265);
        assert_eq!(c.tags.len(), 5);
        assert!(osc.next().is_none());
    }

}
