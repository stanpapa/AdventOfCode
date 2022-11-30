use std::{env, fmt, fmt::Debug, fs, io::Error, str::FromStr};

pub struct Input {
    data: String,
}

impl Input {
    /// read input from text file, which is always
    /// the first command-line argument and
    /// store as string
    pub fn new() -> Input {
        let args: Vec<String> = env::args().collect();
        let data = fs::read_to_string(&args[1]).expect("Input file is missing!");
        Input {
            data
        }
    }
    
    pub fn to_vec<T>(&self) -> Result<Vec<T>, Error>
    where 
        T: FromStr,
        <T as FromStr>::Err: Debug
    {
        Ok(self.data
            .lines()
            .map(|l| l.parse::<T>().expect("Unable to parse line"))
            .collect())
    }
}

/// also gives access to to_string()
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}