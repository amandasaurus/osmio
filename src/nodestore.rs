use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::io::{Seek, SeekFrom};
use std::fs;
use std::io::{BufReader, BufWriter};

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
        NodeStoreWriter{ max_node_id: 0, fp: fp }
    }


    pub fn set(&mut self, node_id: u64, lat: f32, lon: f32) {
        if self.max_node_id < node_id {
            //self.fp.set_len(node_id*8);
            self.fp.seek(SeekFrom::End(0)).unwrap();
            for _ in self.max_node_id..node_id {
                self.fp.write_f32::<BigEndian>(200f32).unwrap();
                self.fp.write_f32::<BigEndian>(200f32).unwrap();
            }
            self.max_node_id = node_id;
        }
        self.fp.seek(SeekFrom::Start(node_id*8)).unwrap();
        self.fp.write_f32::<BigEndian>(lat).unwrap();
        self.fp.write_f32::<BigEndian>(lon).unwrap();
    }
}

impl NodeStoreReader {
    pub fn open(filename: &str) -> Self {
        let fp = BufReader::new(fs::File::open(filename).unwrap());
        NodeStoreReader{ fp: fp }
    }

    pub fn get(&mut self, node_id: &u64) -> Option<(f32, f32)> {
        self.fp.seek(SeekFrom::Start(node_id*8)).unwrap();
        let lat = self.fp.read_f32::<BigEndian>().unwrap();
        let lon = self.fp.read_f32::<BigEndian>().unwrap();
        if lat == 200f32 || lon == 200f32 {
            None
        } else {
            Some((lat, lon))
        }
    }
}

