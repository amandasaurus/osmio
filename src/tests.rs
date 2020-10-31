use super::*;

#[test]
fn char_to_objtype() {
    assert_eq!(OSMObjectType::try_from('n'), Ok(OSMObjectType::Node));
    assert_eq!(OSMObjectType::try_from('w'), Ok(OSMObjectType::Way));
    assert_eq!(OSMObjectType::try_from('r'), Ok(OSMObjectType::Relation));
    assert!(OSMObjectType::try_from('x').is_err());
}

#[test]
fn string_to_objtype() {
    assert_eq!("n".parse(), Ok(OSMObjectType::Node));
    assert_eq!("node".parse(), Ok(OSMObjectType::Node));
    assert_eq!("w".parse(), Ok(OSMObjectType::Way));
    assert_eq!("way".parse(), Ok(OSMObjectType::Way));
    assert_eq!("r".parse(), Ok(OSMObjectType::Relation));
    assert_eq!("relation".parse(), Ok(OSMObjectType::Relation));
    assert_eq!("rel".parse(), Ok(OSMObjectType::Relation));

    assert!(" ".parse::<OSMObjectType>().is_err());
}

mod timestamp_format {
	use super::*;
	use std::cmp::Ordering::*;
	use std::cmp::*;

	macro_rules! assert_cmp {
		( $name:ident, $a:expr, $b:expr, $expected_ord:expr ) => {
			#[test]
			fn $name() {
			    let a: TimestampFormat = $a;
			    let b: TimestampFormat = $b;
                assert_eq!(a.partial_cmp(&b), Some($expected_ord));
			}
		};
	}

	assert_cmp!(ints1, 0.into(), 0.into(), Equal);
	assert_cmp!(ints2, 0.into(), 1.into(), Less);
	assert_cmp!(ints3, 3.into(), 1.into(), Greater);

	assert_cmp!(isos1, "2020-01-01T00:00:00Z".parse().unwrap(), "2020-01-01T00:00:00Z".parse().unwrap(), Equal);
	assert_cmp!(isos2, "2010-01-01T00:00:00Z".parse().unwrap(), "2020-01-01T00:00:00Z".parse().unwrap(), Less);
	assert_cmp!(isos3, "2020-01-01T00:00:00Z".parse().unwrap(), "2000-01-01T00:00:00Z".parse().unwrap(), Greater);

	assert_cmp!(int_iso1, 1577836800.into(), "2020-01-01T00:00:00Z".parse().unwrap(), Equal);
	assert_cmp!(int_iso2, "2020-01-01T00:00:00Z".parse().unwrap(), 1577836800.into(), Equal);

	assert_cmp!(int_iso3, TimestampFormat::EpochNunber(1577836800), TimestampFormat::ISOString("2020-01-01T00:00:00Z".to_string()), Equal);
	assert_cmp!(int_iso4, TimestampFormat::ISOString("2020-01-01T00:00:00Z".to_string()),TimestampFormat::EpochNunber(1577836800), Equal);

}
