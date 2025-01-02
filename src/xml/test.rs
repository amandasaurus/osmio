#![cfg(test)]
use std::io::Cursor;
use smol_str::SmolStr;

use crate::obj_types::{StringNode, StringNodeBuilder, StringOSMObj, StringRelation, StringRelationBuilder, StringWay, StringWayBuilder};
use crate::{Lat, Lon, OSMObjBase, OSMObjectType};

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
        StringOSMObj::Node(n) => {
            assert_eq!(n._id, 298884269);
            assert_eq!(n._version, Some(1));
            assert_eq!(n._deleted, false);
            assert_eq!(n._changeset_id, Some(676636));
            assert_eq!(n._timestamp.unwrap().to_iso_string(), "2008-09-21T21:37:45Z");
            assert_eq!(n._uid, Some(46882));
            assert_eq!(n._user, Some(SmolStr::from("SvenHRO")));
            assert_eq!(n._lat_lon, Some((Lat::try_from(54.0901746).unwrap(), Lon::try_from(12.2482632).unwrap())));
            assert_eq!(n._tags.len(), 1);
            assert_eq!(n._tags[0], (SmolStr::from("mytag"), SmolStr::from("myvalue")));

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
        Some(StringOSMObj::Node(n)) => {
            assert_eq!(n._id, 298884269);
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
        Some(StringOSMObj::Node(_)) => {
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(StringOSMObj::Node(n)) => {
            assert_eq!(n._deleted, true);
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(StringOSMObj::Node(_)) => {
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(StringOSMObj::Way(w)) => {
            assert_eq!(w.nodes(), vec![292403538, 298884289, 261728686]);
            assert_eq!(w._tags.len(), 2);
            assert_eq!(w.tag("highway"), Some("unclassified"));
        },
        _ => { assert!(false); },
    }

    match reader.next() {
        Some(StringOSMObj::Relation(r)) => {
            assert_eq!(r._members, vec![(OSMObjectType::Node, 294942404, "".into()), (OSMObjectType::Node, 364933006, "".into()), (OSMObjectType::Way, 4579143, "".into()), (OSMObjectType::Node, 249673494, "".into())]);
            assert_eq!(r._tags.len(), 6);
            assert_eq!(r.tag("ref"), Some("123"));
        },
        _ => { assert!(false); },
    }
}

#[test]
fn test_writer_empty() {
    let mut output_cursor = Vec::new();
    let mut xml_writer = XMLWriter::new(&mut output_cursor);

    drop(xml_writer);

    let output = String::from_utf8(output_cursor).unwrap();
    assert_eq!(output, format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\">\n</osm>", version()));
}

#[test]
fn test_writer() {
    let mut output_cursor = Vec::new();
    let mut xml_writer = XMLWriter::new(&mut output_cursor);

    let mut node = StringNodeBuilder::default()._id(1)._version(1)._deleted(false)._changeset_id(1)._uid(2)._timestamp(TimestampFormat::ISOString("900 CE".to_string())).build().unwrap();
    node.set_lat_lon((1.,2.));
    node.set_user("Username");
    node.set_tag("highway", "nevar");
    xml_writer.write_obj(&StringOSMObj::Node(node)).ok();

    let mut way = StringWayBuilder::default()._id(2).build().unwrap();
    way.set_version(1);
    way.set_deleted(false);
    way.set_changeset_id(1);
    way.set_user("Username");
    way.set_uid(2);
    way.set_timestamp(TimestampFormat::ISOString("900 CE".to_string()));
    way.set_tag("highway", "nevar");
    way.set_nodes(vec![1, 2, 3]);
    xml_writer.write_obj(&StringOSMObj::Way(way)).ok();

    let mut relation = StringRelationBuilder::default()._id(2).build().unwrap();
    relation.set_version(1);
    relation.set_deleted(false);
    relation.set_changeset_id(1);
    relation.set_user("Username");
    relation.set_uid(2);
    relation.set_timestamp(TimestampFormat::ISOString("900 CE".to_string()));
    relation.set_tag("highway", "nevar");
    relation.set_members(vec![(OSMObjectType::Node, 1, ""),(OSMObjectType::Way, 2, "")]);
    xml_writer.write_obj(&StringOSMObj::Relation(relation)).ok();

    xml_writer.close();
    drop(xml_writer);

    let output = String::from_utf8(output_cursor).unwrap();
    //println!("{}", output);
    assert_eq!(output, format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\">\n\t<node id=\"1\" visible=\"true\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\" lat=\"1\" lon=\"2\">\n\t\t<tag k=\"highway\" v=\"nevar\" />\n\t</node>\n\t<way id=\"2\" visible=\"true\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\">\n\t\t<nd ref=\"1\" />\n\t\t<nd ref=\"2\" />\n\t\t<nd ref=\"3\" />\n\t\t<tag k=\"highway\" v=\"nevar\" />\n\t</way>\n\t<relation id=\"2\" visible=\"true\" version=\"1\" user=\"Username\" uid=\"2\" changeset=\"1\" timestamp=\"900 CE\">\n\t\t<member type=\"node\" ref=\"1\" role=\"\" />\n\t\t<member type=\"way\" ref=\"2\" role=\"\" />\n\t\t<tag k=\"highway\" v=\"nevar\" />\n\t</relation>\n</osm>", version()));
}