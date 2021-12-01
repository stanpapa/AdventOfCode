use std::{
    env,
    fs::{self, File},
    io::{
        Error,
        BufRead,
        BufReader,
    },
};

pub fn read_input_as_string() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }

    // read input from file
    let filename = &args[1];
    Ok(fs::read_to_string(filename)?)
}

pub fn read_input_as_ints() -> Result<Vec<i32>, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }

    // read input from file
    let file = File::open(&args[1])
        .expect("File could not be found.");
    let reader = BufReader::new(file);

    let numbers : Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(numbers)
}