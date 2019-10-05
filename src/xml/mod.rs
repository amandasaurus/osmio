//! XML file format

use std::io::{Read, BufReader, Write};
use super::{OSMReader, OSMWriter, OSMWriteError};
use super::TimestampFormat;
use super::OSMObj;
use ::obj_types::{StringNode, StringWay, StringRelation, StringOSMObj};
use super::version;
use std::char;
use std::collections::HashMap;
use super::ObjId;
use std::iter::Iterator;

use xml_rs::reader::{EventReader, XmlEvent, Events};
use xml_rs::writer::{EventWriter, EmitterConfig, Error};
use xml_rs::attribute::OwnedAttribute;

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
    writer: EventWriter<W>,
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



impl From<Error> for OSMWriteError {
    fn from(err: Error) -> OSMWriteError { OSMWriteError::XMLWrite(err) }
}

impl<W: Write> XMLWriter<W> {
    fn ensure_header(&mut self) -> Result<(), OSMWriteError> {
        use xml_rs::writer::XmlEvent;
        if self._state == State::Initial {
            self.writer.write(XmlEvent::start_element("osm").attr("version", "0.6").attr("generator", &format!("osmio/{}", version())))?;
            self._state = State::WritingObjects;
        }
        Ok(())
    }
}

impl<W: Write> OSMWriter<W> for XMLWriter<W> {
    fn new(writer: W) -> Self {
        // TODO have a config that does indentation and stuff 
        XMLWriter { writer: EventWriter::new_with_config(writer, EmitterConfig::new().perform_indent(true)), _state: State::Initial }
    }

    fn is_open(&self) -> bool {
        self._state != State::Closed
    }

    fn close(&mut self) {
        self.ensure_header().unwrap();

        use xml_rs::writer::XmlEvent;
        // close the osm element

        // FIXME shouldn't this be a result?
        self.writer.write(XmlEvent::end_element()).unwrap();

        self._state = State::Closed;
    }

    fn write_obj(&mut self, _obj: &impl OSMObj) -> Result<(), OSMWriteError> {
        //use xml_rs::writer::XmlEvent;

        match self._state {
            State::Initial => self.ensure_header()?,    // This will update self._state
            State::WritingObjects => {},
            State::Closed => return Err(OSMWriteError::AlreadyClosed),
        }

        unimplemented!();
        //match obj {
        //    &OSMObj::Node(ref n) => {
        //        let id = n.id.to_string();
        //        let lat = n.lat.map(|e| e.to_string()).unwrap_or("".to_string());
        //        let lon = n.lon.map(|e| e.to_string()).unwrap_or("".to_string());
        //        self.writer.write(XmlEvent::start_element("node")
        //            .attr("id", &id)
        //            .attr("lat", &lat)
        //            .attr("lon", &lon)
        //            .attr("visible", if n.deleted { "false" } else { "true" })
        //            .attr("version", &n.version.unwrap().to_string())
        //            .attr("user", &n.user.unwrap().to_string())
        //            .attr("uid", &n.uid.unwrap().to_string())
        //            .attr("changeset", &n.changeset_id.unwrap().to_string())
        //            .attr("timestamp", &n.timestamp.unwrap().to_string())
        //        )?;

        //        for (k, v) in n.tags.iter() {
        //            self.writer.write(XmlEvent::start_element("tag").attr("k", k).attr("v", v))?;
        //            self.writer.write(XmlEvent::end_element())?;
        //        }
        //        self.writer.write(XmlEvent::end_element())?;
        //    },
        //    &OSMObj::Way(ref w) => {
        //        self.writer.write(XmlEvent::start_element("way")
        //            .attr("id", &w.id.to_string())
        //            .attr("visible", if w.deleted { "false" } else { "true" })
        //            .attr("version", &w.version.unwrap().to_string())
        //            .attr("user", &w.user.unwrap().to_string())
        //            .attr("uid", &w.uid.unwrap().to_string())
        //            .attr("changeset", &w.changeset_id.unwrap().to_string())
        //            .attr("timestamp", &w.timestamp.unwrap().to_string())
        //        )?;

        //        for nid in w.nodes.iter() {
        //            self.writer.write(XmlEvent::start_element("nd").attr("ref", &nid.to_string()))?;
        //            self.writer.write(XmlEvent::end_element())?;
        //        }
        //        for (k, v) in w.tags.iter() {
        //            try!(self.writer.write(XmlEvent::start_element("tag").attr("k", k).attr("v", v)));
        //            try!(self.writer.write(XmlEvent::end_element()));
        //        }

        //        try!(self.writer.write(XmlEvent::end_element()));
        //    },
        //    &OSMObj::Relation(ref r) => {
        //        self.writer.write(XmlEvent::start_element("relation")
        //            .attr("id", &r.id.to_string())
        //            .attr("visible", if r.deleted { "false" } else { "true" })
        //            .attr("version", &r.version.unwrap().to_string())
        //            .attr("user", &r.user.unwrap().to_string())
        //            .attr("uid", &r.uid.unwrap().to_string())
        //            .attr("changeset", &r.changeset_id.unwrap().to_string())
        //            .attr("timestamp", &r.timestamp.unwrap().to_string())
        //        )?;

        //        for &(typechar, id, ref role) in r.members.iter() {
        //            try!(self.writer.write(XmlEvent::start_element("member")
        //                              .attr("type", &(match typechar { 'n' => "node", 'w' => "way", 'r' => "relation", _ => ""}))
        //                              .attr("ref", &id.to_string())
        //                              .attr("role", &role)));
        //            try!(self.writer.write(XmlEvent::end_element()));
        //        }

        //        for (k, v) in r.tags.iter() {
        //            try!(self.writer.write(XmlEvent::start_element("tag").attr("k", k).attr("v", v)));
        //            try!(self.writer.write(XmlEvent::end_element()));
        //        }

        //        self.writer.write(XmlEvent::end_element())?;
        //    },
        //}
        //Ok(())
    }

    fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}
