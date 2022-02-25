//! OPL (Object Per Line) file format
//! See http://osmcode.org/opl-file-format/
use super::{OSMReader, OSMWriter};
use super::{OSMWriteError};
use super::OSMObj;
use super::TimestampFormat;
use std::collections::HashMap;
use super::{ObjId, Lat, Lon, Node, Way, Relation};
use std::iter::Iterator;
use std::io::{Read, BufReader, Write, BufRead};
use std::rc::Rc;

pub struct OPLReader<R: Read>  {
    buff_reader: BufReader<R>,
}

impl<R: Read> OSMReader for OPLReader<R> {
    type R = R;

    fn new(reader: R) -> OPLReader<R> {
        OPLReader { buff_reader: BufReader::new(reader) }
    }

    fn into_inner(self) -> R {
        self.buff_reader.into_inner()
    }

    fn next(&mut self) -> Option<OSMObj> {
        let mut line = String::new();
        let res = self.buff_reader.read_line(&mut line);

        if res.is_err() {
            None
        } else {
            decode_line(line.trim()).ok()
        }
    }
}

pub struct OPLWriter<W: Write>  {
    writer: W,
    is_open: bool,
}

impl From<::std::io::Error> for OSMWriteError {
    fn from(err: ::std::io::Error) -> OSMWriteError { OSMWriteError::OPLWrite(err) }
}

impl<W: Write> OSMWriter<W> for OPLWriter<W> {
    fn new(writer: W) -> OPLWriter<W> {
        OPLWriter { writer: writer, is_open: true }
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        // Do nothing
        self.is_open = false;
    }

    fn write_obj(&mut self, _obj: &OSMObj) -> Result<(), OSMWriteError> {
        unimplemented!();
        //match obj {
        //    OSMObj::Node(n) => {
        //        write!(self.writer, "n{} v{} d{} c{} t{} i{} u{} T{} x{} y{}\n", n.id, n.version.unwrap(), if n.deleted { 'D' } else { 'V' }, n.changeset_id.unwrap(), n.timestamp.unwrap(), n.uid.unwrap(), n.user.unwrap(), encode_tags(&n.tags), n.lon.map(|x| { format!("{}", x) }).unwrap_or("".to_string()), n.lat.map(|x| { format!("{}", x) }).unwrap_or("".to_string()))?;
        //    },
        //   OSMObj::Way(w) => {
        //        write!(self.writer, "w{} v{} d{} c{} t{} i{} u{} T{} N{}\n", w.id, w.version.unwrap(), if w.deleted { 'D' } else { 'V' }, w.changeset_id.unwrap(), w.timestamp.unwrap(), w.uid.unwrap(), w.user.unwrap(), encode_tags(&w.tags), encode_way_nodes(&w.nodes))?;
        //    },
        //    OSMObj::Relation(r) => {
        //        write!(self.writer, "r{} v{} d{} c{} t{} i{} u{} T{} M{}\n", r.id, r.version.unwrap(), if r.deleted { 'D' } else { 'V' }, r.changeset_id.unwrap(), r.timestamp.unwrap().to_iso_string(), r.uid.unwrap(), r.user.unwrap(), encode_tags(&r.tags), encode_members(&r.members))?;
        //    },
        //}
        //Ok(())
    }

    fn into_inner(self) -> W {
        self.writer
    }
}

#[derive(Debug)]
pub struct DecodeStringError;

fn decode_string(s: &str) -> Result<String, DecodeStringError> {
    let mut buffer: Vec<char> = Vec::new();
    let mut output: Vec<char> = Vec::new();
    let mut looking_for_percent = false;
    for c in s.chars() {
        if looking_for_percent {
            if c == '%' {
                looking_for_percent = false;
                let hex_string: String = buffer.into_iter().collect();
                let codepoint: u32 = u32::from_str_radix(hex_string.as_str(), 16).or(Err(DecodeStringError))?;
                let new_char: char = ::std::char::from_u32(codepoint).ok_or(DecodeStringError)?;
                output.push(new_char);
                buffer = Vec::new();
            } else {
                buffer.push(c);
            }
        } else {
            if c == '%' {
                looking_for_percent = true;
                buffer.clear();
            } else {
                output.push(c);
            }
        }
    }
    
    Ok(output.into_iter().collect())
}

fn encode_string(s: &str) -> String {
    let mut result: String = s.to_string();
    for c in vec![ ' ', '\n', ',', '=', '@' ] {
        result = result.replace(format!("{}", c).as_str(), format!("%{:X}%", (c as u32)).as_str());
    }
    result
}


fn decode_tags(line: &str) -> Result<HashMap<Rc<String>, Rc<String>>, DecodeStringError> {
    if line.len() == 0 {
        return Ok(HashMap::new());
    }

    let mut result: HashMap<Rc<String>, Rc<String>> = HashMap::new();
    for kv in line.split(",") {
        let kv: Vec<_> = kv.splitn(2, "=").collect();
        let k = Rc::new(decode_string(kv[0])?);
        let v = Rc::new(decode_string(kv[1])?);
        result.insert(k, v);
    }
    Ok(result)
}

fn encode_tags(tags: &HashMap<Rc<String>, Rc<String>>) -> String {
    tags.iter().map(|(k, v)| { format!("{}={}", encode_string(k), encode_string(v)) }).collect::<Vec<String>>().join(",")

}


