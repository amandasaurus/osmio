extern crate osmio;
extern crate separator;

use osmio::pbf::PBFReader;
use osmio::OSMReader;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use separator::FixedPlaceSeparatable;
use separator::Separatable;

fn main() {
    let filename = args().nth(1).expect("provide filename as arg 1");
    let io = BufReader::new(File::open(filename).expect("Couldn't open file"));
    let mut pbf_reader = PBFReader::new(io);

    let start = Instant::now();
    let mut num_objects: u64 = 0;
    let mut last_print = start;

    for _ in pbf_reader.objects() {
        if num_objects % 1_000 == 0 {
            let now = Instant::now();
            let duration = now - last_print;
            if duration.as_millis() > 1_000 {
                let duration_sec = ((now - start).as_millis() as f64) / 1_000.;
                println!(
                    "Processed {} objects so far. Total time {} s. {} objects/sec",
                    num_objects.separated_string(),
                    duration_sec,
                    ((num_objects as f64) / duration_sec).separated_string_with_fixed_place(3)
                );
                last_print = now;
            }
        }
        num_objects += 1;
    }

    let duration_sec = ((Instant::now() - start).as_millis() as f64) / 1_000.;
    println!(
        "\nProcessed {} objects in {} s. {} objects/sec",
        num_objects.separated_string(),
        duration_sec,
        ((num_objects as f64) / duration_sec).separated_string_with_fixed_place(3)
    );
}
