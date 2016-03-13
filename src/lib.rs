use std::collections::HashMap;
use std::io::Read;
use std::iter::Iterator;

pub type ObjId = u64;
pub type Lat = f32;
pub type Lon = f32;

pub struct Node {
    pub id: ObjId,
    version: u32,
    deleted: bool,
    changeset_id: u32,
    timestamp: String,
    uid: u32,
    user: String,
    tags: HashMap<String, String>,
    pub lat: Option<Lat>,
    pub lon: Option<Lon>,
}

pub struct Way {
    pub id: ObjId,
    version: u32,
    deleted: bool,
    changeset_id: u32,
    timestamp: String,
    uid: u32,
    user: String,
    tags: HashMap<String, String>,
    pub nodes: Vec<ObjId>,
}

pub struct Relation {
    pub id: ObjId,
    version: u32,
    deleted: bool,
    changeset_id: u32,
    timestamp: String,
    uid: u32,
    user: String,
    tags: HashMap<String, String>,
    pub members: Vec<(char, ObjId, String)>,
}


pub enum OSMObj {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

pub trait OSMReader<R> {
    fn new(R) -> Self;
    //fn objects(&self) -> Box<Iterator<Item=OSMObj>>;
}

pub struct XMLReader<R: Read>  {
    inner_reader: R,
}


mod opl {
    use std::io::Read;
    use super::OSMReader;
    use super::OSMObj;
    use std::char;
    use std::collections::HashMap;
    use super::{ObjId, Lat, Lon, Node, Way, Relation};
    use std::iter::Iterator;

    pub struct OPLReader<R: Read>  {
        inner_reader: R,
    }

    impl<R: Read> OSMReader<R> for OPLReader<R> {
        fn new(reader: R) -> OPLReader<R> {
            OPLReader { inner_reader: reader }
        }
        //fn objects(&self) -> Box<Iterator<Item=OSMObj>> {
        //    Box::new(self)
        //}
    }

