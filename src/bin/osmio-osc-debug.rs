extern crate osmio;

use osmio::osc::OSCReader;
use osmio::OSMReader;
use std::env::args;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let filename = args().nth(1).expect("provide filename as arg 1");
    let io = BufReader::new(File::open(filename).expect("Couldn't open file"));
    let mut osc_reader = OSCReader::new(io);

    for o in osc_reader.elements() {
        dbg!(o);
    }
}
