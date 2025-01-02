extern crate flate2;

use flate2::{write::GzEncoder, Compression};
use std::{env, fs, io::{self}, time};

fn main () {
    if env::args().len() != 3 {
        eprintln!("Usage:<input> <output>");
        return;
    }

    let mut input = io::BufReader::new(fs::File::open(env::args().nth(1).unwrap()).unwrap());
    let output = fs::File::create(env::args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = time::Instant::now();
    io::copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    let duration = start.elapsed();

    println!(
        "Compressed {} bytes to {} bytes in {:?}", 
        input.get_ref().metadata().unwrap().len(), 
        output.metadata().unwrap().len(),
        duration
    );
}
