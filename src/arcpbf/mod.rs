//! PBF/Protobuf file format and return ArcOSMObj's
//!
//! Reading PBF files. Writing/creating PBF files is not currently supported or implemented
use super::OSMReader;
use super::ObjId;
use super::TimestampFormat;
use byteorder;
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};
use std::iter::Iterator;
use std::sync::Arc;

use super::*;
use crate::COORD_PRECISION_NANOS;

use flate2::read::ZlibDecoder;

use obj_types::{ArcNode, ArcOSMObj, ArcRelation, ArcWay};

use protobuf;
mod fileformat;
mod osmformat;

struct FileReader<R: Read> {
    reader: R,
}

fn blob_raw_data(blob: &mut fileformat::Blob) -> Option<Vec<u8>> {
    // TODO Shame this can't return a Option<&[u8]>, then I don't need blob to be mut. However I
    // get lifetime errors with bytes not living long enough.
    if blob.has_raw() {
        Some(blob.take_raw())
    } else if blob.has_zlib_data() {
        let zlib_data = blob.get_zlib_data();
        let cursor = Cursor::new(zlib_data);
        let mut bytes = Vec::with_capacity(blob.get_raw_size() as usize);
        ZlibDecoder::new(cursor).read_to_end(&mut bytes).ok()?;

        Some(bytes)
    } else {
        None
    }
}

impl<R: Read> FileReader<R> {
    pub fn new(reader: R) -> Self {
        FileReader { reader }
    }

    pub fn inner(&self) -> &R {
        &self.reader
    }

    pub fn into_inner(self) -> R {
        self.reader
    }

    fn get_next_osmdata_blob(&mut self) -> Option<fileformat::Blob> {
        loop {
            // FIXME is there a way we can ask self.reader if it's at EOF? Rather than waiting for
            // the failure and catching that?
            let size = self.reader.read_u32::<byteorder::BigEndian>().ok()?;
            let mut header_bytes_vec = vec![0; size as usize];

            self.reader
                .read_exact(header_bytes_vec.as_mut_slice())
                .unwrap();

            let blob_header: fileformat::BlobHeader =
                protobuf::parse_from_bytes(&header_bytes_vec).unwrap();

            let mut blob_bytes = vec![0; blob_header.get_datasize() as usize];
            self.reader.read_exact(blob_bytes.as_mut_slice()).unwrap();

            if blob_header.get_field_type() != "OSMData" {
                // keep going to the next blob
                continue;
            }

            let blob: fileformat::Blob = protobuf::parse_from_bytes(&blob_bytes).unwrap();

            return Some(blob);
        }
    }
}

fn decode_nodes(
    _primitive_group: &osmformat::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    _stringtable: &[Option<Arc<str>>],
    _results: &mut [ArcOSMObj],
) {
    unimplemented!("Dense node");
}

fn decode_dense_nodes(
    primitive_group: &osmformat::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    stringtable: &[Option<Arc<str>>],
    results: &mut Vec<ArcOSMObj>,
) {
    let dense = primitive_group.get_dense();
    let ids = dense.get_id();
    let lats = dense.get_lat();
    let lons = dense.get_lon();
    let denseinfo = dense.get_denseinfo();

    let uids = denseinfo.get_uid();
    let changesets = denseinfo.get_changeset();
    let user_sids = denseinfo.get_user_sid();
    let timestamps = denseinfo.get_timestamp();

    let num_nodes = ids.len();
    results.reserve(num_nodes);
    // TODO assert that the id, denseinfo, lat, lon and optionally keys_vals has the same
    // length

    let keys_vals = dense.get_keys_vals();
    let has_tags = !keys_vals.is_empty();

    let mut keys_vals_index = 0;

    // NB it's important that these start at zero, makes the code easier later
    let mut last_id: i64 = 0;
    let mut last_raw_lat: i32 = 0;
    let mut last_raw_lon: i32 = 0;
    let mut last_timestamp = 0;
    let mut last_changset = 0;
    let mut last_uid = 0;
    let mut last_user_sid = 0;

    for index in 0..num_nodes {
        // last_* start off 0
        let id = ids[index] + last_id;
        last_id = id;

        let raw_lat = i32::try_from(lats[index] + last_raw_lat as i64)
            .expect("raw_lat was larger than the OSM precision allows");
        last_raw_lat = raw_lat;

        let raw_lon = i32::try_from(lons[index] + last_raw_lon as i64)
            .expect("raw_lon was larger than OSM precision allows");
        last_raw_lon = raw_lon;

        // granularity is in nanodegrees
        let scale_factor = granularity / COORD_PRECISION_NANOS;
        let mut internal_lat = raw_lat * scale_factor;
        let mut internal_lon = raw_lon * scale_factor;

        // Offsets from pbf are in nanodegrees
        let internal_lat_offset = lat_offset / COORD_PRECISION_NANOS as i64;
        let internal_lon_offset = lon_offset / COORD_PRECISION_NANOS as i64;
        internal_lat += internal_lat_offset as i32;
        internal_lon += internal_lon_offset as i32;

        let tags = if !has_tags {
            None
        } else {
            let mut tags = Vec::new();
            loop {
                //assert!(keys_vals_index <= keys_vals.len());
                let next = keys_vals[keys_vals_index];
                keys_vals_index += 1;
                if next == 0 {
                    break;
                } else {
                    let key = next;
                    let val = keys_vals[keys_vals_index];
                    keys_vals_index += 1;
                    tags.push((key, val));
                }
                // FIXME infinite loop detection maybe?
            }

            Some(
                tags.iter()
                    .map(|&(kidx, vidx)| {
                        (
                            stringtable[kidx as usize].clone(),
                            stringtable[vidx as usize].clone(),
                        )
                    })
                    .filter_map(|(k, v)| match (k, v) {
                        (Some(k), Some(v)) => Some((k, v)),
                        _ => None,
                    })
                    .collect(),
            )
        };

        let changeset_id = changesets[index] + last_changset;
        last_changset = changeset_id;
        let uid_id = uids[index] + last_uid;
        last_uid = uid_id;
        let user_sid = user_sids[index] + last_user_sid;
        last_user_sid = user_sid;
        let timestamp = timestamps[index] as i32 + last_timestamp;
        let timestamp = timestamp * date_granularity;
        last_timestamp = timestamp;
        let timestamp = TimestampFormat::EpochNunber(timestamp as i64);
        assert!(uid_id < i32::MAX);

        results.push(ArcOSMObj::Node(ArcNode {
            _id: id as ObjId,
            _tags: tags,
            _lat_lon: Some((Lat(internal_lat), Lon(internal_lon))),
            _deleted: !denseinfo.get_visible().get(index).unwrap_or(&true),
            _changeset_id: Some(changeset_id as u32),
            _uid: Some(uid_id as u32),
            _user: Some(stringtable[user_sid as usize].clone().unwrap()),
            _version: Some(denseinfo.get_version()[index] as u32),
            _timestamp: Some(timestamp),
        }));
    }

    // convert the keys_vals to
}

