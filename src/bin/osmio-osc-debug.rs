extern crate osmio;

use osmio::osc::OSCReader;
use osmio::OSMReader;
use std::env::args;
use std::io::{BufReader};
use std::fs::File;

fn main() {
    let filename = args().nth(1).expect("provide filename as arg 1");
    let io =  BufReader::new(File::open(filename).expect("Couldn't open file"));
    let mut osc_reader = OSCReader::new(io);

    for o in osc_reader.objects() {
        dbg!(o);
    }


}
