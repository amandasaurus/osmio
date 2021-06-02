use crate::{Lat, Lon};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs;
use std::io::{BufReader, BufWriter};
use std::io::{Seek, SeekFrom};

pub struct NodeStoreWriter {
    max_node_id: u64,
    fp: BufWriter<fs::File>,
}

pub struct NodeStoreReader {
    fp: BufReader<fs::File>,
}

impl NodeStoreWriter {
    pub fn create(filename: &str) -> Self {
        let fp = BufWriter::with_capacity(1_000_000_000, fs::File::create(filename).unwrap());
        NodeStoreWriter {
            max_node_id: 0,
            fp: fp,
        }
    }

    pub fn set(&mut self, node_id: u64, lat: Lat, lon: Lon) {
        if self.max_node_id < node_id {
            //self.fp.set_len(node_id*8);
            self.fp.seek(SeekFrom::End(0)).unwrap();
            for _ in self.max_node_id..node_id {
                self.fp
                    .write_i32::<BigEndian>(SENTINAL_VALUE_FOR_EMPTY)
                    .unwrap();
                self.fp
                    .write_i32::<BigEndian>(SENTINAL_VALUE_FOR_EMPTY)
                    .unwrap();
            }
            self.max_node_id = node_id;
        }
        self.fp.seek(SeekFrom::Start(node_id * 8)).unwrap();
        self.fp.write_i32::<BigEndian>(lat.inner()).unwrap();
        self.fp.write_i32::<BigEndian>(lon.inner()).unwrap();
    }
}

const SENTINAL_VALUE_FOR_EMPTY: i32 = i32::MAX;

impl NodeStoreReader {
    pub fn open(filename: &str) -> Self {
        let fp = BufReader::new(fs::File::open(filename).unwrap());
        NodeStoreReader { fp: fp }
    }

    pub fn get(&mut self, node_id: &u64) -> Option<(Lat, Lon)> {
        self.fp.seek(SeekFrom::Start(node_id * 8)).unwrap();
        let lat = self.fp.read_i32::<BigEndian>().unwrap();
        let lon = self.fp.read_i32::<BigEndian>().unwrap();
        if lat == SENTINAL_VALUE_FOR_EMPTY || lon == SENTINAL_VALUE_FOR_EMPTY {
            None
        } else {
            Some((Lat::from_inner(lat), Lon::from_inner(lon)))
        }
    }
}
