//! XML file format

use super::version;
use super::ObjId;
use super::TimestampFormat;
use super::{Node, OSMObj, OSMObjectType, Relation, Way};
use super::{OSMReader, OSMWriteError, OSMWriter};
use crate::obj_types::{
    StringBounds, StringNode, StringOSMEle, StringOSMObj, StringRelation, StringWay,
};
use bzip2::read::MultiBzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::iter::Iterator;
use std::path::Path;

use anyhow::Result;
use xml_rs::attribute::OwnedAttribute;
use xml_rs::reader::{EventReader, Events, XmlEvent};

pub struct XMLReader<R: Read> {
    parser: Events<BufReader<R>>,
}

pub fn from_filename_bz2(
    filename: impl AsRef<Path>,
) -> Result<XMLReader<bzip2::read::MultiBzDecoder<std::fs::File>>> {
    let filename: &Path = filename.as_ref();
    let f = File::open(filename)?;
    let dec = MultiBzDecoder::new(f);
    Ok(XMLReader::new(dec))
}
pub fn from_filename_uncompressed(
    filename: impl AsRef<Path>,
) -> Result<XMLReader<BufReader<File>>> {
    let filename: &Path = filename.as_ref();
    Ok(XMLReader::new(BufReader::new(File::open(filename)?)))
}

/*
 * TODO, fix this
pub fn from_filename(filename: impl AsRef<Path>) -> Result<XMLReader<dyn Read>>> {
    match filename.as_ref().to_str() {
        None => { anyhow::bail!("filename {:?} cannot automatically determine whether this is compressed or not, call from_filename_bz2(…) or from_filename_uncompressed(…) directly", filename); },
        Some(s) => {
            if s.ends_with(".osm.bz2") {
                from_filename_bz2(s)
            } else if s.ends_with(".osm") {
                from_filename_uncompressed(s)
            } else {
                anyhow::bail!("filename {:?} cannot automatically determine whether this is compressed or not, call from_filename_bz2(…) or from_filename_uncompressed(…) directly", filename)
                }
        },
    }
}
*/

pub(crate) fn write_xml_escaped(writer: &mut impl Write, s: &str) -> std::io::Result<()> {
    for c in s.chars() {
        match c {
            '&' => write!(writer, "&amp;")?,
            '"' => write!(writer, "&quot;")?,
            '\'' => write!(writer, "&apos;")?,
            '<' => write!(writer, "&lt;")?,
            '>' => write!(writer, "&gt;")?,
            c => write!(writer, "{}", c)?,
        }
    }
    Ok(())
}

impl<R: Read> OSMReader for XMLReader<R> {
    type R = R;
    type Ele = StringOSMEle;

    fn new(reader: R) -> XMLReader<R> {
        XMLReader {
            parser: EventReader::new(BufReader::new(reader)).into_iter(),
        }
    }

    fn into_inner(self) -> R {
        self.parser.into_inner().into_inner().into_inner()
    }

    fn inner(&self) -> &R {
        todo!("{} {} XMLReader inner()", file!(), line!());
    }

    fn next(&mut self) -> Option<Self::Ele> {
        let mut elements = Vec::new();

        // Pull xml/sax elements from the xml parser into a vector so we know what to work with.
        let mut should_push = false;
        loop {
            let el = match self.parser.next() {
                None => {
                    break;
                }
                Some(e) => e,
            };

            let el = el.unwrap();

            let mut should_break = false;
            match el {
                XmlEvent::StartElement { ref name, .. } => match name.local_name.as_str() {
                    "bounds" | "node" | "way" | "relation" => {
                        should_push = true;
                    }
                    _ => {}
                },
                XmlEvent::EndElement { ref name, .. } => match name.local_name.as_str() {
                    "bounds" | "node" | "way" | "relation" => {
                        should_break = true;
                    }
                    _ => {}
                },
                _ => {}
            }

            if should_push {
                elements.push(el);
            }
            if should_break {
                break;
            }
        }

        xml_elements_to_osm_obj(&mut elements)
    }
}

// FIXME can I put this enum inside XMLWriter?
#[derive(PartialEq)]
enum State {
    Initial,
    WritingObjects,
    Closed,
}

/// Write as OSM XML file format
pub struct XMLWriter<W: Write> {
    writer: W,
    headers: HashMap<String, String>,
    _state: State,
}

#[allow(unused_variables)]
pub(crate) fn extract_attrs(x: &mut XmlEvent) -> Option<&mut Vec<OwnedAttribute>> {
    match *x {
        XmlEvent::StartElement {
            ref name,
            ref mut attributes,
            ref namespace,
        } => Some(attributes),
        _ => None,
    }
}

