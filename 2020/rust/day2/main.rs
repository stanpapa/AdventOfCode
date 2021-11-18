use std::env;
use std::fs;
use std::io::{Error};


struct PasswordPolicy {
	low: usize,
	high: usize,
	letter: char,
	password: String,
}

impl PasswordPolicy {
	fn new(line: &str) -> PasswordPolicy {
		let l: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();

		PasswordPolicy {
			low: l[0].parse().unwrap(),
			high: l[1].parse().unwrap(),
			letter: l[2].chars().next().unwrap(),
			password: l[3].to_string(),
		}
	}

	fn is_valid_part1(&self) -> bool {
		let count = self.password.matches(self.letter).count();
		count >= self.low && count <= self.high
	}

	fn is_valid_part2(&self) -> bool {
		(self.password.chars().nth(self.low-1).unwrap() == self.letter) ^
		(self.password.chars().nth(self.high-1).unwrap() == self.letter)
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
    	let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");

		let policies: Vec<PasswordPolicy> = input
	    	.lines()
	    	.map(|line| PasswordPolicy::new(line))
	    	.collect();

	    
	    assert_eq!(2, policies
	    	.iter()
	    	.filter(|p| p.is_valid_part1())
	    	.count()
	    );	
    }

    #[test]
    fn part2() {
    	let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");

		let policies: Vec<PasswordPolicy> = input
	    	.lines()
	    	.map(|line| PasswordPolicy::new(line))
	    	.collect();

	    
	    assert_eq!(1, policies
	    	.iter()
	    	.filter(|p| p.is_valid_part2())
	    	.count()
	    );	
    }
}


fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
  
    if args.len() != 2 { panic!("Input file is missing! ABORT.\n"); }
    
    // read input from file
    let filename = &args[1];
    let input = fs::read_to_string(filename)?;

    let policies: Vec<PasswordPolicy> = input
    	.lines()
    	.map(|line| PasswordPolicy::new(line))
    	.collect();

    
    println!("Part1: {}", policies
    	.iter()
    	.filter(|p| p.is_valid_part1())
    	.count()
    );

    println!("Part2: {}", policies
    	.iter()
    	.filter(|p| p.is_valid_part2())
    	.count()
    );

  
    Ok(())
} 
