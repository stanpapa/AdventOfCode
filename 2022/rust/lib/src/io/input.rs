use std::{fmt::Debug, fs, io::Error, str::FromStr};

pub struct Input {
//    filename: String,
    data: String,
}

impl Input {
    pub fn new(filename: &str) -> Input {
        let data = fs::read_to_string(filename).expect("Input file is missing!");
        Input {
 //           filename: String::from(filename),
            data
        }
    }
    
    pub fn to_string(&self) -> String {
        self.data.clone()
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
