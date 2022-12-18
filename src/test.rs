#[test]
fn test_read_then_write() {
    use std::io::Cursor;
    use super::opl::{OPLReader, OPLWriter};
    use super::xml::{XMLReader, XMLWriter};
    use super::{OSMReader, OSMWriter, OSMObj};
    use super::version;

    let input_line = "n197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM x14.274163 y36.02929\nw197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Nn1,n2,n3\nr197801 v6 dV c10009832 t2011-12-01T17:03:42Z i20673 umikefalzon Tcreated_by=JOSM Mn1@inner,w2@outer\n";

    let cursor = Cursor::new(input_line);
    let mut opl_reader = OPLReader::new(cursor);

    let output_cursor = Cursor::new(Vec::new());
    let mut xml_writer = XMLWriter::from_iter(output_cursor, opl_reader.objects());

    let output = String::from_utf8(xml_writer.into_inner().into_inner()).unwrap();
    assert_eq!(output, format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<osm version=\"0.6\" generator=\"osmio/{}\">\n  <node id=\"197801\" version=\"6\" user=\"mikefalzon\" uid=\"20673\" changeset=\"10009832\" timestamp=\"2011-12-01T17:03:42Z\" lat=\"36.02929\" lon=\"14.274163\" visible=\"true\">\n    <tag k=\"created_by\" v=\"JOSM\" />\n  </node>\n  <way id=\"197801\" version=\"6\" user=\"mikefalzon\" uid=\"20673\" changeset=\"10009832\" timestamp=\"2011-12-01T17:03:42Z\" visible=\"true\">\n    <nd ref=\"1\" />\n    <nd ref=\"2\" />\n    <nd ref=\"3\" />\n    <tag k=\"created_by\" v=\"JOSM\" />\n  </way>\n  <relation id=\"197801\" version=\"6\" user=\"mikefalzon\" uid=\"20673\" changeset=\"10009832\" timestamp=\"2011-12-01T17:03:42Z\" visible=\"true\">\n    <member type=\"node\" ref=\"1\" role=\"inner\" />\n    <member type=\"way\" ref=\"2\" role=\"outer\" />\n    <tag k=\"created_by\" v=\"JOSM\" />\n  </relation>\n</osm>", version()));

}

#[test]
fn test_time() {
    use super::utils::{epoch_to_iso, iso_to_epoch};
    assert_eq!(epoch_to_iso(1), "1970-01-01T00:00:01+00:00");
    assert_eq!(iso_to_epoch("1970-01-01T00:00:01Z"), 1);
    assert_eq!(iso_to_epoch("1970-01-01T00:00:01+00:00"), 1);
}
