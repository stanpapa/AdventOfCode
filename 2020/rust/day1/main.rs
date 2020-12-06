use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn readInput<R: Read>(io: R) -> Result<Vec<i32>, Error> {
  let br = BufReader::new(io);
  br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

fn main() -> Result<(), Error> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }

  // read input from file
  let filename = &args[1];
  let entries = readInput(File::open(filename)?)?;

  // part 1
  for i in 0..=entries.len()-1 {
    for j in i..=entries.len()-1 {
      let sum = entries[i] + entries[j];
      if sum == 2020 {
        let product = entries[i] * entries[j];
        println!("Answer Part 1: {}", product);
      }
    }
  }

  // part 2
  for i in 0..=entries.len()-1 {
    for j in i..=entries.len()-1 {
      let sum_ij = entries[i] + entries[j];
      if sum_ij >= 2020 { continue }
      for k in j..=entries.len()-1 {
        let sum = sum_ij + entries[k];
        if sum == 2020 {
          let product = entries[i] * entries[j] * entries[k];
          println!("Answer Part 2: {}", product);
        }
      }
    }
  }

  Ok(())
} 
