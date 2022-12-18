#![cfg(test)]
use std::io::Cursor;
use super::{XMLReader, XMLWriter};
use super::super::{OSMReader, OSMWriter};
use std::iter::Iterator;
use super::super::OSMObj;
use super::super::TimestampFormat;
use super::super::{Node, Way, Relation};
use std::collections::HashMap;
use super::super::version;

#[test]
fn test_parsing1() {

    let sample1 = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<osm version=\"0.6\" generator=\"CGImap 0.0.2\">
<bounds minlat=\"54.0889580\" minlon=\"12.2487570\" maxlat=\"54.0913900\" maxlon=\"12.2524800\"/>
<node id=\"298884269\" lat=\"54.0901746\" lon=\"12.2482632\" user=\"SvenHRO\" uid=\"46882\" visible=\"true\" version=\"1\" changeset=\"676636\" timestamp=\"2008-09-21T21:37:45Z\"><tag k=\"mytag\" v=\"myvalue\"/></node></osm>";
    let sample1_cursor = Cursor::new(sample1);

    let mut reader = XMLReader::new(sample1_cursor);
    let obj = reader.next();
    assert!(!obj.is_none());
    let obj = obj.unwrap();
    match obj {
        OSMObj::Node(n) => {
            assert_eq!(n.id, 298884269);
            assert_eq!(n.version, 1);
            assert_eq!(n.deleted, false);
            assert_eq!(n.changeset_id, 676636);
            assert_eq!(n.timestamp.to_iso_string(), "2008-09-21T21:37:45Z");
            assert_eq!(n.uid, 46882);
            assert_eq!(n.user, "SvenHRO");
            assert_eq!(n.lat, Some(54.0901746));
            assert_eq!(n.lon, Some(12.2482632));
            assert_eq!(n.tags.len(), 1);
            assert_eq!(n.tags["mytag"], "myvalue");

        },
        _ => { assert!(false); },
    }
    assert!(reader.next().is_none());
    assert!(reader.next().is_none());
    assert!(reader.next().is_none());
    assert!(reader.next().is_none());

    
}

