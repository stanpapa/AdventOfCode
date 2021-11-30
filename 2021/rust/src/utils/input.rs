use std::{
    env,
    fs,
    io::Error,
};

pub fn read_input() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }

    // read input from file
    let filename = &args[1];
    Ok(fs::read_to_string(filename)?)
}