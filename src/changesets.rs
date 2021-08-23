//! Changeset dump files
//!
//! Parses the `changesets-latest.osm.bz2` changeset dump file from [https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2](https://planet.openstreetmap.org/planet/changesets-latest.osm.bz2)
//! 
//! The `ChangesetReader` reads the file fully, but `ChangesetTagReader` is optimized to just
//! return the tags
use super::*;
use anyhow::{bail, ensure};
use bzip2::read::MultiBzDecoder;
use quick_xml::events::Event;
use std::fs::*;
use std::io::{BufReader, Read};

/// A single OSM changeset entry
///
/// fields match the XML attributes
#[derive(Debug, Builder)]
pub struct Changeset {
    pub id: u32,
    pub created: TimestampFormat,
    #[builder(setter(strip_option), default)]
    pub closed: Option<TimestampFormat>,
    pub open: bool,
    #[builder(setter(strip_option), default)]
    pub uid: Option<i64>,
    #[builder(setter(strip_option), default)]
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

/// Reads the `changesets-latest.osm.bz2` file and produces `Changesets`
pub struct ChangesetReader<R: Read> {
    reader: quick_xml::Reader<BufReader<R>>,
    buf: Vec<u8>,
}

impl<R: Read> ChangesetReader<R> {
    fn new(reader: R) -> ChangesetReader<R> {
        ChangesetReader {
            reader: quick_xml::Reader::from_reader(BufReader::new(reader)),
            buf: Vec::new(),
        }
    }

    fn next_changeset(&mut self) -> Result<Option<Changeset>> {
        // move forward until we are at a changeset tag (happens at the start)
        let changeset;
        loop {
            match self.reader.read_event(&mut self.buf)? {
                Event::Eof => {
                    return Ok(None);
                }
                Event::Start(ref e) => {
                    if e.name() != "changeset".as_bytes() {
                        continue;
                    }

                    let mut changeset_builder = ChangesetBuilder::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"id" => {
                                changeset_builder
                                    .id(self.reader.decode(&attr.unescaped_value()?)?.parse()?);
                            }
                            b"created_at" => {
                                changeset_builder.created(TimestampFormat::ISOString(
                                    attr.unescape_and_decode_value(&self.reader)?,
                                ));
                            }
                            b"closed_at" => {
                                changeset_builder.closed(TimestampFormat::ISOString(
                                    attr.unescape_and_decode_value(&self.reader)?,
                                ));
                            }
                            b"open" => {
                                changeset_builder.open(match attr.value.as_ref() {
                                    b"true" => true,
                                    b"false" => false,
                                    _ => bail!("unknown value"),
                                });
                            }
                            b"user" => {
                                changeset_builder
                                    .user(attr.unescape_and_decode_value(&self.reader)?);
                            }
                            b"uid" => {
                                changeset_builder
                                    .uid(self.reader.decode(&attr.unescaped_value()?)?.parse()?);
                            }
                            b"num_changes" => {
                                changeset_builder.num_changes(
                                    self.reader.decode(&attr.unescaped_value()?)?.parse()?,
                                );
                            }
                            b"comments_count" => {
                                changeset_builder.comments_count(
                                    self.reader.decode(&attr.unescaped_value()?)?.parse()?,
                                );
                            }
                            _ => {}
                        }
                    }

                    // go for tags
                    let mut tags = HashMap::new();
                    let mut buf = Vec::new();
                    loop {
                        match self.reader.read_event(&mut buf)? {
                            Event::End(ref e) => {
                                if e.name() == "changeset".as_bytes() {
                                    break;
                                }
                            }
                            Event::Start(ref e) | Event::Empty(ref e) => {
                                if e.name() != "tag".as_bytes() {
                                    continue;
                                }
                                let mut k = None;
                                let mut v = None;
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    match attr.key {
                                        b"k" => {
                                            k = Some(attr.unescape_and_decode_value(&self.reader)?);
                                        }
                                        b"v" => {
                                            v = Some(attr.unescape_and_decode_value(&self.reader)?);
                                        }
                                        _ => {}
                                    }
                                }
                                ensure!(k.is_some(), "No k for tag");
                                ensure!(v.is_some(), "No v for tag");
                                tags.insert(k.unwrap(), v.unwrap());
                            }
                            _ => continue,
                        }
                    }

                    changeset_builder.tags(tags);

                    changeset = Some(changeset_builder.build()?);
                    break;
                }
                Event::Empty(ref e) => {
                    if e.name() != "changeset".as_bytes() {
                        continue;
                    }

                    let mut changeset_builder = ChangesetBuilder::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"id" => {
                                changeset_builder
                                    .id(self.reader.decode(&attr.unescaped_value()?)?.parse()?);
                            }
                            b"created_at" => {
                                changeset_builder.created(TimestampFormat::ISOString(
                                    attr.unescape_and_decode_value(&self.reader)?,
                                ));
                            }
                            b"closed_at" => {
                                changeset_builder.closed(TimestampFormat::ISOString(
                                    attr.unescape_and_decode_value(&self.reader)?,
                                ));
                            }
                            b"open" => {
                                changeset_builder.open(match attr.value.as_ref() {
                                    b"true" => true,
                                    b"false" => false,
                                    _ => bail!("unknown value"),
                                });
                            }
                            b"user" => {
                                changeset_builder
                                    .user(attr.unescape_and_decode_value(&self.reader)?);
                            }
                            b"uid" => {
                                changeset_builder
                                    .uid(self.reader.decode(&attr.unescaped_value()?)?.parse()?);
                            }
                            b"num_changes" => {
                                changeset_builder.num_changes(
                                    self.reader.decode(&attr.unescaped_value()?)?.parse()?,
                                );
                            }
                            b"comments_count" => {
                                changeset_builder.comments_count(
                                    self.reader.decode(&attr.unescaped_value()?)?.parse()?,
                                );
                            }
                            _ => {}
                        }
                    }

