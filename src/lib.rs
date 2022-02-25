//! Read and write OpenStreetMap files
//!
//! # Reading files
//!
//! ```rust
//! use osmio::prelude::*;
//!
//! let reader = osmio::read_pbf("path/to/filename.osm.pbf")?;
//! for obj in reader.objects() {
//!     ...
//! }
//! ```
extern crate byteorder;
extern crate chrono;
extern crate flate2;
extern crate protobuf;
extern crate quick_xml;
extern crate xml as xml_rs;
#[macro_use]
extern crate derive_builder;
extern crate anyhow;
extern crate bzip2;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::iter::{ExactSizeIterator, Iterator};
use std::path::Path;
use std::str::FromStr;
use utils::{epoch_to_iso, iso_to_epoch};

use anyhow::Result;

#[macro_use]
pub mod utils;

pub mod pbf;
pub mod xml;
//pub mod opl;
pub mod osc;

pub mod obj_types;

#[cfg(test)]
mod tests;

pub mod changesets;

/// Type that stores the OSM Id
pub type ObjId = i64;

/// How many nanodegrees are represented by each unit in [`Lat::inner()`].
/// We use the same internal precision as OpenStreetMap.org, 100 nanodegrees.
pub const COORD_PRECISION_NANOS: i32 = 100;

/// Number of internal units (as returned from [`Lat::inner`]) in one degree.
///
/// See [`COORD_PRECISION_NANOS`].
pub const COORD_SCALE_FACTOR: f64 = (1_000_000_000 / COORD_PRECISION_NANOS) as f64;

pub mod prelude {
    //! Useful things for osmio
    pub use crate::OSMReader;
    pub use crate::OSMObjectType;
    pub use crate::{Node, Way, Relation};
}

macro_rules! lat_lon_impl {
    ($lat_or_lon: ident) => {
        /// Latitude and Longitude are stored internally as a 32-bit signed integer, in units
        /// of [`COORD_PRECISION_NANOS`].
        ///
        /// This gives us 7 decimal places of precision - the same precision that OSM uses.
        ///
        /// ```
        /// use std::convert::TryFrom;
        /// use osmio::Lat;
        /// let lat = Lat::try_from(1.0).unwrap();
        /// let float_lat: f64 = lat.into();
        /// assert_eq!(float_lat, 1.);
        /// ```
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $lat_or_lon(i32);

        impl $lat_or_lon {
            /// Build a Lat/Lon from it's inner representation, which is `degrees * 1e7`.
            ///
            /// ```
            /// use osmio::Lat;
            /// // build a Lat for 1.2345678 degrees South
            /// let lat = Lat::from_inner(12345678);
            /// ```
            pub fn from_inner(inner: i32) -> Self {
                Self(inner)
            }

            pub fn inner(&self) -> i32 {
                self.0
            }

            /// Returns the number of degrees as a 64-bit float.
            ///
            /// Note: The actual precision is [`COORD_PRECISION_NANOS`], which is less than
            /// 64-bits. It is derived from an inner i32 representation, which mirrors the
            /// precision used by OpenStreetMap.org
            pub fn degrees(&self) -> f64 {
                self.0 as f64 / COORD_SCALE_FACTOR
            }
        }

        impl Display for $lat_or_lon {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: fix precision
                Display::fmt(&self.degrees(), f)
            }
        }

        impl Debug for $lat_or_lon {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: fix precision
                Debug::fmt(&self.degrees(), f)
            }
        }

        /// ```rust
        ///  use osmio::Lat;
        ///  use std::str::FromStr;
        ///
        ///  let lat = Lat::from_str("1.23").unwrap();
        ///  assert_eq!(1.23, lat.degrees());
        ///  assert_eq!(12300000, lat.inner());
        ///
        ///  assert!(Lat::from_str("-180.0").is_ok());
        ///  assert!(Lat::from_str("xxxx").is_err());
        ///  assert!(Lat::from_str("600.0").is_err());
        /// ```
        impl FromStr for $lat_or_lon {
            type Err = ParseLatLonError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match (f64::from_str(s)? * COORD_SCALE_FACTOR).round() {
                    x if x > (i32::MAX as f64) => Err(ParseLatLonError::TooLarge(x)),
                    x if x < (i32::MIN as f64) => Err(ParseLatLonError::TooSmall(x)),
                    x => Ok(Self(x as i32)),
                }
            }
        }

        impl From<$lat_or_lon> for f64 {
            fn from(val: $lat_or_lon) -> f64 {
                val.degrees()
            }
        }

        impl TryFrom<f64> for $lat_or_lon {
            type Error = ParseLatLonError;
            fn try_from(val: f64) -> Result<$lat_or_lon, Self::Error> {
                match (val * COORD_SCALE_FACTOR).round() {
                    x if x > (i32::MAX as f64) => Err(ParseLatLonError::TooLarge(x)),
                    x if x < (i32::MIN as f64) => Err(ParseLatLonError::TooSmall(x)),
                    x => Ok(Self(x as i32)),
                }
            }
        }

        impl TryFrom<f32> for $lat_or_lon {
            type Error = ParseLatLonError;
            fn try_from(val: f32) -> Result<$lat_or_lon, Self::Error> {
                $lat_or_lon::try_from(val as f64)
            }
        }
    };
}