fn decode_ways(
    primitive_group: &osmformat::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[Option<Arc<str>>],
    results: &mut Vec<ArcOSMObj>,
) {
    let ways = primitive_group.get_ways();
    results.reserve(ways.len());
    for way in ways {
        let id = way.get_id() as ObjId;
        // TODO check for +itive keys/vals
        let keys = way
            .get_keys()
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = way
            .get_vals()
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let tags = keys.zip(vals);
        let tags: Vec<_> = tags
            .filter_map(|(k, v)| match (k, v) {
                (Some(k), Some(v)) => Some((k, v)),
                _ => None,
            })
            .collect();

        let refs = way.get_refs();
        let mut nodes = Vec::with_capacity(refs.len());
        // TODO assert node.len() > 0
        if !refs.is_empty() {
            let mut last_id = refs[0];
            nodes.push(last_id as ObjId);
            for nid in &refs[1..] {
                last_id += nid;
                nodes.push(last_id as ObjId);
            }
        }

        // TODO assert all node ids are positive

        // TODO could there be *no* info? What should be done there

        //println!("from pbf {} last_timestamp {}", way.get_info().get_timestamp(), last_timestamp);
        //let timestamp = way.get_info().get_timestamp() as i32 + last_timestamp;
        //let timestamp = timestamp * date_granularity;
        //last_timestamp = timestamp;
        //let timestamp = epoch_to_iso(timestamp);
        let timestamp = TimestampFormat::EpochNunber(way.get_info().get_timestamp());

        results.push(ArcOSMObj::Way(ArcWay {
            _id: id,
            _tags: tags,
            _nodes: nodes,
            _deleted: !way.get_info().get_visible(),
            _changeset_id: Some(way.get_info().get_changeset() as u32),
            _uid: Some(way.get_info().get_uid() as u32),
            _user: Some(
                stringtable[way.get_info().get_user_sid() as usize]
                    .clone()
                    .unwrap()
                    .clone(),
            ),
            _version: Some(way.get_info().get_version() as u32),
            _timestamp: Some(timestamp),
        }));
    }
}

