extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
  if args().len() != 3 {
    eprintln!("Usages: `source` `target`");
    return;
  }

  let mut input: BufReader<File> = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
  let output: File = File::create(args().nth(2).unwrap()).unwrap();
  let mut encoder: GzEncoder<File> = GzEncoder::new(output, Compression::default());
  let start: Instant = Instant::now();
  copy(&mut input, &mut encoder).unwrap();
  let output: File =encoder.finish().unwrap();

  println!("Source len: {:?}kb", input.get_ref().metadata().unwrap().len() / 1024);
  println!("Compressed len:{:?}kb", output.metadata().unwrap().len()/ 1024);
  println!("Duration: {:?}", start.elapsed());
}