// Latitude
lat_lon_impl!(Lat);

// Longitude
lat_lon_impl!(Lon);

/// An error while trying to parse a string into a [`Lat`] or [`Lon`]
#[derive(Debug)]
pub enum ParseLatLonError {
    /// Number was not parseable as a Float, wraps the underlying [`std::num::ParseFloatError`].
    ParseFloatError(std::num::ParseFloatError),

    /// The parsed float was too large to fit into our Lat/Lon representation.
    ///
    /// This should never happen for "normal" Lats and Lons, i.e. those between (-90..90) or
    /// (-180..+180), respectively.
    TooLarge(f64),

    /// The parsed float was too small to fit into our Lat/Lon representation.
    ///
    /// This should never happen for "normal" Lats and Lons, i.e. those between (-90..90) or
    /// (-180..+180), respectively.
    TooSmall(f64),
}

impl std::error::Error for ParseLatLonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::ParseFloatError(inner) = self {
            Some(inner)
        } else {
            None
        }
    }
}

impl Display for ParseLatLonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFloatError(inner) => Display::fmt(inner, f),
            Self::TooLarge(float) => write!(f, "{} is too large to represent as a Lat/Lon", float),
            Self::TooSmall(float) => write!(f, "{} is too small to represent as a Lat/Lon", float),
        }
    }
}

impl From<std::num::ParseFloatError> for ParseLatLonError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParseLatLonError::ParseFloatError(err)
    }
}

/// Timestamps can be stored as an ISO formatted string, or number of seconds since unix epoch
///
/// In XML files, timestamps are represented as ISO strings, and in PBF files, as integer seconds
/// since the epoch
#[derive(Debug, Clone, Eq, Ord)]
pub enum TimestampFormat {
    ISOString(String),
    EpochNunber(i64),
}

impl TimestampFormat {
    pub fn to_iso_string(&self) -> String {
        match self {
            &TimestampFormat::ISOString(ref s) => s.clone(),
            &TimestampFormat::EpochNunber(ref t) => epoch_to_iso(*t as i32),
        }
    }

    pub fn to_epoch_number(&self) -> i64 {
        match self {
            &TimestampFormat::ISOString(ref s) => iso_to_epoch(s) as i64,
            &TimestampFormat::EpochNunber(t) => t,
        }
    }
}

impl<T> From<T> for TimestampFormat
where
    T: Into<i64>,
{
    fn from(v: T) -> Self {
        TimestampFormat::EpochNunber(v.into())
    }
}

impl std::str::FromStr for TimestampFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let date: i64 = chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|_| "invalid date")?
            .timestamp();
        Ok(TimestampFormat::EpochNunber(date))
    }
}

impl fmt::Display for TimestampFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_iso_string())
    }
}

impl std::cmp::PartialOrd for TimestampFormat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TimestampFormat::ISOString(a), TimestampFormat::ISOString(b)) => a.partial_cmp(b),
            (TimestampFormat::EpochNunber(a), TimestampFormat::EpochNunber(b)) => a.partial_cmp(b),
            (a, b) => a.to_epoch_number().partial_cmp(&b.to_epoch_number()),
        }
    }
}
impl std::cmp::PartialEq for TimestampFormat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimestampFormat::ISOString(a), TimestampFormat::ISOString(b)) => a.eq(b),
            (TimestampFormat::EpochNunber(a), TimestampFormat::EpochNunber(b)) => a.eq(b),
            (a, b) => a.to_epoch_number().eq(&b.to_epoch_number()),
        }
    }
}

