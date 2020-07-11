//! XML file format

use std::io::{Read, BufReader, Write};
use super::{OSMReader, OSMWriter, OSMWriteError};
use super::TimestampFormat;
use super::{OSMObj, Node, Way};
use ::obj_types::{StringNode, StringWay, StringRelation, StringOSMObj};
use super::version;
use std::char;
use std::collections::HashMap;
use super::ObjId;
use std::iter::Iterator;

use xml_rs::reader::{EventReader, XmlEvent, Events};
use xml_rs::attribute::OwnedAttribute;
use quick_xml::events::{Event, BytesEnd, BytesStart};

pub struct XMLReader<R: Read>  {
    parser: Events<BufReader<R>>,
}

impl<R: Read> OSMReader for XMLReader<R> {
    type R = R;
    type Obj = StringOSMObj;

    fn new(reader: R) -> XMLReader<R> {
        XMLReader { parser: EventReader::new(BufReader::new(reader)).into_iter() }
    }

    fn into_inner(self) -> R {
        self.parser.into_inner().into_inner().into_inner()
    }

    fn inner(&self) -> &R {
        unimplemented!()
    }

    fn next(&mut self) -> Option<StringOSMObj> {
        let mut elements = Vec::new();

        // Pull xml/sax elements from the xml parser into a vector so we know what to work with.
        let mut should_push = false;
        loop {
            let el = match self.parser.next() {
                None => { break; },
                Some(e) => e,
            };

            let el = el.unwrap();

            let mut should_break = false;
            match el {
                XmlEvent::StartElement{ref name, ..} => {
                    match name.local_name.as_str() {
                        "node"|"way"|"relation" => {
                            should_push = true;
                        }
                        _ => {}
                    }
                },
                XmlEvent::EndElement{ref name, ..} => {
                    match name.local_name.as_str() {
                        "node"|"way"|"relation" => {
                            should_break = true;
                        }
                        _ => {}
                    }
                },
                _ => {},
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
pub struct XMLWriter<W: Write>  {
    writer: quick_xml::Writer<W>,
    _state: State,
}



#[allow(unused_variables)]
fn extract_attrs(x: &mut XmlEvent) -> Option<&mut Vec<OwnedAttribute>> {
    match *x {
        XmlEvent::StartElement{ ref name, ref mut attributes, ref namespace } => {
            Some(attributes)
        },
        _ => None,
    }
}

fn get_xml_attribute<'a>(attrs: &mut Vec<OwnedAttribute>, key: &str) -> Option<String> {
    attrs.iter()
        .position(|attr| attr.name.local_name == key)
        .map(|idx| {
            let attr = attrs.swap_remove(idx);
            let OwnedAttribute{ name: _, value } = attr;
            value
        })
}

fn get_tags(els: &mut Vec<XmlEvent>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement{ ref name, ref mut attributes, ..} = el {
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

fn get_nodes(els: &mut Vec<XmlEvent>) -> Vec<ObjId> {
    let mut result = Vec::new();

    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement{ ref name, ref mut attributes, ..} = el {
            if name.local_name == "nd" {
                let ref_id_o: Option<ObjId> = get_xml_attribute(attributes, "ref")
                    .and_then(|e| e.parse().ok());
                if let Some(ref_id) = ref_id_o {
                    result.push(ref_id);
                }
            }
            
        }
    }

    result
}

fn get_members(els: &mut Vec<XmlEvent>) -> Vec<(char, ObjId, String)> {
    let mut result = Vec::new();

    for el in els.iter_mut() {
        if let &mut XmlEvent::StartElement{ ref name, ref mut attributes, ..} = el {
            if name.local_name == "member" {
                let ref_id_o: Option<ObjId> = get_xml_attribute(attributes, "ref").and_then(|e| e.parse().ok());
                let member_type_o: Option<char> = get_xml_attribute(attributes, "type")
                    .and_then(|t| match t.as_ref() {
                        "node" => Some('n'),
                        "way" => Some('w'),
                        "relation" => Some('r'),
                        _ => { None },
                    });
                let role = get_xml_attribute(attributes, "role").unwrap_or_else(|| String::new());
                if let (Some(ref_id), Some(member_type)) = (ref_id_o, member_type_o) {
                    result.push((member_type, ref_id, role));
                }
            }
            
        }
    }

    result

}




pub(crate) fn xml_elements_to_osm_obj(els: &mut Vec<XmlEvent>) -> Option<StringOSMObj> {
    match els.first() {
        Some(&XmlEvent::StartElement{ ref name, .. }) => {
            match name.local_name.as_str() {
                "node" => { node_xml_elements_to_osm_obj(els) },
                "way" => { way_xml_elements_to_osm_obj(els) },
                "relation" => { relation_xml_elements_to_osm_obj(els) },
                _ => None,
            }
        },
        _ => { None }
    }
}

fn node_xml_elements_to_osm_obj(els: &mut Vec<XmlEvent>) -> Option<StringOSMObj> {
    let mut attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(&mut attrs, "id").and_then(|x| x.parse().ok())?;
    let version  = get_xml_attribute(&mut attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(&mut attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(&mut attrs, "timestamp").map(|x| TimestampFormat::ISOString(x.to_owned()));
    let uid = get_xml_attribute(&mut attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(&mut attrs, "user");
    let lat = get_xml_attribute(&mut attrs, "lat")?.parse().ok();
    let lon = get_xml_attribute(&mut attrs, "lon")?.parse().ok();

    let lat_lon = match (lat, lon) {
        (Some(lat), Some(lon)) => Some((lat, lon)),
        _ => None,
    };
    let deleted = get_xml_attribute(&mut attrs, "visible").and_then(|val| match val.as_str() { "true" => Some(false), "false" => Some(true), _ => None }).unwrap_or(false);

    let tags = get_tags(els);

    Some(StringOSMObj::Node(StringNode{ _id: id, _version: version, _deleted: deleted, _changeset_id: changeset_id, _timestamp: timestamp, _uid: uid, _user: user, _lat_lon: lat_lon, _tags: tags }))
}

fn way_xml_elements_to_osm_obj(els: &mut Vec<XmlEvent>) -> Option<StringOSMObj> {
    let mut attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(&mut attrs, "id").and_then(|x| x.parse().ok())?;
    let version  = get_xml_attribute(&mut attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(&mut attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(&mut attrs, "timestamp").map(|x| TimestampFormat::ISOString(x.to_owned()));
    let uid = get_xml_attribute(&mut attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(&mut attrs, "user");
    let deleted = get_xml_attribute(&mut attrs, "visible").and_then(|val| match val.as_str() { "true" => Some(false), "false" => Some(true), _ => None }).unwrap_or(false);

    let tags = get_tags(els);
    let nodes = get_nodes(els);
    Some(StringOSMObj::Way(StringWay{ _id: id, _version: version, _deleted: deleted, _changeset_id: changeset_id, _timestamp: timestamp, _uid: uid, _user: user, _tags: tags, _nodes: nodes }))
}

fn relation_xml_elements_to_osm_obj(els: &mut Vec<XmlEvent>) -> Option<StringOSMObj> {
    let mut attrs = extract_attrs(els.first_mut()?)?;
    let id: ObjId = get_xml_attribute(&mut attrs, "id").and_then(|x| x.parse().ok())?;
    let version  = get_xml_attribute(&mut attrs, "version").and_then(|x| x.parse().ok());
    let changeset_id = get_xml_attribute(&mut attrs, "changeset").and_then(|x| x.parse().ok());
    let timestamp = get_xml_attribute(&mut attrs, "timestamp").map(|x| TimestampFormat::ISOString(x.to_owned()));
    let uid = get_xml_attribute(&mut attrs, "uid").and_then(|x| x.parse().ok());
    let user = get_xml_attribute(&mut attrs, "user");
    let deleted = get_xml_attribute(&mut attrs, "visible").and_then(|val| match val.as_str() { "true" => Some(false), "false" => Some(true), _ => None }).unwrap_or(false);

    let tags = get_tags(els);
    let members = get_members(els);
    Some(StringOSMObj::Relation(StringRelation{ _id: id, _version: version, _deleted: deleted, _changeset_id: changeset_id, _timestamp: timestamp, _uid: uid, _user: user, _tags: tags, _members: members }))
}



impl From<quick_xml::Error> for OSMWriteError {
    fn from(err: quick_xml::Error) -> OSMWriteError { OSMWriteError::XMLWrite(err) }
}

impl<W: Write> XMLWriter<W> {
    fn ensure_header(&mut self) -> Result<(), OSMWriteError> {
        if self._state == State::Initial {
            let mut elem = BytesStart::borrowed_name(b"osm");
            elem.push_attribute(("version", "0.6"));

            elem.push_attribute(("generator", format!("osmio/{}", version()).as_str()));

            self.writer.write_event(Event::Start(elem)).unwrap(); // fixme
            self._state = State::WritingObjects;
        }
        Ok(())
    }
}

impl<W: Write> OSMWriter<W> for XMLWriter<W> {
    fn new(writer: W) -> Self {
        // TODO have a config that does indentation and stuff 
        XMLWriter {
            writer: quick_xml::Writer::new_with_indent(writer, '\t' as u8, 1),
            _state: State::Initial,
        }
    }

    fn is_open(&self) -> bool {
        self._state != State::Closed
    }

    fn close(&mut self) -> Result<(), OSMWriteError> {
        self.ensure_header()?;

        self.writer.write_event(Event::End(BytesEnd::borrowed(b"osm")))?;

        self._state = State::Closed;

        Ok(())
    }

    fn write_obj(&mut self, obj: &impl OSMObj) -> Result<(), OSMWriteError> {

        match self._state {
            State::Initial => self.ensure_header()?,    // This will update self._state
            State::WritingObjects => {},
            State::Closed => return Err(OSMWriteError::AlreadyClosed),
        }

        let tag_name = format!("{}", obj.object_type());
        let mut xml_el = BytesStart::borrowed_name(tag_name.as_bytes());
        xml_el.push_attribute(("id", obj.id().to_string().as_ref()));
        xml_el.push_attribute(("visible", if obj.deleted() { "false" } else { "true" }));
        xml_el.push_attribute(("version", obj.version().unwrap().to_string().as_ref()));
        xml_el.push_attribute(("user", obj.user().unwrap().to_string().as_ref()));
        xml_el.push_attribute(("uid", obj.uid().unwrap().to_string().as_ref()));
        xml_el.push_attribute(("changeset", obj.changeset_id().unwrap().to_string().as_ref()));
        xml_el.push_attribute(("timestamp", obj.timestamp().as_ref().unwrap().to_string().as_ref()));

        if let Some(node) = obj.as_node() {
            xml_el.push_attribute(("lat", node.lat_lon().unwrap().0.to_string().as_str()));
            xml_el.push_attribute(("lon", node.lat_lon().unwrap().1.to_string().as_str()));

        }

        self.writer.write_event(Event::Start(xml_el))?;

        let mut nd_el;
        if let Some(way) = obj.as_way() {
            for nid in way.nodes() {
                nd_el = BytesStart::borrowed_name(b"nd");
                nd_el.push_attribute(("ref", nid.to_string().as_str()));
                self.writer.write_event(Event::Empty(nd_el))?;
            }
        }

        if let Some(_relation) = obj.as_relation() {
            unimplemented!();
        }

        let mut tag_el;
        for (k, v) in obj.tags() {
            tag_el = BytesStart::borrowed_name(b"tag");
            tag_el.push_attribute(("k", k));
            tag_el.push_attribute(("v", v));
            self.writer.write_event(Event::Empty(tag_el))?;
        }
        self.writer.write_event(Event::End(BytesEnd::borrowed(tag_name.as_bytes())))?;

        Ok(())
    }

    fn into_inner(self) -> W {
        unimplemented!();
        //self.writer.into_inner()
    }
}

impl<W: Write> Drop for XMLWriter<W> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
