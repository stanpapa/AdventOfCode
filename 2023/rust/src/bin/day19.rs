use std::{collections::HashMap, io::Error, str::FromStr};

use libaoc::io::input::Input;

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    category: char,  // x, m, a, s
    condition: char, // >, <
    value: usize,
    destination: String,
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((c, destination)) = s.split_once(':') {
            if let Some((category, value)) = c.split_once('>') {
                Ok(Self {
                    category: category.parse::<char>().unwrap(),
                    condition: '>',
                    value: value.parse::<usize>().unwrap(),
                    destination: destination.to_string(),
                })
            } else if let Some((category, value)) = c.split_once('<') {
                Ok(Self {
                    category: category.parse::<char>().unwrap(),
                    condition: '<',
                    value: value.parse::<usize>().unwrap(),
                    destination: destination.to_string(),
                })
            } else {
                Err("No comparison operator found")
            }
        } else {
            // Err("No \':\' found")
            Ok(Self {
                category: ' ',
                condition: ' ',
                value: 0,
                destination: s.to_string(),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow {
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rules: s.split(',').map(|rule| rule.parse().unwrap()).collect(),
        })
    }
}

impl Workflow {
    fn iter(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
}

type Part = HashMap<char, usize>;

fn is_accepted(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut label = "in";

    loop {
        if label == "A" {
            return true;
        } else if label == "R" {
            return false;
        }

        for rule in workflows[label].iter() {
            match rule.condition {
                '>' => {
                    if part[&rule.category] > rule.value {
                        label = &rule.destination;
                        break;
                    }
                }
                '<' => {
                    if part[&rule.category] < rule.value {
                        label = &rule.destination;
                        break;
                    }
                }
                _ => label = &rule.destination,
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    let workflows: HashMap<&str, Workflow> = input
        .lines()
        .take_while(|&line| !line.trim().is_empty())
        .map(|line| {
            let (label, workflows) = line.split_once('{').unwrap();
            (
                label,
                workflows[..workflows.len() - 1]
                    .parse::<Workflow>()
                    .unwrap(),
            )
        })
        .collect();

    let parts: Vec<Part> = input
        .lines()
        .skip_while(|line| line.trim().is_empty() || line.chars().next().unwrap().is_alphabetic())
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|rating| {
                    let (label, value) = rating.split_once('=').unwrap();
                    (
                        label.parse::<char>().unwrap(),
                        value.parse::<usize>().unwrap(),
                    )
                })
                .collect::<HashMap<char, usize>>()
        })
        .collect();

    parts
        .par_iter()
        .filter(|part| is_accepted(part, &workflows))
        .map(|part| part.values().sum::<usize>())
        .sum()
}

fn main() -> Result<(), Error> {
    let input = Input::new().to_string();

    println!("{}", part_1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Rule, Workflow};

    const INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn rule() {
        assert_eq!(
            "a<2006:qkq".parse::<Rule>().unwrap(),
            Rule {
                category: 'a',
                condition: '<',
                value: 2006,
                destination: "qkq".to_string()
            }
        );
    }

    #[test]
    fn workflow() {
        assert_eq!(
            "a<2006:qkq,m>2090:A,rfg".parse::<Workflow>().unwrap(),
            Workflow {
                rules: vec![
                    Rule {
                        category: 'a',
                        condition: '<',
                        value: 2006,
                        destination: "qkq".to_string()
                    },
                    Rule {
                        category: 'm',
                        condition: '>',
                        value: 2090,
                        destination: "A".to_string()
                    },
                    Rule {
                        category: ' ',
                        condition: ' ',
                        value: 0,
                        destination: "rfg".to_string()
                    },
                ]
            }
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 19114);
    }
}