#[test]
fn test_parsing2() {
    let sample1 = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<osm version=\"0.6\" generator=\"CGImap 0.0.2\">
 <bounds minlat=\"54.0889580\" minlon=\"12.2487570\" maxlat=\"54.0913900\" maxlon=\"12.2524800\"/>
 <node id=\"298884269\" lat=\"54.0901746\" lon=\"12.2482632\" user=\"SvenHRO\" uid=\"46882\" visible=\"true\" version=\"1\" changeset=\"676636\" timestamp=\"2008-09-21T21:37:45Z\"/>
 <node id=\"261728686\" lat=\"54.0906309\" lon=\"12.2441924\" user=\"PikoWinter\" uid=\"36744\" visible=\"true\" version=\"1\" changeset=\"323878\" timestamp=\"2008-05-03T13:39:23Z\"/>
 <node id=\"1831881213\" version=\"1\" changeset=\"12370172\" lat=\"54.0900666\" lon=\"12.2539381\" user=\"lafkor\" uid=\"75625\" visible=\"false\" timestamp=\"2012-07-20T09:43:19Z\">
  <tag k=\"name\" v=\"Neu Broderstorf\"/>
  <tag k=\"traffic_sign\" v=\"city_limit\"/>
 </node>
 <node id=\"298884272\" lat=\"54.0901447\" lon=\"12.2516513\" user=\"SvenHRO\" uid=\"46882\" visible=\"true\" version=\"1\" changeset=\"676636\" timestamp=\"2008-09-21T21:37:45Z\"/>
 <way id=\"26659127\" user=\"Masch\" uid=\"55988\" visible=\"true\" version=\"5\" changeset=\"4142606\" timestamp=\"2010-03-16T11:47:08Z\">
  <nd ref=\"292403538\"/>
  <nd ref=\"298884289\"/>
  <nd ref=\"261728686\"/>
  <tag k=\"highway\" v=\"unclassified\"/>
  <tag k=\"name\" v=\"Pastower Straße\"/>
 </way>
 <relation id=\"56688\" user=\"kmvar\" uid=\"56190\" visible=\"true\" version=\"28\" changeset=\"6947637\" timestamp=\"2011-01-12T14:23:49Z\">
  <member type=\"node\" ref=\"294942404\" role=\"\"/>
  <member type=\"node\" ref=\"364933006\" role=\"\"/>
  <member type=\"way\" ref=\"4579143\" role=\"\"/>
  <member type=\"node\" ref=\"249673494\" role=\"\"/>
  <tag k=\"name\" v=\"Küstenbus Linie 123\"/>
  <tag k=\"network\" v=\"VVW\"/>
  <tag k=\"operator\" v=\"Regionalverkehr Küste\"/>
  <tag k=\"ref\" v=\"123\"/>
  <tag k=\"route\" v=\"bus\"/>
  <tag k=\"type\" v=\"route\"/>
 </relation>
</osm>
";

    let mut reader = XMLReader::new(Cursor::new(sample1));

    match reader.next() {
        Some(OSMObj::Node(n)) => {
            assert_eq!(n.id, 298884269);
            //assert_eq!(n.version, 1);
            //assert_eq!(n.deleted, false);
            //assert_eq!(n.changeset_id, 676636);
            //assert_eq!(n.timestamp, "2008-09-21T21:37:45Z");
            //assert_eq!(n.uid, 46882);
            //assert_eq!(n.user, "SvenHRO");
            //assert_eq!(n.lat, Some(54.0901746));
            //assert_eq!(n.lon, Some(12.2482632));
            //assert_eq!(n.tags.len(), 1);
            //assert_eq!(n.tags["mytag"], "myvalue");

        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(OSMObj::Node(_)) => {
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(OSMObj::Node(n)) => {
            assert_eq!(n.deleted, true);
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(OSMObj::Node(_)) => {
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(OSMObj::Way(w)) => {
            assert_eq!(w.nodes, vec![292403538, 298884289, 261728686]);
            assert_eq!(w.tags.len(), 2);
            assert_eq!(w.tags["highway"], "unclassified");
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(OSMObj::Relation(r)) => {
            assert_eq!(r.members, vec![('n', 294942404, "".to_string()), ('n', 364933006, "".to_string()), ('w', 4579143, "".to_string()), ('n', 249673494, "".to_string())]);
            assert_eq!(r.tags.len(), 6);
            assert_eq!(r.tags["ref"], "123");
        },
        _ => { assert!(false); },
    }
}

#[test]
fn test_writer_empty() {
    let output_cursor = Cursor::new(Vec::new());
    let mut xml_writer = XMLWriter::new(output_cursor);

    xml_writer.close();

    let output = String::from_utf8(xml_writer.into_inner().into_inner()).unwrap();
    assert_eq!(output, format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\" />", version()));
}

#[test]
fn test_writer() {
    let output_cursor = Cursor::new(Vec::new());
    let mut xml_writer = XMLWriter::new(output_cursor);

    let mut tags = HashMap::new();
    tags.insert("highway".to_string(), "nevar".to_string());
    xml_writer.write_obj(&OSMObj::Node(Node{ id: 1, version: 1, deleted: false, changeset_id: 1, user: "Username".to_string(), uid: 2, lat: Some(1.), lon: Some(2.), timestamp: TimestampFormat::ISOString("900 CE".to_string()), tags: tags.clone() })).ok();

    xml_writer.write_obj(&OSMObj::Way(Way{ id: 2, version: 1, deleted: false, changeset_id: 1, user: "Username".to_string(), uid: 2, timestamp: TimestampFormat::ISOString("900 CE".to_string()), tags: tags.clone(), nodes: vec![1, 2, 3] })).ok();

    xml_writer.write_obj(&OSMObj::Relation(Relation{ id: 2, version: 1, deleted: false, changeset_id: 1, user: "Username".to_string(), uid: 2, timestamp: TimestampFormat::ISOString("900 CE".to_string()), tags: tags.clone(), members: vec![('n', 1, "".to_string()), ('w', 2, "".to_string() ) ]})).ok();

    xml_writer.close();

    let output = String::from_utf8(xml_writer.into_inner().into_inner()).unwrap();
    //println!("{}", output);
    assert_eq!(output, format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\">\n  <node id=\"1\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\" lat=\"1\" lon=\"2\" visible=\"true\">\n    <tag k=\"highway\" v=\"nevar\" />\n  </node>\n  <way id=\"2\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\" visible=\"true\">\n    <nd ref=\"1\" />\n    <nd ref=\"2\" />\n    <nd ref=\"3\" />\n    <tag k=\"highway\" v=\"nevar\" />\n  </way>\n  <relation id=\"2\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\" visible=\"true\">\n    <member type=\"node\" ref=\"1\" role=\"\" />\n    <member type=\"way\" ref=\"2\" role=\"\" />\n    <tag k=\"highway\" v=\"nevar\" />\n  </relation>\n</osm>", version()));


}