pub(crate) fn get_xml_attribute(attrs: &mut Vec<OwnedAttribute>, key: &str) -> Option<String> {
    attrs
        .iter()
        .position(|attr| attr.name.local_name == key)
        .map(|idx| {
            let attr = attrs.swap_remove(idx);
            let OwnedAttribute { name: _, value } = attr;
            value
        })
}

fn get_tags(els: &mut [XmlEvent]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement {
            ref name,
            ref mut attributes,
            ..
        } = el
        {
            if name.local_name == "tag" {
                let ko = get_xml_attribute(attributes, "k");
                let vo = get_xml_attribute(attributes, "v");
                if let (Some(k), Some(v)) = (ko, vo) {
                    result.insert(k, v);
                }
            }
        }
    }

    result
}

fn get_nodes(els: &mut [XmlEvent]) -> Vec<ObjId> {
    let mut result = Vec::new();

    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement {
            ref name,
            ref mut attributes,
            ..
        } = el
        {
            if name.local_name == "nd" {
                let ref_id_o: Option<ObjId> =
                    get_xml_attribute(attributes, "ref").and_then(|e| e.parse().ok());
                if let Some(ref_id) = ref_id_o {
                    result.push(ref_id);
                }
            }
        }
    }

    result
}

fn get_members(els: &mut [XmlEvent]) -> Vec<(OSMObjectType, ObjId, String)> {
    let mut result = Vec::new();

    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement {
            ref name,
            ref mut attributes,
            ..
        } = el
        {
            if name.local_name == "member" {
                let ref_id_o: Option<ObjId> =
                    get_xml_attribute(attributes, "ref").and_then(|e| e.parse().ok());
                let member_type_o: Option<OSMObjectType> =
                    get_xml_attribute(attributes, "type").and_then(|t| t.parse().ok());
                let role = get_xml_attribute(attributes, "role").unwrap_or_default();
                if let (Some(ref_id), Some(member_type)) = (ref_id_o, member_type_o) {
                    result.push((member_type, ref_id, role));
                }
            }
        }
    }

    result
}

pub(crate) fn xml_elements_to_osm_obj(els: &mut [XmlEvent]) -> Option<StringOSMEle> {
    match els.first() {
        Some(&XmlEvent::StartElement { ref name, .. }) => match name.local_name.as_str() {
            "bounds" => bounds_xml_elements_to_osm_obj(els).map(StringOSMEle::Bounds),
            "node" => node_xml_elements_to_osm_obj(els).map(StringOSMEle::Object),
            "way" => way_xml_elements_to_osm_obj(els).map(StringOSMEle::Object),
            "relation" => relation_xml_elements_to_osm_obj(els).map(StringOSMEle::Object),
            _ => None,
        },
        _ => None,
    }
}

fn bounds_xml_elements_to_osm_obj(els: &mut [XmlEvent]) -> Option<StringBounds> {
    let xml = els.first_mut()?;
    let attrs = extract_attrs(xml)?;
    let minlat = get_xml_attribute(attrs, "minlat")?.parse().ok()?;
    let minlon = get_xml_attribute(attrs, "minlon")?.parse().ok()?;
    let maxlat = get_xml_attribute(attrs, "maxlat")?.parse().ok()?;
    let maxlon = get_xml_attribute(attrs, "maxlon")?.parse().ok()?;
    Some(StringBounds {
        min: (minlat, minlon),
        max: (maxlat, maxlon),
    })
}

