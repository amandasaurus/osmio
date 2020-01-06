use std::io::{Read, BufReader};
use super::OSMReader;
use ::obj_types::StringOSMObj;
use std::iter::Iterator;

use ::xml::xml_elements_to_osm_obj;

use xml_rs::reader::{EventReader, XmlEvent, Events};

pub struct OSCReader<R: Read>  {
    parser: Events<BufReader<R>>,
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


