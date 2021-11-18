use std::env;
use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {

  let args: Vec<String> = env::args().collect();

  if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }

  // read input from file
  let filename = &args[1];
  let input = fs::read_to_string(filename)?;

  for line in input.lines() {
    println!("{}", line);
  }

  Ok(())
} 
