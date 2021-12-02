use std::{env, fmt::Debug, fs, io::Error, str::FromStr};

pub fn read_input_as_string() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    Ok(fs::read_to_string(&args[1]).expect("Input file is missing"))
}

// I don't like that I need the std::fmt::Debug here,
// but I can't find a solution for this
pub fn read_input_as_vec<T>() -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Input file is missing");

    Ok(input
        .lines()
        .map(|l| l.parse::<T>().expect("Unable to parse line"))
        .collect())
}