/// The basic metadata fields all OSM objects share
pub trait OSMObjBase: PartialEq + Debug + Clone {
    fn id(&self) -> ObjId;
    fn set_id(&mut self, val: impl Into<ObjId>);
    fn version(&self) -> Option<u32>;
    fn set_version(&mut self, val: impl Into<Option<u32>>);
    fn deleted(&self) -> bool;
    fn set_deleted(&mut self, val: bool);
    fn changeset_id(&self) -> Option<u32>;
    fn set_changeset_id(&mut self, val: impl Into<Option<u32>>);
    fn timestamp(&self) -> &Option<TimestampFormat>;
    fn set_timestamp(&mut self, val: impl Into<Option<TimestampFormat>>);
    fn uid(&self) -> Option<u32>;
    fn set_uid(&mut self, val: impl Into<Option<u32>>);
    fn user(&self) -> Option<&str>;
    fn set_user<'a>(&mut self, val: impl Into<Option<&'a str>>);

    fn tags<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a>;
    fn tag(&self, key: impl AsRef<str>) -> Option<&str>;
    fn has_tag(&self, key: impl AsRef<str>) -> bool {
        self.tag(key).is_some()
    }
    fn num_tags(&self) -> usize {
        self.tags().count()
    }

    /// True iff this object has tags
    fn tagged(&self) -> bool {
        !self.untagged()
    }
    /// True iff this object has no tags
    fn untagged(&self) -> bool {
        self.num_tags() == 0
    }

    fn set_tag(&mut self, key: impl AsRef<str>, value: impl Into<String>);
    fn unset_tag(&mut self, key: impl AsRef<str>);

    fn strip_metadata(&mut self) {
        self.set_uid(None);
        self.set_user(None);
        self.set_changeset_id(None);
    }
}

/// A Node
pub trait Node: OSMObjBase {
    /// Latitude & Longitude of the node (if it's set)
    fn lat_lon(&self) -> Option<(Lat, Lon)>;

    /// Latitude & Longitude of the node as `f64` (if it's set)
    fn lat_lon_f64(&self) -> Option<(f64, f64)> {
        self.lat_lon().map(|(lat, lon)| (lat.into(), lon.into()))
    }
    /// True iff this node has latitude & longitude set
    fn has_lat_lon(&self) -> bool {
        self.lat_lon().is_some()
    }

    /// Remove the lat & lon for this node
    fn unset_lat_lon(&mut self) {
        self.set_lat_lon_direct(None);
    }

    /// Directly set the lat & lon of the node, see `set_lat_lon()` as a more convienient method.
    fn set_lat_lon_direct(&mut self, loc: Option<(Lat, Lon)>);

    /// Set the Latitude & Longitude.
    ///
    /// The type signature is complicated so you can convert from f64
    /// ```rust
    /// use osmio::Node;
    /// # use osmio::obj_types::StringNodeBuilder;
    /// # let mut node = StringNodeBuilder::default()._id(1).build().unwrap();
    ///
    /// // It can convert from f64
    /// node.set_lat_lon((0.0_f64, 0.0_f64));
    /// assert_eq!(node.lat_lon_f64().unwrap(), (0., 0.));
    ///
    /// // .. or from f32
    /// node.set_lat_lon((0.0_f32, 0.0_f32));
    /// assert_eq!(node.lat_lon_f64().unwrap(), (0., 0.));
    ///
    /// // You can set to None too
    /// node.set_lat_lon(None as Option<(f64, f64)>);
    /// assert!(node.lat_lon_f64().is_none());
    /// ```
    fn set_lat_lon<LL, L1, L2>(&mut self, loc: LL) -> Result<(), ParseLatLonError>
    where
        L1: TryInto<Lat>,
        L2: TryInto<Lon>,
        LL: Into<Option<(L1, L2)>>,
        ParseLatLonError: From<<L1 as TryInto<Lat>>::Error>,
        ParseLatLonError: From<<L2 as TryInto<Lon>>::Error>,
    {
        let ll: Option<(L1, L2)> = loc.into();
        match ll {
            None => self.set_lat_lon_direct(None),
            Some((l1, l2)) => {
                let l1: Lat = l1.try_into()?;
                let l2: Lon = l2.try_into()?;
                self.set_lat_lon_direct(Some((l1, l2)));
            }
        }
        Ok(())
    }
}

/// A Way
pub trait Way: OSMObjBase {
    fn nodes(&self) -> &[ObjId];
    fn num_nodes(&self) -> usize;
    fn node(&self, idx: usize) -> Option<ObjId>;
    fn set_nodes(&mut self, nodes: impl IntoIterator<Item = impl Into<ObjId>>);

