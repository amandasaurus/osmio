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
            assert_eq!(n.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
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
            assert_eq!(n.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
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
            assert_eq!(n.timestamp.to_iso_string(), "2006-05-10T18:27:47Z");
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
            assert_eq!(w.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
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
            assert_eq!(r.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
            assert_eq!(r.uid, 20673);
            assert_eq!(r.user, "mikefalzon");
            assert_eq!(r.tags.len(), 1);
            assert_eq!(r.tags["created_by"], "JOSM");
            assert_eq!(r.members, vec![('n', 1, "inner".to_string()), ('w', 2, "outer".to_string())]);
        }
        _ => { assert_eq!(0, 1); }
    }
}

#[test]
fn test_reader() {
    use std::io::Cursor;
    use super::{OPLReader, OPLWriter};
    use super::super::{OSMReader, OSMWriter, OSMObj};

    let input_line = "n197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM x14.274163 y36.02929\nw197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Nn1,n2,n3\nr197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Mn1@inner,w2@outer\n";

    let cursor = Cursor::new(input_line);
    let mut opl_reader = OPLReader::new(cursor);

    let output_cursor = Cursor::new(Vec::new());
    let mut opl_writer = OPLWriter::new(output_cursor);

    match opl_reader.next() {
        Some(OSMObj::Node(n)) => {
            assert_eq!(n.id, 197801);
            assert_eq!(n.version, 6);
            assert_eq!(n.deleted, false);
            assert_eq!(n.changeset_id, 10009832);
            assert_eq!(n.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
            assert_eq!(n.uid, 20673);
            assert_eq!(n.user, "mikefalzon");
            assert_eq!(n.lon, Some(14.2741628));
            assert_eq!(n.lat, Some(36.0292900));
            assert_eq!(n.tags.len(), 1);
            assert_eq!(n.tags["created_by"], "JOSM");

            opl_writer.write_obj(&OSMObj::Node(n)).unwrap();
        },
        _ => { assert!(false); },
    }

    match opl_reader.next() {
        Some(OSMObj::Way(w)) => {
            assert_eq!(w.id, 197801);
            assert_eq!(w.version, 6);
            assert_eq!(w.deleted, false);
            assert_eq!(w.changeset_id, 10009832);
            assert_eq!(w.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
            assert_eq!(w.uid, 20673);
            assert_eq!(w.user, "mikefalzon");
            assert_eq!(w.tags.len(), 1);
            assert_eq!(w.tags["created_by"], "JOSM");
            assert_eq!(w.nodes, vec![1, 2, 3]);

            opl_writer.write_obj(&OSMObj::Way(w)).unwrap();
        },
        _ => { assert!(false); },
    }

    match opl_reader.next() {
        Some(OSMObj::Relation(r)) => {
            assert_eq!(r.id, 197801);
            assert_eq!(r.version, 6);
            assert_eq!(r.deleted, false);
            assert_eq!(r.changeset_id, 10009832);
            assert_eq!(r.timestamp.to_iso_string(), "2011-12-01T17:03:42Z");
            assert_eq!(r.uid, 20673);
            assert_eq!(r.user, "mikefalzon");
            assert_eq!(r.tags.len(), 1);
            assert_eq!(r.tags["created_by"], "JOSM");
            assert_eq!(r.members, vec![('n', 1, "inner".to_string()), ('w', 2, "outer".to_string())]);

            opl_writer.write_obj(&OSMObj::Relation(r)).unwrap();
        },
        _ => { assert!(false); },
    }

    assert!(opl_reader.next().is_none());

    opl_writer.close();

    let output = String::from_utf8(opl_writer.into_inner().into_inner()).unwrap();
    assert_eq!(output, input_line);

}

