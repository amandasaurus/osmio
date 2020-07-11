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
