#![allow(warnings)]
use super::OSMReader;
use super::TimestampFormat;
use crate::{Lat, Lon, ObjId};
use byteorder;
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};
use std::iter::Iterator;

use super::*;
use crate::COORD_PRECISION_NANOS;

use flate2::read::ZlibDecoder;

use obj_types::{StringNode, StringOSMObj, StringRelation, StringWay};

type NodeIdPos = (ObjId, (Lat, Lon));

/// Reads a PBF file and returns just (nodeid, pos)
pub struct PBFNodePositionReader<R: Read> {
    filereader: FileReader<R>,
    _buffer: Vec<NodeIdPos>,
}

impl PBFNodePositionReader<BufReader<File>> {
    fn new(reader: BufReader<File>) -> Self {
        Self {
            filereader: FileReader::new(reader),
            _buffer: Vec::new(),
        }
    }

    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self> {
        let filename: &Path = filename.as_ref();
        Ok(Self::new(BufReader::new(File::open(filename)?)))
    }
}

impl<R: Read> Iterator for PBFNodePositionReader<R> {
    type Item = NodeIdPos;

    fn next(&mut self) -> Option<Self::Item> {
        while self._buffer.is_empty() {
            // get the next file block and fill up our buffer
            // FIXME make this parallel

            // get the next block
            let mut blob = self.filereader.next()?;

            let blob_data = blob_raw_data(&mut blob).unwrap();
            let block: osmformat::PrimitiveBlock = protobuf::parse_from_bytes(&blob_data).unwrap();

            // Turn a block into OSM objects
            let mut objs = decode_block_to_objs(block);
            if objs.is_none() {
                continue;
            }
            let mut objs = objs.unwrap();

            // we reverse the Vec so that we can .pop from the buffer, rather than .remove(0)
            // IME pop'ing is faster, since it means less memory moving
            objs.reverse();

            self._buffer = objs;
        }

        self._buffer.pop()
    }
}

fn decode_block_to_objs(mut block: osmformat::PrimitiveBlock) -> Option<Vec<NodeIdPos>> {
    let granularity = block.get_granularity();
    let lat_offset = block.get_lat_offset();
    let lon_offset = block.get_lon_offset();
    let mut results = None;

    for primitive_group in block.get_primitivegroup() {
        if !primitive_group.get_nodes().is_empty() {
            unimplemented!()
        } else if !primitive_group.get_ways().is_empty() {
            continue;
        } else if !primitive_group.get_relations().is_empty() {
            continue;
        } else if primitive_group.has_dense() {
            let dense = primitive_group.get_dense();
            let ids = dense.get_id();
            let lats = dense.get_lat();
            let lons = dense.get_lon();
            let denseinfo = dense.get_denseinfo();

            let num_nodes = ids.len();
            let mut inner_result = Vec::with_capacity(num_nodes);

            // NB it's important that these start at zero, makes the code easier later
            let mut last_id: i64 = 0;
            let mut last_raw_lat: i32 = 0;
            let mut last_raw_lon: i32 = 0;

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

                inner_result.push((id as ObjId, (Lat(internal_lat), Lon(internal_lon))));
            }

            results = Some(inner_result)
        } else {
            unreachable!();
        }
    }

    results
}
