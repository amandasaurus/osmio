#![allow(warnings)]
//! PBF/Protobuf file format and return StringOSMObj's
//!
//! Reading PBF files. Writing/creating PBF files is not currently supported or implemented
use super::OSMReader;
use super::ObjId;
use super::TimestampFormat;
use byteorder;
use byteorder::ReadBytesExt;
use std::collections::VecDeque;
use std::io::{Cursor, Read};
use std::iter::Iterator;
use quick_protobuf::{MessageRead, BytesReader};

use super::*;
use crate::COORD_PRECISION_NANOS;

use flate2::read::ZlibDecoder;

use obj_types::{StringNode, StringOSMObj, StringRelation, StringWay};

use protobuf;
mod fileformat;
mod node_id_pos;
mod OSMPBF;
pub use self::node_id_pos::PBFNodePositionReader;
use quick_protobuf::reader::deserialize_from_slice;

type ObjectFilter = (bool, bool, bool);

fn blob_raw_data(
    blob: &mut fileformat::Blob,
    mut buf: &mut Vec<u8>,
) {
    // TODO Shame this can't return a Option<&[u8]>, then I don't need blob to be mut. However I
    // get lifetime errors with bytes not living long enough.
    buf.truncate(0);
    let fileformat::Blob { raw, raw_size, zlib_data, lzma_data } = blob;
    if let Some(raw) = raw {
        buf.reserve(raw.len());
        buf.copy_from_slice(&raw);
    } else if let Some(zlib_data) = zlib_data {
        let cursor = Cursor::new(zlib_data);
        ZlibDecoder::new(cursor).read_to_end(&mut buf).unwrap();
    }
}


fn decode_nodes(
    _primitive_group: OSMPBF::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    _stringtable: &[Option<String>],
    _sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    unimplemented!("Dense node");
}

