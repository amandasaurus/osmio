#![allow(warnings)]
use super::*;
use xml_rs::reader::{EventReader, Events, XmlEvent};
use xml_rs::attribute::{OwnedAttribute};
use xml_rs::name::{OwnedName};
use std::io::{BufReader, Read};
use bzip2::Compression;
use bzip2::read::{BzEncoder, BzDecoder, MultiBzDecoder};
use std::iter::Peekable;
use crate::xml::{get_xml_attribute, extract_attrs};
use std::io::prelude::*;
use std::fs::*;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Changeset {
    pub id: u32,
    pub created: TimestampFormat,
    pub closed: Option<TimestampFormat>,
    pub open: bool,
    pub uid: Option<i64>,
    pub user: Option<String>,
    pub tags: HashMap<String, String>,
    pub num_changes: u64,
    pub comments_count: u64,
}

impl Changeset {

    pub fn tag(&self, key: impl AsRef<str>) -> Option<&str> {
        self.tags.get(key.as_ref()).map(|s| s.as_str())
    }
    pub fn has_tag(&self, key: impl AsRef<str>) -> bool {
        self.tag(key).is_some()
    }
    pub fn num_tags(&self) -> usize {
        self.tags.len()
    }

    /// True iff this object has tags
    pub fn tagged(&self) -> bool {
        !self.untagged()
    }
    /// True iff this object has no tags
    pub fn untagged(&self) -> bool {
        self.num_tags() == 0
    }

    pub fn tags_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.tags
    }

    pub fn into_tags(self) -> HashMap<String, String> {
        self.tags
    }
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

    fn next_changeset(&mut self) -> Result<Option<Changeset>> {
        // move forward until we are at a changeset tag (happens at the start)
        loop {
            match self.parser.peek() {
                None => { return Ok(None); },
                Some(Err(e)) => { return Err(e.clone().into()); }
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


        let mut changeset_el = self.parser.next().unwrap()?;
        let mut attrs = extract_attrs(&mut changeset_el).ok_or(anyhow!("no atts"))?;
        let id = get_xml_attribute(attrs, "id").ok_or(anyhow!("required xml attribute {} not found. attributes: {:?}", "id", attrs))?.parse()?;


        let mut att = |key: &str| -> Result<String> {
            get_xml_attribute(attrs, key).ok_or(anyhow!("required xml attribute {} not found. changeset id {} attributes: {:?}", key, id, attrs))
        };
        let created = TimestampFormat::ISOString(att("created_at")?);
        let closed = att("closed_at").ok().map_or(None, |v| Some(TimestampFormat::ISOString(v)));
        let open = att("open")?.parse()?;
        let uid = att("uid").ok().map(|v| v.parse()).transpose()?;
        let user = att("user").ok().map(|v| v.parse()).transpose()?;
        let num_changes = att("num_changes")?.parse()?;
        let comments_count = att("comments_count")?.parse()?;

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
                      None
                    }
                    ).next().unwrap();
                    let v = attributes.iter().filter_map(|a| {
                          if let &OwnedAttribute{ name: OwnedName{ ref local_name, .. }, ref value } = a { 
                          if local_name == "v" { return Some(value.to_owned()) }
                        }
                        None
                    }
                    ).next().unwrap();
                    tags.insert(k, v);
                }
            }
        }

        let changeset = Changeset {
            id: id,
            created: created,
            closed: closed,
            open: open,
            uid: uid,
            user: user,
            tags: tags,
            num_changes: num_changes,
            comments_count: comments_count,
        };

        Ok(Some(changeset))
    }
}

impl<R: Read> Iterator for ChangesetReader<R> {
    type Item = Result<Changeset>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_changeset().transpose()
    }
}

impl ChangesetReader<bzip2::read::MultiBzDecoder<std::fs::File>> {
    pub fn from_filename(filename: &str) -> Result<Self> {
        let mut f = File::open(filename)?;
        let mut dec = MultiBzDecoder::new(f);
        
        Ok(ChangesetReader::new(dec))
    }
}

pub fn changeset_file_to_tags(filename: &str) -> Result<Vec<Vec<(String, String)>>> {
    let mut osc = ChangesetReader::from_filename(filename)?;
    let mut results = Vec::new();
    let mut cid;
    let mut tags;
    let mut changeset;
    for changeset_res in osc.into_iter() {
        changeset = changeset_res?;
        cid = changeset.id as usize;
        tags = changeset.into_tags();
        if results.len() <= cid {
            results.resize(cid+1, Vec::new());
        }
        results[cid] = tags.into_iter().collect();
    }

    Ok(results)
}


#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::*;
    use super::*;

    #[test]
    fn changeset_files() {
        let mut osc = ChangesetReader::from_filename("/home/amanda/code/rust/osmio/changeset-examples.osm.bz2").unwrap();
        let c = osc.next().unwrap().unwrap();
        assert_eq!(c.id, 2);
        assert_eq!(c.tags.len(), 0);
        let c = osc.next().unwrap().unwrap();
        assert_eq!(c.id, 98870265);
        assert_eq!(c.tags.len(), 5);
        assert!(osc.next().is_none());
    }

    fn changeset_all() {
        let mut osc = ChangesetReader::from_filename("/home/amanda/osm/data/changesets-210208.osm.bz2").unwrap();
        let mut num = 0;
        for c in osc {
            num += 1;
        }
        dbg!(num);
        assert!(false);
    }


}
