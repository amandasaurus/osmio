//! PBF/Protobuf file format and return StringOSMObj's
//!
//! Reading PBF files. Writing/creating PBF files is not currently supported or implemented
use super::OSMReader;
use super::ObjId;
use super::TimestampFormat;
use byteorder;
use byteorder::ReadBytesExt;
use smallvec::SmallVec;
use smol_str::SmolStr;
use std::collections::VecDeque;
use std::io::{Cursor, Read};
use std::iter::Iterator;
use protobuf::Message;

use super::*;
use crate::COORD_PRECISION_NANOS;

use flate2::read::ZlibDecoder;

use obj_types::{StringNode, StringOSMObj, StringRelation, StringWay};

use protobuf;
mod fileformat;
mod node_id_pos;
mod osmformat;
pub use self::node_id_pos::PBFNodePositionReader;

type ObjectFilter = (bool, bool, bool);

struct FileReader<R: Read> {
    reader: R,
}

fn blob_raw_data(blob: &mut fileformat::Blob, buf: &mut Vec<u8>, _object_filter: &ObjectFilter) {
    // TODO Shame this can't return a Option<&[u8]>, then I don't need blob to be mut. However I
    // get lifetime errors with bytes not living long enough.
    buf.clear();
    if let Some(raw) = &blob.raw {
        buf.reserve(raw.len());
        buf.copy_from_slice(raw.as_slice());
    } else if let Some(zlib_data) = &blob.zlib_data {
        let cursor = Cursor::new(zlib_data);
        ZlibDecoder::new(cursor).read_to_end(buf).unwrap();
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
                fileformat::BlobHeader::parse_from_bytes(&header_bytes_vec).unwrap();

            let mut blob_bytes = vec![0; blob_header.datasize() as usize];
            self.reader.read_exact(blob_bytes.as_mut_slice()).unwrap();

            if blob_header.type_.unwrap() != "OSMData" {
                // keep going to the next blob
                continue;
            }

            let blob: fileformat::Blob = protobuf::Message::parse_from_bytes(&blob_bytes).unwrap();

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
    _stringtable: &[SmolStr],
    _sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    unimplemented!("Dense node");
}

fn decode_dense_nodes(
    primitive_group: &osmformat::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    stringtable: &[SmolStr],
    results: &mut VecDeque<StringOSMObj>,
) -> usize {
    let mut num_objects_written = 0;
    let dense = &primitive_group.dense;
    let ids = &dense.id;
    let lats = &dense.lat;
    let lons = &dense.lon;
    let denseinfo = &dense.denseinfo;

    let uids = &denseinfo.uid;
    let changesets = &denseinfo.changeset;
    let user_sids = &denseinfo.user_sid;
    let timestamps = &denseinfo.timestamp;

    let num_nodes = ids.len();
    results.reserve(num_nodes);
    // TODO assert that the id, denseinfo, lat, lon and optionally keys_vals has the same
    // length

    let keys_vals = &dense.keys_vals;

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

        let mut tags: SmallVec<[(SmolStr, SmolStr); 1]> = SmallVec::new();
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
                tags.push((
                    stringtable[key as usize].clone(),
                    stringtable[val as usize].clone(),
                ));
            }
            // FIXME infinite loop detection maybe?
        }

        let changeset_id = changesets[index] + last_changset;
        last_changset = changeset_id;
        let uid_id = uids[index] + last_uid;
        last_uid = uid_id;
        let user_sid = user_sids[index] + last_user_sid;
        last_user_sid = user_sid;
        let timestamp = timestamps[index] as i32 + last_timestamp;
        let timestamp = timestamp * date_granularity;
        last_timestamp = timestamp;
        let timestamp = TimestampFormat::EpochNumber(timestamp as i64);
        assert!(uid_id < i32::MAX);

        results.push_back(StringOSMObj::Node(StringNode {
            _id: id as ObjId,
            _tags: tags,
            _lat_lon: Some((Lat(internal_lat), Lon(internal_lon))),
            _deleted: !denseinfo.visible.get(index).unwrap_or(&true),
            _changeset_id: Some(changeset_id as u32),
            _uid: Some(uid_id as u32),
            _user: Some(stringtable[user_sid as usize].clone()),
            _version: Some(denseinfo.version[index] as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1
    }

    num_objects_written
}

fn decode_ways(
    primitive_group: &osmformat::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[SmolStr],
    results: &mut VecDeque<StringOSMObj>,
) -> usize {
    let mut num_objects_written = 0;
    results.reserve(primitive_group.ways.len());
    for way in primitive_group.ways.iter() {
        let id = way.id.unwrap() as ObjId;
        let mut tags = SmallVec::with_capacity(way.keys.len());
        // TODO check for +itive keys/vals
        let keys = way
            .keys
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = way.vals
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        assert_eq!(keys.len(), vals.len());
        tags.extend(keys.zip(vals));

        let refs = &way.refs;
        let mut nodes = SmallVec::with_capacity(refs.len());
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
        let timestamp = TimestampFormat::EpochNumber(way.info.timestamp.unwrap());

        results.push_back(StringOSMObj::Way(StringWay {
            _id: id,
            _tags: tags,
            _nodes: nodes,
            _deleted: !way.info.visible.unwrap_or(false),
            _changeset_id: Some(way.info.changeset.unwrap() as u32),
            _uid: Some(way.info.uid.unwrap() as u32),
            _user: Some(stringtable[way.info.user_sid.unwrap() as usize].clone()),
            _version: Some(way.info.version.unwrap() as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1;
    }
    num_objects_written
}

fn decode_relations(
    primitive_group: &osmformat::PrimitiveGroup,
    _granularity: i32,
    _lat_offset: i64,
    _lon_offset: i64,
    _date_granularity: i32,
    stringtable: &[SmolStr],
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let _last_timestamp = 0;
    let mut num_objects_written = 0;
    sink.reserve(primitive_group.relations.len());
    for relation in primitive_group.relations.iter() {
        let id = relation.id() as ObjId;
        // TODO check for +itive keys/vals
        let keys = relation
            .keys
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        let vals = relation
            .vals
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());
        assert_eq!(keys.len(), vals.len());
        let tags: SmallVec<_> = keys.zip(vals).collect();

        let roles = relation
            .roles_sid
            .iter()
            .map(|&idx| stringtable[idx as usize].clone());

        let refs = &relation.memids;
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

        let member_types = relation.types.iter().map(::protobuf::EnumOrUnknown::unwrap).map(|t| match t {
            osmformat::relation::MemberType::NODE => OSMObjectType::Node,
            osmformat::relation::MemberType::WAY => OSMObjectType::Way,
            osmformat::relation::MemberType::RELATION => OSMObjectType::Relation,
        });

        let members: Vec<_> = member_types
            .zip(member_ids)
            .zip(roles)
            .map(|((t, &id), r)| (t, id, r.clone()))
            .collect();

        // TODO could there be *no* info? What should be done there
        //let timestamp = relation.get_info().get_timestamp() as i32 + last_timestamp;
        //let timestamp = timestamp * date_granularity;
        //last_timestamp = timestamp;
        //let timestamp = epoch_to_iso(timestamp);
        let timestamp = TimestampFormat::EpochNumber(relation.info.timestamp.unwrap());

        sink.push_back(StringOSMObj::Relation(StringRelation {
            _id: id,
            _tags: tags,
            _members: members,
            _deleted: !relation.info.visible.unwrap(),
            _changeset_id: Some(relation.info.changeset.unwrap() as u32),
            _uid: Some(relation.info.uid.unwrap() as u32),
            _user: Some(stringtable[relation.info.user_sid.unwrap() as usize].clone()),
            _version: Some(relation.info.version.unwrap() as u32),
            _timestamp: Some(timestamp),
        }));
        num_objects_written += 1;
    }
    num_objects_written
}

#[allow(clippy::too_many_arguments)]
fn decode_primitive_group_to_objs(
    primitive_group: &osmformat::PrimitiveGroup,
    granularity: i32,
    lat_offset: i64,
    lon_offset: i64,
    date_granularity: i32,
    mut raw_stringtable: osmformat::StringTable,
    object_filter: &ObjectFilter,
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let date_granularity = date_granularity / 1000;
    let mut num_objects_written = 0;
    if !primitive_group.nodes.is_empty() && object_filter.0 {
        let mut stringtable: Vec<SmolStr> =
            Vec::with_capacity(raw_stringtable.s.len());
        stringtable.extend(raw_stringtable.s.iter().map(|chars| {
            SmolStr::from(String::from_utf8(chars).expect("Invalid, non-utf8 String"))
        }));

        num_objects_written += decode_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            sink,
        );
    } else if primitive_group.dense.is_some() && object_filter.0 {
        let mut stringtable: Vec<SmolStr> =
            Vec::with_capacity(raw_stringtable.s.len());
        stringtable.extend(raw_stringtable.take_s().into_iter().map(|chars| {
            SmolStr::from(String::from_utf8(chars).expect("Invalid, non-utf8 String"))
        }));

        num_objects_written += decode_dense_nodes(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            sink,
        );
    } else if !primitive_group.get_ways().is_empty() && object_filter.1 {
        let mut stringtable: Vec<SmolStr> =
            Vec::with_capacity(raw_stringtable.get_s().iter().count());
        stringtable.extend(raw_stringtable.take_s().into_iter().map(|chars| {
            SmolStr::from(String::from_utf8(chars).expect("Invalid, non-utf8 String"))
        }));

        num_objects_written += decode_ways(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            sink,
        );
    } else if !primitive_group.get_relations().is_empty() && object_filter.2 {
        let mut stringtable: Vec<SmolStr> =
            Vec::with_capacity(raw_stringtable.get_s().iter().count());
        stringtable.extend(raw_stringtable.take_s().into_iter().map(|chars| {
            SmolStr::from(String::from_utf8(chars).expect("Invalid, non-utf8 String"))
        }));

        num_objects_written += decode_relations(
            primitive_group,
            granularity,
            lat_offset,
            lon_offset,
            date_granularity,
            &stringtable,
            sink,
        );
    } else {
        // can happen if there is an object filter in operation
    }

    num_objects_written
}

fn decode_block_to_objs(
    mut block: osmformat::PrimitiveBlock,
    object_filter: &ObjectFilter,
    sink: &mut VecDeque<StringOSMObj>,
) -> usize {
    let raw_stringtable = block.take_stringtable();

    let granularity = block.granularity();
    let lat_offset = block.lat_offset();
    let lon_offset = block.lon_offset();
    let date_granularity = block.date_granularity();

    let mut results = 0;

    assert_eq!(block.primitivegroup().len(), 1);
    results += decode_primitive_group_to_objs(
        &block.primitivegroup()[0],
        granularity,
        lat_offset,
        lon_offset,
        date_granularity,
        raw_stringtable,
        object_filter,
        sink,
    );

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
            filereader: FileReader::new(reader),
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
        self.filereader.inner()
    }

    fn into_inner(self) -> R {
        self.filereader.into_inner()
    }

    fn next(&mut self) -> Option<StringOSMObj> {
        let mut blob_data = Vec::new();
        while self.buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            let mut blob = self.filereader.next()?;

            blob_data.clear();
            blob_raw_data(&mut blob, &mut blob_data, &self.object_filter);
            if blob_data.is_empty() {
                // maybe the filter meant nothing was read
                continue;
            }
            let block: osmformat::PrimitiveBlock = osmformat::PrimitiveBlock::parse_from(&blob_data).unwrap();

            // Turn a block into OSM objects
            decode_block_to_objs(block, &self.object_filter, &mut self.buffer);
        }

        self.buffer.pop_front()
    }
}
