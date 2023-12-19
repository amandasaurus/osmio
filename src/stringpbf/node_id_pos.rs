use crate::{Lat, Lon, ObjId};
use std::io::Read;
use std::iter::Iterator;

use std::collections::VecDeque;

use super::*;
use crate::COORD_PRECISION_NANOS;

/// (node id, (latitude, longitude))
type NodeIdPos = (ObjId, (Lat, Lon));

/// Reads a PBF file and returns just (nodeid, pos). This is a little faster than reading the whole
/// file
///
/// ```
/// let mut reader =
/// osmio::stringpbf::PBFNodePositionReader::from_filename("region-latest.osm.pbf")?;
/// let (nid, (lat, lon)) = reader.next().unwrap();
/// ```
pub struct PBFNodePositionReader<R: Read> {
    reader: R,
    buffer: VecDeque<NodeIdPos>,
}

impl<R: Read> PBFNodePositionReader<R> {
    /// Create a new PBFNodePositionReader from this reader
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: VecDeque::new(),
        }
    }

    /// Create a new PBFNodePositionReader from this reader
    pub fn from_reader(reader: R) -> Self {
        Self::new(reader)
    }
}

impl PBFNodePositionReader<BufReader<File>> {
    /// Create a new PBFNodePositionReader for this path
    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self> {
        let filename: &Path = filename.as_ref();
        Ok(Self::new(BufReader::new(File::open(filename)?)))
    }
}

impl<R: Read> Iterator for PBFNodePositionReader<R> {
    type Item = NodeIdPos;

    fn next(&mut self) -> Option<Self::Item> {
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

                let blob_header =
                    fileformat::BlobHeader::from_reader(&mut reader, &header_bytes_vec).unwrap();

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
            decode_block_to_objs(block, &mut self.buffer);
        }

        self.buffer.pop_front()
    }
}

fn decode_block_to_objs(block: OSMPBF::PrimitiveBlock, sink: &mut VecDeque<NodeIdPos>) -> usize {
    let granularity = block.granularity;
    let lat_offset = block.lat_offset;
    let lon_offset = block.lon_offset;
    let mut num_objects = 0;

    for primitive_group in block.primitivegroup.into_iter() {
        if !primitive_group.nodes.is_empty() {
            unimplemented!()
        } else if !primitive_group.ways.is_empty() || !primitive_group.relations.is_empty() {
            continue;
        } else if let Some(dense) = primitive_group.dense {
            let ids = dense.id;
            let lats = dense.lat;
            let lons = dense.lon;

            let num_nodes = ids.len();

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

                sink.push_back((id as ObjId, (Lat(internal_lat), Lon(internal_lon))));
                num_objects += 1;
            }
        } else {
            unreachable!();
        }
    }

    num_objects
}