    impl<R: Read> Iterator for OPLReader<R> {
        type Item = OSMObj;
        fn next(&mut self) -> Option<Self::Item> {
            None
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
                    let codepoint: u32 = try!(u32::from_str_radix(hex_string.as_str(), 16).or(Err(DecodeStringError)));
                    let new_char: char = try!(::std::char::from_u32(codepoint).ok_or(DecodeStringError));
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


    fn decode_tags(line: &str) -> Result<HashMap<String, String>, DecodeStringError> {
        if line.len() == 0 {
            return Ok(HashMap::new());
        }

        let mut result: HashMap<String, String> = HashMap::new();
        for kv in line.split(",") {
            let kv: Vec<_> = kv.splitn(2, "=").collect();
            let k = try!(decode_string(kv[0]));
            let v = try!(decode_string(kv[1]));
            result.insert(k, v);
        }
        Ok(result)
    }

    fn encode_tags(tags: &HashMap<String, String>) -> String {
        tags.iter().map(|(k, v)| { format!("{}={}", encode_string(k), encode_string(v)) }).collect::<Vec<String>>().join(",")

    }


    fn decode_way_nodes(line: &str) -> Result<Vec<ObjId>, DecodeStringError> {
        let mut result: Vec<ObjId> = Vec::new();
        for x in line.split(",").map(|x| { x.chars().skip(1).collect::<String>() }) {
            let node_id = try!(x.parse::<ObjId>().or(Err(DecodeStringError)));
            result.push(node_id);
        }
        Ok(result)
    }

    fn encode_way_nodes(nodes: &Vec<ObjId>) -> String {
        nodes.iter().map(|&n| { format!("n{}", n) }).collect::<Vec<String>>().join(",")
    }

    fn decode_members(line: &str) -> Result<Vec<(char, ObjId, String)>, DecodeStringError> {
        let mut result = Vec::new();
        for x in line.split(",") {
            let (obj_type, rest) = try!(split_key_value(x));
            let obj_type = try!(obj_type.chars().next().ok_or(DecodeStringError));
            let rest: Vec<_> = rest.splitn(2, "@").collect();
            let (id, role) = (rest[0], rest[1]);
            let id: ObjId = try!(id.parse().or(Err(DecodeStringError)));
            result.push((obj_type, id, role.to_string()));
        }
        Ok(result)
    }

    fn encode_members(members: &Vec<(char, ObjId, String)>) -> String {
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
                let tags = try!(decode_tags(items[7].1));
                let lon = if items[8].1.len() == 0 { None } else { Some(try!(items[8].1.parse::<Lat>().or(Err(DecodeStringError)))) };
                let lat = if items[9].1.len() == 0 { None } else { Some(try!(items[9].1.parse::<Lat>().or(Err(DecodeStringError)))) };

                let node = Node {
                    id: try!(items[0].1.parse::<ObjId>().or(Err(DecodeStringError))),
                    version: try!(items[1].1.parse::<u32>().or(Err(DecodeStringError))),
                    deleted: items[2].1 == "D",
                    changeset_id: try!(items[3].1.parse::<u32>().or(Err(DecodeStringError))),
                    timestamp: items[4].1.to_string(),
                    uid: try!(items[5].1.parse::<u32>().or(Err(DecodeStringError))),
                    user: items[6].1.to_string(),
                    tags: tags,
                    lon: lon,
                    lat: lat,
                };
                Ok(OSMObj::Node(node))
            },
            "w" => {
                let tags = try!(decode_tags(items[7].1));
                let nodes = try!(decode_way_nodes(items[8].1));
                let way = Way {
                    id: try!(items[0].1.parse::<ObjId>().or(Err(DecodeStringError))),
                    version: try!(items[1].1.parse::<u32>().or(Err(DecodeStringError))),
                    deleted: items[2].1 == "D",
                    changeset_id: try!(items[3].1.parse::<u32>().or(Err(DecodeStringError))),
                    timestamp: items[4].1.to_string(),
                    uid: try!(items[5].1.parse::<u32>().or(Err(DecodeStringError))),
                    user: items[6].1.to_string(),
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
                    version: try!(items[1].1.parse::<u32>().or(Err(DecodeStringError))),
                    deleted: items[2].1 == "D",
                    changeset_id: try!(items[3].1.parse::<u32>().or(Err(DecodeStringError))),
                    timestamp: items[4].1.to_string(),
                    uid: try!(items[5].1.parse::<u32>().or(Err(DecodeStringError))),
                    user: items[6].1.to_string(),
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
    
    mod test {

        #[test]
        fn decode_string1() {
            use super::decode_string;

            assert_eq!(decode_string("hello").unwrap_or("".to_string()), "hello");
            assert_eq!(decode_string("hello%20%world").unwrap_or("".to_string()), "hello world");
        }

        #[test]
        fn encode_string1() {
            use super::encode_string;

            assert_eq!(encode_string("hello"), "hello");
            assert_eq!(encode_string("hello world"), "hello%20%world");
        }

        #[test]
        fn decode_line_node1() {
            use super::decode_line;
            use super::super::OSMObj;

            let line = "n197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM x14.2741628 y36.0292900";
            match decode_line(line).unwrap() {
                OSMObj::Node(n) => { 
                    assert_eq!(n.id, 197801);
                    assert_eq!(n.version, 6);
                    assert_eq!(n.deleted, false);
                    assert_eq!(n.changeset_id, 10009832);
                    assert_eq!(n.timestamp, "2011-12-01T17:03:42Z");
                    assert_eq!(n.uid, 20673);
                    assert_eq!(n.user, "mikefalzon");
                    assert_eq!(n.lon, Some(14.2741628));
                    assert_eq!(n.lat, Some(36.0292900));
                    assert_eq!(n.tags.len(), 1);
                    assert_eq!(n.tags["created_by"], "JOSM");
                }
                _ => { assert_eq!(0, 1); }
            }
        }

        #[test]
        fn decode_line_node2() {
            use super::decode_line;
            use super::super::OSMObj;

            let line = "n197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon T x14.2741628 y36.0292900";
            match decode_line(line).unwrap() {
                OSMObj::Node(n) => { 
                    assert_eq!(n.id, 197801);
                    assert_eq!(n.version, 6);
                    assert_eq!(n.deleted, false);
                    assert_eq!(n.changeset_id, 10009832);
                    assert_eq!(n.timestamp, "2011-12-01T17:03:42Z");
                    assert_eq!(n.uid, 20673);
                    assert_eq!(n.user, "mikefalzon");
                    assert_eq!(n.lon, Some(14.2741628));
                    assert_eq!(n.lat, Some(36.0292900));
                    assert_eq!(n.tags.len(), 0);
                }
                _ => { assert_eq!(0, 1); }
            }
        }

        #[test]
        fn decode_line_node3() {
            use super::decode_line;
            use super::super::OSMObj;

            let line = "n1 v1 dD c9257 t2006-05-10T18:27:47Z i1298 u\u{3c4}12 T x y";
            match decode_line(line).unwrap() {
                OSMObj::Node(n) => { 
                    assert_eq!(n.id, 1);
                    assert_eq!(n.version, 1);
                    assert_eq!(n.deleted, true);
                    assert_eq!(n.changeset_id, 9257);
                    assert_eq!(n.timestamp, "2006-05-10T18:27:47Z");
                    assert_eq!(n.uid, 1298);
                    //assert_eq!(n.user, "");
                    assert_eq!(n.lon, None);
                    assert_eq!(n.lat, None);
                    assert_eq!(n.tags.len(), 0);
                }
                _ => { assert_eq!(0, 1); }
            }
        }

        #[test]
        fn decode_line_node4() {
            use super::decode_line;
            use super::super::OSMObj;

            // test for panic
            let line = "n1 v1 € c9257 t2006-05-10T18:27:47Z i1298 u\u{3c4}12 T x y";
            match decode_line(line) {
                Err(DecodeStringError) => { assert_eq!(1, 1); }
                _ => { assert_eq!(0, 1); }
            }
        }


        #[test]
        fn decode_line_way() {
            use super::decode_line;
            use super::super::OSMObj;

            let line = "w197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Nn1,n2,n3";
            match decode_line(line).unwrap() {
                OSMObj::Way(w) => { 
                    assert_eq!(w.id, 197801);
                    assert_eq!(w.version, 6);
                    assert_eq!(w.deleted, false);
                    assert_eq!(w.changeset_id, 10009832);
                    assert_eq!(w.timestamp, "2011-12-01T17:03:42Z");
                    assert_eq!(w.uid, 20673);
                    assert_eq!(w.user, "mikefalzon");
                    assert_eq!(w.tags.len(), 1);
                    assert_eq!(w.tags["created_by"], "JOSM");
                    assert_eq!(w.nodes, vec![1, 2, 3]);
                }
                _ => { assert_eq!(0, 1); }
            }
        }

        #[test]
        fn decode_line_relation() {
            use super::decode_line;
            use super::super::OSMObj;

            let line = "r197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Mn1@inner,w2@outer";
            match decode_line(line).unwrap() {
                OSMObj::Relation(r) => { 
                    assert_eq!(r.id, 197801);
                    assert_eq!(r.version, 6);
                    assert_eq!(r.deleted, false);
                    assert_eq!(r.changeset_id, 10009832);
                    assert_eq!(r.timestamp, "2011-12-01T17:03:42Z");
                    assert_eq!(r.uid, 20673);
                    assert_eq!(r.user, "mikefalzon");
                    assert_eq!(r.tags.len(), 1);
                    assert_eq!(r.tags["created_by"], "JOSM");
                    assert_eq!(r.members, vec![('n', 1, "inner".to_string()), ('w', 2, "outer".to_string())]);
                }
                _ => { assert_eq!(0, 1); }
            }
        }

    }

}

//impl<R: Read> OsmReader<R> for XmlReader<R> {
//    fn new(reader: R) -> XmlReader<R> {
//        XmlReader { inner_reader: reader }
//    }
//    fn objects(&self) -> Box<Iterator<Item=OSMObj>> {
//        Box::new(vec![OSMObj::Node(Node{ id: 0, lat: Some(0f32), lon: Some(0f32), tags: HashMap::new()})].iter())
//    }
//}








#[cfg(test)]
mod test {


    #[test]
    fn test_eq() {
        use std::io::Cursor;
        use super::XMLReader;
        use super::OSMReader;

        let sample1 = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<osm version=\"0.6\" generator=\"CGImap 0.0.2\">
 <bounds minlat=\"54.0889580\" minlon=\"12.2487570\" maxlat=\"54.0913900\" maxlon=\"12.2524800\"/>
 <node id=\"298884269\" lat=\"54.0901746\" lon=\"12.2482632\" user=\"SvenHRO\" uid=\"46882\" visible=\"true\" version=\"1\" changeset=\"676636\" timestamp=\"2008-09-21T21:37:45Z\"/></osm>";
        //let sample1_cursor = Cursor::new(sample1);

        //let reader = XMLReader::new(sample1_cursor);
        //assert_eq!(reader.next(), None);

        //let objs = reader.objects();
        //assert_eq!(objs.next(), None);
        
    }
}