fn decode_dense_nodes(
    primitive_group: OSMPBF::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    stringtable: &[Option<String>],
    results: &mut VecDeque<StringOSMObj>,
) -> usize {
    let mut num_objects_written = 0;
    let dense = primitive_group.dense.unwrap();
    let ids = dense.id;
    let lats = dense.lat;
    let lons = dense.lon;
    let denseinfo = dense.denseinfo.unwrap();

    let uids = denseinfo.uid;
    let changesets = denseinfo.changeset;
    let user_sids = denseinfo.user_sid;
    let timestamps = denseinfo.timestamp;

    let num_nodes = ids.len();
    results.reserve(num_nodes);
    // TODO assert that the id, denseinfo, lat, lon and optionally keys_vals has the same
    // length

    let keys_vals = dense.keys_vals;
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
        assert!(uid_id < std::i32::MAX);

        results.push_back(StringOSMObj::Node(StringNode {
            _id: id as ObjId,
            _tags: tags,
            _lat_lon: Some((Lat(internal_lat), Lon(internal_lon))),
            _deleted: !denseinfo.visible.get(index).unwrap_or(&true),
            _changeset_id: Some(changeset_id as u32),
            _uid: Some(uid_id as u32),
            _user: Some(stringtable[user_sid as usize].clone().unwrap()),
            _version: Some(denseinfo.version[index] as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1
    }

    num_objects_written
}

fn decode_ways(
    primitive_group: OSMPBF::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[Option<String>],
    results: &mut VecDeque<StringOSMObj>,
) -> usize {
    let mut num_objects_written = 0;
    let ways = primitive_group.ways;
    results.reserve(ways.len());
    for way in ways {
        let id = way.id as ObjId;
        // TODO check for +itive keys/vals
        let keys = way
            .keys
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = way
            .vals
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let tags = keys.zip(vals);
        let tags: Vec<_> = tags
            .filter_map(|(k, v)| match (k, v) {
                (Some(k), Some(v)) => Some((k, v)),
                _ => None,
            })
            .collect();

        let refs = way.refs;
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
        let timestamp = TimestampFormat::EpochNunber(way.info.as_ref().unwrap().timestamp.unwrap());
        let info = way.info.as_ref().unwrap();

        results.push_back(StringOSMObj::Way(StringWay {
            _id: id,
            _tags: tags,
            _nodes: nodes,
            _deleted: !info.visible.unwrap_or(true),
            _changeset_id: Some(info.changeset.unwrap() as u32),
            _uid: Some(info.uid.unwrap() as u32),
            _user: Some(
                stringtable[info.user_sid.unwrap() as usize]
                    .clone()
                    .unwrap()
                    .clone(),
            ),
            _version: Some(info.version as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1;
    }
    num_objects_written
}

fn decode_relations(
    primitive_group: OSMPBF::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[Option<String>],
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let _last_timestamp = 0;
    let mut num_objects_written = 0;
    for relation in primitive_group.relations.into_iter() {
        let id = relation.id as ObjId;
        // TODO check for +itive keys/vals
        let keys = relation
            .keys
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = relation
            .vals
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
            .roles_sid
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());

        let refs = relation.memids;
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

        let member_types = relation.types.iter().map(|t| match *t {
            OSMPBF::mod_Relation::MemberType::NODE => OSMObjectType::Node,
            OSMPBF::mod_Relation::MemberType::WAY => OSMObjectType::Way,
            OSMPBF::mod_Relation::MemberType::RELATION => OSMObjectType::Relation,
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
        let info = relation.info.as_ref().unwrap();
        let timestamp = TimestampFormat::EpochNunber(info.timestamp.unwrap());

        sink.push_back(StringOSMObj::Relation(StringRelation {
            _id: id,
            _tags: tags,
            _members: members,
            _deleted: !info.visible.unwrap(),
            _changeset_id: Some(info.changeset.unwrap() as u32),
            _uid: Some(info.uid.unwrap() as u32),
            _user: Some(
                stringtable[info.user_sid.unwrap() as usize]
                    .clone()
                    .unwrap(),
            ),
            _version: Some(info.version as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1;
    }
    num_objects_written
}

fn decode_primitive_group_to_objs(
    primitive_group: OSMPBF::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    stringtable: &[Option<String>],
    object_filter: &ObjectFilter,
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let date_granularity = date_granularity / 1000;
    let mut num_objects_written = 0;
    if !primitive_group.nodes.is_empty() && object_filter.0 {
        num_objects_written += decode_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            sink,
        );
    } else if primitive_group.dense.is_some() && object_filter.0 {
        num_objects_written += decode_dense_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            sink,
        );
    } else if !primitive_group.ways.is_empty() && object_filter.1 {
        num_objects_written += decode_ways(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            sink,
        );
    } else if !primitive_group.relations.is_empty() && object_filter.2 {
        num_objects_written += decode_relations(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            stringtable,
            sink,
        );
    } else {
        // can happen if there is an object filter in operation
    }

    num_objects_written
}

fn decode_block_to_objs(
    mut block: OSMPBF::PrimitiveBlock,
    object_filter: &ObjectFilter,
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let stringtable: Vec<Option<String>> = block
        .stringtable.s.iter()
        .map(|chars| std::str::from_utf8(&chars).ok().map(String::from))
        .collect();

    let granularity = block.granularity;
    let lat_offset = block.lat_offset;
    let lon_offset = block.lon_offset;
    let date_granularity = block.date_granularity;

    let mut results = 0;

    for primitive_group in block.primitivegroup.into_iter() {
        results += decode_primitive_group_to_objs(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            object_filter,
            sink,
        );
    }

    results
}

/// A thing that read PBF files
pub struct PBFReader<R: Read> {
    reader: R,
    buffer: VecDeque<StringOSMObj>,
    _sorted_assumption: bool,
    object_filter: ObjectFilter,
}

impl<R: Read> PBFReader<R> {
    /// Iterate over all the nodes in this source
    pub fn nodes(&mut self) -> impl Iterator<Item = StringNode> + '_ {
        self.object_filter = (true, false, false);
        self.objects().filter_map(|o| o.into_node())
    }

    /// Iterate over all the ways in this source
    pub fn ways(&mut self) -> impl Iterator<Item = StringWay> + '_ {
        self.object_filter = (false, true, false);
        self.objects().filter_map(|o| o.into_way())
    }

    /// Iterate over all the relations in this source
    pub fn relations(&mut self) -> impl Iterator<Item = StringRelation> + '_ {
        self.object_filter = (false, false, true);
        self.objects().filter_map(|o| o.into_relation())
    }
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
    type Obj = StringOSMObj;

    fn new(reader: R) -> PBFReader<R> {
        PBFReader {
            reader,
            buffer: VecDeque::new(),
            _sorted_assumption: false,
            object_filter: (true, true, true),
        }
    }

    fn set_sorted_assumption(&mut self, sorted_assumption: bool) {
        self._sorted_assumption = sorted_assumption;
    }
    fn get_sorted_assumption(&mut self) -> bool {
        self._sorted_assumption
    }

    fn inner(&self) -> &R {
        &self.reader
    }

    fn into_inner(self) -> R {
        self.reader
    }

    fn next(&mut self) -> Option<StringOSMObj> {
        let mut blob_bytes = Vec::new();
        let mut blob_raw_bytes = Vec::new();
        let mut blob;
        while self.buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            // FIXME is there a way we can ask self.reader if it's at EOF? Rather than waiting for
            // the failure and catching that?


            // read the next blob
            loop {
                let size = self.reader.read_u32::<byteorder::BigEndian>().ok()?;
                let mut header_bytes_vec = vec![0; size as usize];

                self.reader
                    .read_exact(header_bytes_vec.as_mut_slice())
                    .unwrap();

                let mut reader = BytesReader::from_bytes(&header_bytes_vec);
                
                let blob_header = fileformat::BlobHeader::from_reader(&mut reader, &header_bytes_vec).unwrap();

                blob_bytes.resize(blob_header.datasize as usize, 0);
                self.reader.read_exact(blob_bytes.as_mut_slice()).unwrap();

                if blob_header.type_pb != "OSMData" {
                    // keep going to the next blob
                    continue;
                }

                let mut reader = BytesReader::from_bytes(&blob_bytes);

                blob = fileformat::Blob::from_reader(&mut reader, &blob_bytes).unwrap();
                break;
            }

            blob_raw_bytes.truncate(0);
            blob_raw_data(&mut blob, &mut blob_raw_bytes);
            if blob_raw_bytes.is_empty() {
                // maybe the filter meant nothing was read
                continue;
            }
            let mut reader = BytesReader::from_bytes(&blob_raw_bytes);
            let block = OSMPBF::PrimitiveBlock::from_reader(&mut reader, &blob_raw_bytes).unwrap();

            // Turn a block into OSM objects
            decode_block_to_objs(block, &self.object_filter, &mut self.buffer);
        }

        self.buffer.pop_front()
    }
}