fn decode_relations(
    primitive_group: &osmformat::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[Option<Arc<str>>],
    results: &mut Vec<ArcOSMObj>,
) {
    let _last_timestamp = 0;
    for relation in primitive_group.get_relations() {
        let id = relation.get_id() as ObjId;
        // TODO check for +itive keys/vals
        let keys = relation
            .get_keys()
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = relation
            .get_vals()
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let tags = keys.zip(vals);
        let tags: Vec<_> = tags
            .filter_map(|(k, v)| match (k, v) {
                (Some(k), Some(v)) => Some((k, v)),
                _ => None,
            })
            .collect();

        let roles = relation
            .get_roles_sid()
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());

        let refs = relation.get_memids();
        let mut member_ids = Vec::with_capacity(refs.len());
        // TODO assert node.len() > 0
        if !refs.is_empty() {
            let mut last_id = refs[0];
            member_ids.push(last_id as ObjId);
            for nid in &refs[1..] {
                last_id += nid;
                member_ids.push(last_id as ObjId);
            }
        }
        let _num_members = member_ids.len();
        let member_ids = member_ids.iter();

        let member_types = relation.get_types().iter().map(|t| match *t {
            osmformat::Relation_MemberType::NODE => OSMObjectType::Node,
            osmformat::Relation_MemberType::WAY => OSMObjectType::Way,
            osmformat::Relation_MemberType::RELATION => OSMObjectType::Relation,
        });

        let members: Vec<_> = member_types
            .zip(member_ids)
            .zip(roles)
            .filter_map(|((t, &id), r_opt)| r_opt.map(|r| (t, id, r)))
            .collect();

        // TODO could there be *no* info? What should be done there
        //let timestamp = relation.get_info().get_timestamp() as i32 + last_timestamp;
        //let timestamp = timestamp * date_granularity;
        //last_timestamp = timestamp;
        //let timestamp = epoch_to_iso(timestamp);
        let timestamp = TimestampFormat::EpochNunber(relation.get_info().get_timestamp());

        results.push(ArcOSMObj::Relation(ArcRelation {
            _id: id,
            _tags: tags,
            _members: members,
            _deleted: !relation.get_info().get_visible(),
            _changeset_id: Some(relation.get_info().get_changeset() as u32),
            _uid: Some(relation.get_info().get_uid() as u32),
            _user: Some(
                stringtable[relation.get_info().get_user_sid() as usize]
                    .clone()
                    .unwrap(),
            ),
            _version: Some(relation.get_info().get_version() as u32),
            _timestamp: Some(timestamp),
        }));
    }
}

fn decode_primitive_group_to_objs(
    primitive_group: &osmformat::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    stringtable: &[Option<Arc<str>>],
    results: &mut Vec<ArcOSMObj>,
) {
    let date_granularity = date_granularity / 1000;
    if !primitive_group.get_nodes().is_empty() {
        decode_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            results,
        );
    } else if primitive_group.has_dense() {
        decode_dense_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            results,
        );
    } else if !primitive_group.get_ways().is_empty() {
        decode_ways(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            results,
        );
    } else if !primitive_group.get_relations().is_empty() {
        decode_relations(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            results,
        );
    } else {
        unreachable!();
    }
}

fn decode_block_to_objs(mut block: osmformat::PrimitiveBlock) -> Vec<ArcOSMObj> {
    let stringtable: Vec<Option<Arc<str>>> = block
        .take_stringtable()
        .take_s()
        .into_iter()
        .map(|chars| std::str::from_utf8(&chars).ok().map(Arc::from))
        .collect();

    let granularity = block.get_granularity();
    let lat_offset = block.get_lat_offset();
    let lon_offset = block.get_lon_offset();
    let date_granularity = block.get_date_granularity();

    let mut results: Vec<ArcOSMObj> = Vec::new();

    for primitive_group in block.get_primitivegroup() {
        decode_primitive_group_to_objs(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            &mut results,
        );
    }

    results
}

impl<R: Read> Iterator for FileReader<R> {
    type Item = fileformat::Blob;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_osmdata_blob()
    }
}

/// A thing that read PBF files
pub struct PBFReader<R: Read> {
    filereader: FileReader<R>,
    _buffer: Vec<ArcOSMObj>,
    _sorted_assumption: bool,
}

impl PBFReader<BufReader<File>> {
    /// Creates a PBF Reader from a path.
    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self> {
        let filename: &Path = filename.as_ref();
        Ok(Self::new(BufReader::new(File::open(filename)?)))
    }
}

impl<R: Read> OSMReader for PBFReader<R> {
    type R = R;
    type Obj = ArcOSMObj;

    fn new(reader: R) -> PBFReader<R> {
        PBFReader {
            filereader: FileReader::new(reader),
            _buffer: Vec::new(),
            _sorted_assumption: false,
        }
    }

    fn set_sorted_assumption(&mut self, sorted_assumption: bool) {
        self._sorted_assumption = sorted_assumption;
    }
    fn get_sorted_assumption(&mut self) -> bool {
        self._sorted_assumption
    }

    fn inner(&self) -> &R {
        self.filereader.inner()
    }

    fn into_inner(self) -> R {
        self.filereader.into_inner()
    }

    fn next(&mut self) -> Option<ArcOSMObj> {
        while self._buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            let mut blob = self.filereader.next()?;

            let blob_data = blob_raw_data(&mut blob).unwrap();
            let block: osmformat::PrimitiveBlock = protobuf::parse_from_bytes(&blob_data).unwrap();

            // Turn a block into OSM objects
            let mut objs = decode_block_to_objs(block);

            // we reverse the Vec so that we can .pop from the buffer, rather than .remove(0)
            // IME pop'ing is faster, since it means less memory moving
            objs.reverse();

            self._buffer = objs;
        }

        self._buffer.pop()
    }
}
