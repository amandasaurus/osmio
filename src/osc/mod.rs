//! OSC File format

use std::io::{Read, BufReader, Write};
use super::{OSMReader, OSMWriter, OSMWriteError};
use super::{OSMObj, Node, Way};
use ::obj_types::StringOSMObj;
use std::iter::Iterator;
use super::version;

use ::xml::xml_elements_to_osm_obj;

use xml_rs::reader::{EventReader, XmlEvent, Events};
use quick_xml::events::{Event, BytesEnd, BytesStart};

pub struct OSCReader<R: Read>  {
    parser: Events<BufReader<R>>,
}

#[derive(PartialEq)]
enum State {
    Initial,
    WritingObjects,
    Closed,
}

pub struct OSCWriter<W: Write>  {
    writer: quick_xml::Writer<W>,
    _state: State,
}


impl<R: Read> OSMReader for OSCReader<R> {
    type R = R;
    type Obj = StringOSMObj;

    fn new(reader: R) -> Self {
        OSCReader { parser: EventReader::new(BufReader::new(reader)).into_iter() }
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

impl<W: Write> OSCWriter<W> {
    fn ensure_header(&mut self) -> Result<(), OSMWriteError> {
        if self._state == State::Initial {
            let mut elem = BytesStart::borrowed_name(b"osmChange");
            elem.push_attribute(("version", "0.6"));

            elem.push_attribute(("generator", format!("osmio/{}", version()).as_str()));

            self.writer.write_event(Event::Start(elem)).unwrap(); // fixme
            self.writer.write_event(Event::Start(BytesStart::borrowed_name(b"modify")))?;
            self._state = State::WritingObjects;
        }
        Ok(())
    }
}



impl<W: Write> OSMWriter<W> for OSCWriter<W> {
    fn new(writer: W) -> Self {
        // TODO have a config that does indentation and stuff 
        OSCWriter {
            writer: quick_xml::Writer::new_with_indent(writer, '\t' as u8, 1),
            _state: State::Initial,
        }
    }

    fn is_open(&self) -> bool {
        self._state != State::Closed
    }

    fn close(&mut self) -> Result<(), OSMWriteError> {
        self.ensure_header()?;

        if self._state != State::Closed {
            self.writer.write_event(Event::End(BytesEnd::borrowed(b"modify")))?;
            self.writer.write_event(Event::End(BytesEnd::borrowed(b"osmChange")))?;
            self._state = State::Closed;
        }

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

impl<W: Write> Drop for OSCWriter<W> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