    /// A Way which begins and ends with the same Node is considered "closed".
    ///
    /// A closed way that also has an `area=yes` tag should be interpreted as an area.
    ///
    /// This method only compares Node ID's, and does no location based comparison.
    fn is_closed(&self) -> bool {
        match (self.nodes().first(), self.nodes().last()) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// When `is_area` is true, the Way should be interpreted as a 2-D shape rather than a 1-D
    /// linestring.
    fn is_area(&self) -> bool {
        // Generally any closed way represents an area the `area=yes` tag should also be present,
        // but sometimes it's `area=highway` or other things. In the interest of accepting all
        // plausible input, we assume it's an area unless explicitly marked otherwise.
        //
        // See also: https://taginfo.openstreetmap.org/keys/area#values
        self.is_closed() && self.tag("area") != Some("no")
    }
}

/// A Relation
pub trait Relation: OSMObjBase {
    fn members<'a>(
        &'a self,
    ) -> Box<dyn ExactSizeIterator<Item = (OSMObjectType, ObjId, &'a str)> + 'a>;
    fn set_members(
        &mut self,
        members: impl IntoIterator<Item = (OSMObjectType, ObjId, impl Into<String>)>,
    );
}

/// A Node, Way or Relation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OSMObjectType {
    Node,
    Way,
    Relation,
}

impl std::fmt::Debug for OSMObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OSMObjectType::Node => write!(f, "n"),
            OSMObjectType::Way => write!(f, "w"),
            OSMObjectType::Relation => write!(f, "r"),
        }
    }
}

impl std::fmt::Display for OSMObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OSMObjectType::Node => write!(f, "node"),
            OSMObjectType::Way => write!(f, "way"),
            OSMObjectType::Relation => write!(f, "relation"),
        }
    }
}

impl TryFrom<char> for OSMObjectType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'n' => Ok(OSMObjectType::Node),
            'w' => Ok(OSMObjectType::Way),
            'r' => Ok(OSMObjectType::Relation),
            _ => Err(format!("Cannot convert {} to OSMObjectType", c)),
        }
    }
}

impl std::str::FromStr for OSMObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" | "node" => Ok(OSMObjectType::Node),
            "w" | "way" => Ok(OSMObjectType::Way),
            "r" | "relation" | "rel" => Ok(OSMObjectType::Relation),
            _ => Err(format!("Cannot convert {} to OSMObjectType", s)),
        }
    }
}

/// Something which could be any one of the possible OSM objects
pub trait OSMObj: OSMObjBase {
    /// The type of the Node type
    type Node: Node;
    /// The type of the Way type
    type Way: Way;
    /// The type of the Relation type
    type Relation: Relation;

    fn object_type(&self) -> OSMObjectType;

    fn into_node(self) -> Option<Self::Node>;
    fn into_way(self) -> Option<Self::Way>;
    fn into_relation(self) -> Option<Self::Relation>;

    fn as_node(&self) -> Option<&Self::Node>;
    fn as_way(&self) -> Option<&Self::Way>;
    fn as_relation(&self) -> Option<&Self::Relation>;

    fn as_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn as_way_mut(&mut self) -> Option<&mut Self::Way>;
    fn as_relation_mut(&mut self) -> Option<&mut Self::Relation>;

    fn is_node(&self) -> bool {
        self.object_type() == OSMObjectType::Node
    }
    fn is_way(&self) -> bool {
        self.object_type() == OSMObjectType::Way
    }
    fn is_relation(&self) -> bool {
        self.object_type() == OSMObjectType::Relation
    }
}

/// A Generic reader that reads OSM objects
pub trait OSMReader {
    /// The underlying `std::io::Read`.
    type R: Read;
    /// The type of OSM Object that this returns
    type Obj: OSMObj;

    /// Create this reader from a `std::io::Read`.
    fn new(_: Self::R) -> Self;

    #[allow(unused_variables)]
    fn set_sorted_assumption(&mut self, sorted_assumption: bool) {}
    fn get_sorted_assumption(&mut self) -> bool {
        false
    }

    fn assume_sorted(&mut self) {
        self.set_sorted_assumption(true);
    }
    fn assume_unsorted(&mut self) {
        self.set_sorted_assumption(false);
    }

    /// Convert to the underlying reader
    fn into_inner(self) -> Self::R;

    /// Reference to the inner
    fn inner(&self) -> &Self::R;

    /// Returns the next OSM Object in this reader
    fn next(&mut self) -> Option<Self::Obj>;