                    // no tags here
                    changeset_builder.tags(HashMap::new());

                    changeset = Some(changeset_builder.build()?);
                    break;
                }
                _ => continue,
            }
        }

        ensure!(changeset.is_some(), "No changeset created?!");
        Ok(Some(changeset.unwrap()))
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
        let f = File::open(filename)?;
        let dec = MultiBzDecoder::new(f);

        Ok(ChangesetReader::new(dec))
    }
}

/// Reads the `changesets-latest.osm.bz2` file and produces tuples of (id, tags) for every (tagged) changesets.
///
/// Can be quicker than parsing all data.
///
/// Create it with `ChangesetTagReader::from_filename("changesets-latest.osm.bz2")?`
pub struct ChangesetTagReader<R: Read> {
    reader: quick_xml::Reader<BufReader<R>>,
    curr_id: Option<u64>,
    tags: Vec<(String, String)>,
}

impl ChangesetTagReader<bzip2::read::MultiBzDecoder<std::fs::File>> {
    /// Read bz2 zipped filename. 
    pub fn from_filename(filename: &str) -> Result<Self> {
        let f = File::open(filename)?;
        let dec = MultiBzDecoder::new(f);

        Ok(ChangesetTagReader::new(dec))
    }
}


impl<R: Read> ChangesetTagReader<R> {
    fn new(reader: R) -> Self {
        ChangesetTagReader {
            reader: quick_xml::Reader::from_reader(BufReader::new(reader)),
            curr_id: None,
            tags: Vec::new(),
        }
    }

    /// The next changeset (& it's tags)
    fn next_tag(&mut self) -> Result<Option<(u64, Vec<(String, String)>)>> {
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event(&mut buf)? {
                Event::Eof => {
                    return Ok(None);
                },
                Event::End(ref e) => {
                    if e.name() ==  b"changeset" {
                        ensure!(self.curr_id.is_some(), "Should be an id set");

                        return Ok(Some((self.curr_id.unwrap(), std::mem::take(&mut self.tags))));
                    }
                },
                Event::Start(ref e) if e.name() == b"changeset" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key == b"id" {
                            self.curr_id = Some(self.reader.decode(&attr.unescaped_value()?)?.parse()?);
                        }
                    }
                    self.tags.truncate(0);
                },
                Event::Start(ref e)|Event::Empty(ref e) if e.name() == b"tag" => {
                    let mut k = None;
                    let mut v = None;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"k" => {
                                k = Some(attr.unescape_and_decode_value(&self.reader)?);
                            },
                            b"v" => {
                                v = Some(attr.unescape_and_decode_value(&self.reader)?);
                            },
                            _ => continue,
                        }
                    }
                    ensure!(k.is_some(), "No k for tag");
                    ensure!(v.is_some(), "No v for tag");
                    self.tags.push((k.unwrap(), v.unwrap()));
                },
                _ => continue,
            }
        }
    }
}

impl<R: Read> Iterator for ChangesetTagReader<R> {
    type Item = Result<(u64, Vec<(String, String)>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_tag().transpose()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changeset_files() {
        let mut osc = ChangesetTagReader::from_filename("/home/amanda/code/rust/osmio/changeset-examples.osm.bz2").unwrap();
        dbg!(osc.next_tag().unwrap());
    }

}