fn node_xml_elements_to_osm_obj(els: &mut [XmlEvent]) -> Option<StringOSMObj> {
    let attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(attrs, "id").and_then(|x| x.parse().ok())?;
    let version = get_xml_attribute(attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(attrs, "timestamp").map(TimestampFormat::ISOString);
    let uid = get_xml_attribute(attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(attrs, "user");
    let lat = get_xml_attribute(attrs, "lat")?.parse();
    let lon = get_xml_attribute(attrs, "lon")?.parse();

    let lat_lon = match (lat, lon) {
        (Ok(lat), Ok(lon)) => Some((lat, lon)),
        _errs => None,
    };
    let deleted = get_xml_attribute(attrs, "visible")
        .and_then(|val| match val.as_str() {
            "true" => Some(false),
            "false" => Some(true),
            _ => None,
        })
        .unwrap_or(false);

    let tags = get_tags(els);

    Some(StringOSMObj::Node(StringNode {
        _id: id,
        _version: version,
        _deleted: deleted,
        _changeset_id: changeset_id,
        _timestamp: timestamp,
        _uid: uid,
        _user: user,
        _lat_lon: lat_lon,
        _tags: tags,
    }))
}

fn way_xml_elements_to_osm_obj(els: &mut [XmlEvent]) -> Option<StringOSMObj> {
    let attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(attrs, "id").and_then(|x| x.parse().ok())?;
    let version = get_xml_attribute(attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(attrs, "timestamp").map(TimestampFormat::ISOString);
    let uid = get_xml_attribute(attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(attrs, "user");
    let deleted = get_xml_attribute(attrs, "visible")
        .and_then(|val| match val.as_str() {
            "true" => Some(false),
            "false" => Some(true),
            _ => None,
        })
        .unwrap_or(false);

    let tags = get_tags(els);
    let nodes = get_nodes(els);
    Some(StringOSMObj::Way(StringWay {
        _id: id,
        _version: version,
        _deleted: deleted,
        _changeset_id: changeset_id,
        _timestamp: timestamp,
        _uid: uid,
        _user: user,
        _tags: tags,
        _nodes: nodes,
    }))
}

fn relation_xml_elements_to_osm_obj(els: &mut [XmlEvent]) -> Option<StringOSMObj> {
    let attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(attrs, "id").and_then(|x| x.parse().ok())?;
    let version = get_xml_attribute(attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(attrs, "timestamp").map(TimestampFormat::ISOString);
    let uid = get_xml_attribute(attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(attrs, "user");
    let deleted = get_xml_attribute(attrs, "visible")
        .and_then(|val| match val.as_str() {
            "true" => Some(false),
            "false" => Some(true),
            _ => None,
        })
        .unwrap_or(false);

    let tags = get_tags(els);
    let members = get_members(els);
    Some(StringOSMObj::Relation(StringRelation {
        _id: id,
        _version: version,
        _deleted: deleted,
        _changeset_id: changeset_id,
        _timestamp: timestamp,
        _uid: uid,
        _user: user,
        _tags: tags,
        _members: members,
    }))
}

impl From<quick_xml::Error> for OSMWriteError {
    fn from(err: quick_xml::Error) -> OSMWriteError {
        OSMWriteError::XMLWriteXMLError(err)
    }
}

impl From<std::io::Error> for OSMWriteError {
    fn from(err: std::io::Error) -> OSMWriteError {
        OSMWriteError::XMLWriteIOError(err)
    }
}

impl<W: Write> XMLWriter<W> {
    fn ensure_header(&mut self) -> Result<(), OSMWriteError> {
        if self._state == State::Initial {
            writeln!(self.writer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
            write!(
                self.writer,
                "<osm version=\"0.6\" generator=\"osmio/{}\"",
                version()
            )?;

            for (k, v) in self.headers.iter() {
                write!(self.writer, " {}=\"", k)?;
                write_xml_escaped(&mut self.writer, v)?;
                write!(self.writer, "\"")?;
            }
            write!(self.writer, ">")?;

            self._state = State::WritingObjects;
        }
        Ok(())
    }
}

impl<W: Write> OSMWriter<W> for XMLWriter<W> {
    fn new(writer: W) -> Self {
        // TODO have a config that does indentation and stuff
        XMLWriter {
            writer,
            headers: HashMap::new(),
            _state: State::Initial,
        }
    }

    fn set_header(&mut self, (key, value): (&str, &str)) -> Result<(), OSMWriteError> {
        match self._state {
            State::Initial => {
                self.headers.insert(key.into(), value.into());
                Ok(())
            }
            State::Closed => Err(OSMWriteError::AlreadyClosed),
            _ => Err(OSMWriteError::AlreadyStarted),
        }
    }

    fn is_open(&self) -> bool {
        self._state != State::Closed
    }

    fn close(&mut self) -> Result<(), OSMWriteError> {
        self.ensure_header()?;

        write!(self.writer, "\n</osm>")?;

        self._state = State::Closed;

        Ok(())
    }

    fn write_obj(&mut self, obj: &impl OSMObj) -> Result<(), OSMWriteError> {
        match self._state {
            State::Initial => self.ensure_header()?, // This will update self._state
            State::WritingObjects => {}
            State::Closed => return Err(OSMWriteError::AlreadyClosed),
        }

        write!(
            self.writer,
            "{}",
            match obj.object_type() {
                OSMObjectType::Node => "\n\t<node",
                OSMObjectType::Way => "\n\t<way",
                OSMObjectType::Relation => "\n\t<relation",
            }
        )?;
        write!(self.writer, " id=\"{}\"", obj.id())?;
        write!(
            self.writer,
            " visible=\"{}\"",
            if obj.deleted() { "false" } else { "true" }
        )?;
        write!(self.writer, " version=\"{}\"", obj.version().unwrap())?;
        if let Some(user) = obj.user() {
            write!(self.writer, " user=\"")?;
            write_xml_escaped(&mut self.writer, user)?;
            write!(self.writer, "\"")?;
        }
        if let Some(uid) = obj.uid() {
            write!(self.writer, " uid=\"{}\"", uid)?;
        }
        if let Some(changeset_id) = obj.changeset_id() {
            write!(self.writer, " changeset=\"{}\"", changeset_id)?;
        }
        if let Some(timestamp) = obj.timestamp() {
            write!(self.writer, " timestamp=\"{}\"", timestamp)?;
        }

        if let Some(node) = obj.as_node() {
            if let Some((lat, lon)) = node.lat_lon() {
                write!(self.writer, " lat=\"{}\"", lat)?;
                write!(self.writer, " lon=\"{}\"", lon)?;
            }
        }

        if obj.is_node() && obj.untagged() {
            write!(self.writer, " />")?;
            return Ok(());
        }
        write!(self.writer, ">")?;

        if let Some(way) = obj.as_way() {
            for nid in way.nodes() {
                write!(self.writer, "\n\t\t<nd ref=\"{}\" />", nid)?;
            }
        }

        if let Some(relation) = obj.as_relation() {
            for member in relation.members() {
                write!(
                    self.writer,
                    "\n\t\t<member type=\"{}\" ref=\"{}\" role=\"",
                    member.0, member.1
                )?;
                if !member.2.is_empty() {
                    write_xml_escaped(&mut self.writer, member.2)?;
                }
                write!(self.writer, "\"/>")?;
            }
        }

        for (k, v) in obj.tags() {
            write!(self.writer, "\n\t\t<tag k=\"")?;
            write_xml_escaped(&mut self.writer, k)?;
            write!(self.writer, "\" v=\"")?;
            write_xml_escaped(&mut self.writer, v)?;
            write!(self.writer, "\" />")?;
        }

        write!(
            self.writer,
            "{}",
            match obj.object_type() {
                OSMObjectType::Node => "\n\t</node>",
                OSMObjectType::Way => "\n\t</way>",
                OSMObjectType::Relation => "\n\t</relation>",
            }
        )?;

        Ok(())
    }

    fn into_inner(self) -> W {
        todo!("converting an XMLWriter into_inner");
        //self.writer.into_inner()
    }
}

impl<W: Write> Drop for XMLWriter<W> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lat, Lon};
    use obj_types::StringNodeBuilder;

    macro_rules! assert_escape {
        ( $name:ident, $input:expr, $output:expr ) => {
            #[test]
            fn $name() {
                let s = $input;
                let mut res = Vec::new();
                write_xml_escaped(&mut res, s).unwrap();
                let new_str = std::str::from_utf8(&res).unwrap();
                assert_eq!(new_str, $output);
            }
        };
    }

    assert_escape!(escape1, "foo", "foo");
    assert_escape!(escape2, "&foo", "&amp;foo");
    assert_escape!(escape3, "foo bar", "foo bar");
    assert_escape!(escape4, "foo\nbar", "foo\nbar");
    assert_escape!(escape5, "&&foo", "&amp;&amp;foo");
    assert_escape!(escape6, "&ergio", "&amp;ergio");
    assert_escape!(escape7, "foo & bar", "foo &amp; bar");
    assert_escape!(
        escape8,
        "foo & bar \" ' <whoop>",
        "foo &amp; bar &quot; &apos; &lt;whoop&gt;"
    );

    macro_rules! assert_write_obj {
        ( $name:ident, $input:expr, $output:expr ) => {
            #[test]
            fn $name() {
                let o: StringOSMObj = $input.into();
                let mut res = Vec::new();
                let mut xmlwr = XMLWriter::new(&mut res);
                xmlwr.write_obj(&o).unwrap();
                xmlwr.close().unwrap();
                drop(xmlwr);
                let new_str = std::str::from_utf8(&res).unwrap();
                assert_eq!(new_str, $output);
            }
        };
    }

    assert_write_obj!(node1,
        StringNodeBuilder::default()
			._id(1)
			._version(2)
			._changeset_id(1)
			._timestamp(700.into())
			._uid(1)
			._user("&foo".to_string())
			._lat_lon((Lat(0), Lon(0)))
			.build()
			.unwrap(),
	    format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\">\n\t<node id=\"1\" visible=\"true\" version=\"2\" user=\"&amp;foo\" uid=\"1\" changeset=\"1\" timestamp=\"1970-01-01T00:11:40Z\" lat=\"0\" lon=\"0\" />\n</osm>\n</osm>", crate::version())
	);
}