    /// Returns an iterator over the objects in this reader.
    fn objects<'a>(&'a mut self) -> OSMObjectIterator<'a, Self>
    where
        Self: Sized,
    {
        OSMObjectIterator { inner: self }
    }

    //fn nodes<'a, N: Node>(&'a mut self) -> Box<dyn Iterator<Item=N>+'a> where Self:Sized {
    //    if self.get_sorted_assumption() {
    //        Box::new(self.objects().take_while(|o| o.is_node()).filter_map(|o| o.into_node()))
    //    } else {
    //        Box::new(self.objects().filter_map(|o| o.into_node()))
    //    }
    //}

    //fn nodes_locations<'a>(&'a mut self) -> Box<Iterator<Item=(ObjId, Lat, Lon)>+'a> where Self:Sized {
    //    Box::new(self.nodes().filter_map(|n| if n.deleted || n.lat.is_none() { None } else { Some((n.id, n.lat.unwrap(), n.lon.unwrap())) } ))
    //}

    //fn ways<'a>(&'a mut self) -> Box<Iterator<Item=Way>+'a> where Self:Sized {
    //    if self.get_sorted_assumption() {
    //        Box::new(self.objects().take_while(|o| (o.is_node() || o.is_way())).filter_map(|o| o.into_way()))
    //    } else {
    //        Box::new(self.objects().filter_map(|o| o.into_way()))
    //    }
    //}

    //fn relations<'a>(&'a mut self) -> Box<Iterator<Item=Relation>+'a> where Self:Sized {
    //    Box::new(self.objects().filter_map(|o| o.into_relation()))
    //}
}

/// Something that produces OSMObjects
///
/// Created by `OSMReader::objects`
pub struct OSMObjectIterator<'a, R>
where
    R: OSMReader + 'a,
{
    inner: &'a mut R,
}

impl<'a, R> OSMObjectIterator<'a, R>
where
    R: OSMReader,
{
    pub fn inner(&self) -> &R {
        self.inner
    }

}

impl<'a, R> Iterator for OSMObjectIterator<'a, R>
where
    R: OSMReader,
{
    type Item = R::Obj;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An error when trying to write from an OSMWriter
#[derive(Debug)]
pub enum OSMWriteError {
    FormatDoesntSupportHeaders,
    AlreadyStarted,
    AlreadyClosed,
    OPLWrite(::std::io::Error),
    XMLWriteXMLError(quick_xml::Error),
    XMLWriteIOError(::std::io::Error),
}
impl std::fmt::Display for OSMWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for OSMWriteError {}

/// A generic writer for OSM objects.
pub trait OSMWriter<W: Write> {
    /// Create a writer from an underying writer
    fn new(_: W) -> Self;

    /// Close this writer, cannot write any more objects.
    /// Some fileformats have certain 'end of file' things. After you write those, you cannot write
    /// any more OSM objects. e.g. an XML file format will require that you close your root XML
    /// tag.
    /// After calling this method, you cannot add any more OSM objects to this writer, and
    /// `is_open` will return `false`.
    fn close(&mut self) -> Result<(), OSMWriteError>;

    /// Return true iff this writer is not closed.
    /// If open you should be able to continue to write objects to it. If closed you cannot write
    /// any more OSM objects to it.
    fn is_open(&self) -> bool;

    /// Write an OSM object to this.
    fn write_obj(&mut self, obj: &impl OSMObj) -> Result<(), OSMWriteError>;

    /// Convert back to the underlying writer object
    fn into_inner(self) -> W;

    fn set_header(&mut self, _key_value: (&str, &str)) -> Result<(), OSMWriteError> {
        todo!("set_header not done yet")
    }

    /// Create a new OSMWriter, consume all the objects from an OSMObj iterator source, and then
    /// close this source. Returns this OSMWriter.
    fn from_iter<I: Iterator<Item = impl OSMObj>>(writer: W, iter: I) -> Self
    where
        Self: Sized,
    {
        let mut writer = Self::new(writer);

        // FIXME return the results of these operations?
        for obj in iter {
            writer.write_obj(&obj).unwrap();
        }
        writer.close().unwrap();

        writer
    }
}

/// The version string of this library.
///
/// calls the "CARGO_PKG_VERSION"
pub fn version<'a>() -> &'a str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("unknown-non-cargo-build")
}

/// Opens a PBF filename
pub fn read_pbf(filename: impl AsRef<Path>) -> Result<pbf::PBFReader<BufReader<File>>> {
    Ok(pbf::PBFReader::from_filename(filename)?)
}

/// Opens a bzip2 filename
pub fn read_xml(
    filename: impl AsRef<Path>,
) -> Result<xml::XMLReader<bzip2::read::MultiBzDecoder<std::fs::File>>> {
    Ok(xml::from_filename_bz2(filename)?)
}
