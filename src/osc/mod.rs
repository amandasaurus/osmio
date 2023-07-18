//! OSC File format

use super::version;
use super::{Node, OSMObj, OSMObjectType, Relation, Way};
use super::{OSMReader, OSMWriteError, OSMWriter};
use obj_types::StringOSMObj;
use std::io::{BufReader, Read, Write};
use std::iter::Iterator;

use xml::{write_xml_escaped, xml_elements_to_osm_obj};

use xml_rs::reader::{EventReader, Events, XmlEvent};

pub struct OSCReader<R: Read> {
    parser: Events<BufReader<R>>,
}

#[derive(PartialEq)]
enum State {
    Initial,
    WritingObjects,
    Closed,
}

pub struct OSCWriter<W: Write> {
    writer: W,
    //headers: HashMap<String, String>,
    _state: State,
}

impl<R: Read> OSMReader for OSCReader<R> {
    type R = R;
    type Obj = StringOSMObj;

    fn new(reader: R) -> Self {
        OSCReader {
            parser: EventReader::new(BufReader::new(reader)).into_iter(),
        }
    }

    fn into_inner(self) -> R {
        self.parser.into_inner().into_inner().into_inner()
    }

    fn inner(&self) -> &R {
        todo!("{} {} OSCReader inner()", file!(), line!());
    }

    fn next(&mut self) -> Option<StringOSMObj> {
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
                    "node" | "way" | "relation" => {
                        should_push = true;
                    }
                    _ => {}
                },
                XmlEvent::EndElement { ref name, .. } => match name.local_name.as_str() {
                    "node" | "way" | "relation" => {
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

impl<W: Write> OSCWriter<W> {
    fn ensure_header(&mut self) -> Result<(), OSMWriteError> {
        if self._state == State::Initial {
            writeln!(self.writer, "<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
            write!(
                self.writer,
                "<osmChange version=\"0.6\" generator=\"osmio/{}\"",
                version()
            )?;

            //for (k, v) in self.headers.iter() {
            //    write!(self.writer, " {}=\"", k)?;
            //    write_xml_escaped(&mut self.writer, v)?;
            //    write!(self.writer, "\"")?;
            //}
            write!(self.writer, ">")?;
            write!(self.writer, "\n<modify>")?;

            self._state = State::WritingObjects;
        }
        Ok(())
    }
}

impl<W: Write> OSMWriter<W> for OSCWriter<W> {
    fn new(writer: W) -> Self {
        OSCWriter {
            writer,
            //headers: HashMap::new(),
            _state: State::Initial,
        }
    }

    fn is_open(&self) -> bool {
        self._state != State::Closed
    }

    fn close(&mut self) -> Result<(), OSMWriteError> {
        self.ensure_header()?;

        if self._state != State::Closed {
            write!(self.writer, "\n</modify>")?;
            write!(self.writer, "\n</osmChange>")?;

            self._state = State::Closed;
        }

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
        todo!("{} {}  OSCWriter into_inner", file!(), line!());
        //self.writer.into_inner()
    }
}

impl<W: Write> Drop for OSCWriter<W> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