fn decode_way_nodes(line: &str) -> Result<Vec<ObjId>, DecodeStringError> {
    let mut result: Vec<ObjId> = Vec::new();
    for x in line.split(",").map(|x| { x.chars().skip(1).collect::<String>() }) {
        let node_id = x.parse::<ObjId>().or(Err(DecodeStringError))?;
        result.push(node_id);
    }
    Ok(result)
}

fn encode_way_nodes(nodes: &Vec<ObjId>) -> String {
    nodes.iter().map(|&n| { format!("n{}", n) }).collect::<Vec<String>>().join(",")
}

fn decode_members(line: &str) -> Result<Vec<(char, ObjId, Rc<String>)>, DecodeStringError> {
    let mut result = Vec::new();
    for x in line.split(",") {
        let (obj_type, rest) = split_key_value(x)?;
        let obj_type = obj_type.chars().next().ok_or(DecodeStringError)?;
        let rest: Vec<_> = rest.splitn(2, "@").collect();
        let (id, role) = (rest[0], rest[1]);
        let id: ObjId = id.parse().or(Err(DecodeStringError))?;
        result.push((obj_type, id, Rc::new(role.to_string())));
    }
    Ok(result)
}

fn encode_members(members: &Vec<(char, ObjId, Rc<String>)>) -> String {
    members.iter().map(|&(t, id, ref role)| { format!("{}{}@{}", t, id, role) }).collect::<Vec<String>>().join(",")
}

fn split_key_value(s: &str) -> Result<(&str, &str), DecodeStringError> {
    if s.len() == 1 {
        // e.g. empty tags
       Ok((s, ""))
    } else {
        // Check if the 2nd (ie index 1) character actually starts at byte 1. This fails when the
        // first character is a multibyte character (which has happened with the real history file)
        match s.char_indices().nth(1) {
            Some((1, _)) => {
                Ok(s.split_at(1))
            }
            _ => {
                // TODO This is invalid input, so it should be logged
                Err(DecodeStringError)
            }
        }
    }
}


pub fn decode_line(line: &str) -> Result<OSMObj, DecodeStringError> {
    let intermediate_items: Vec<_> = line.split(" ").map(split_key_value).collect();
    let mut items = Vec::with_capacity(intermediate_items.len());
    for i in intermediate_items {
        match i {
            Err(DecodeStringError) => { return Err(DecodeStringError); },
            Ok(x) => { items.push(x); }
        }
    }
    
    match items[0].0 {
        "n" => {
            let tags = decode_tags(items[7].1)?;
            let lon = if items[8].1.len() == 0 { None } else { Some(try!(items[8].1.parse::<Lon>().or(Err(DecodeStringError)))) };
            let lat = if items[9].1.len() == 0 { None } else { Some(try!(items[9].1.parse::<Lat>().or(Err(DecodeStringError)))) };

            let node = Node {
                id: try!(items[0].1.parse::<ObjId>().or(Err(DecodeStringError))),
                version: Some(items[1].1.parse::<u32>().or(Err(DecodeStringError))?),
                deleted: items[2].1 == "D",
                changeset_id: Some(items[3].1.parse::<u32>().or(Err(DecodeStringError))?),
                timestamp: Some(TimestampFormat::ISOString(items[4].1.to_string())),
                uid: Some(items[5].1.parse::<u32>().or(Err(DecodeStringError))?),
                user: Some(Rc::new(items[6].1.to_string())),
                tags: tags,
                lon: lon,
                lat: lat,
            };
            Ok(OSMObj::Node(node))
        },
        "w" => {
            let tags = decode_tags(items[7].1)?;
            let nodes = decode_way_nodes(items[8].1)?;
            let way = Way {
                id: items[0].1.parse::<ObjId>().or(Err(DecodeStringError))?;
                version: Some(items[1].1.parse::<u32>().or(Err(DecodeStringError))?),
                deleted: items[2].1 == "D",
                changeset_id: Some(items[3].1.parse::<u32>().or(Err(DecodeStringError))?),
                timestamp: Some(TimestampFormat::ISOString(items[4].1.to_string())),
                uid: Some(items[5].1.parse::<u32>().or(Err(DecodeStringError))?),
                user: Some(Rc::new(items[6].1.to_string())),
                tags: tags,
                nodes: nodes,
            };
            Ok(OSMObj::Way(way))
        },
        "r" => {
            let tags = try!(decode_tags(items[7].1));
            let members = try!(decode_members(items[8].1));
            let relation = Relation {
                id: try!(items[0].1.parse::<ObjId>().or(Err(DecodeStringError))),
                version: Some(items[1].1.parse::<u32>().or(Err(DecodeStringError))?),
                deleted: items[2].1 == "D",
                changeset_id: Some(items[3].1.parse::<u32>().or(Err(DecodeStringError))?),
                timestamp: Some(TimestampFormat::ISOString(items[4].1.to_string())),
                uid: Some(items[5].1.parse::<u32>().or(Err(DecodeStringError))?),
                user: Some(Rc::new(items[6].1.to_string())),
                tags: tags,
                members: members,
            };
            Ok(OSMObj::Relation(relation))
        }
        _ => Err(DecodeStringError)
    }
}


//pub fn read<R: BufRead>(reader: &mut R) -> std::iter::Map<std::io::Lines<&mut R>> {
//    reader.lines().map(|line| { decode_line(line.unwrap().as_str()).ok() })
//}
//
