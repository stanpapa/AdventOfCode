use std::{collections::HashMap, io::Error, ops::RangeInclusive, str::FromStr};

use libaoc::io::input::Input;

use rayon::prelude::*;

/// Rule to check if a part meets the requirements
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
            Ok(Self {
                category: ' ',
                condition: ' ',
                value: 0,
                destination: s.to_string(),
            })
        }
    }
}

/// A Workflow is a list of Rules
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

/// construct workflows from input
fn parse_workflows(input: &str) -> HashMap<&str, Workflow> {
    input
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
        .collect()
}

type Part = HashMap<char, usize>;

/// check if a part is accepted
fn is_accepted(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    // starting point in workflows
    let mut label = "in";

    loop {
        // check if a part is either accepted ('A') or rejected ('R')
        if label == "A" {
            return true;
        } else if label == "R" {
            return false;
        }

        // check if a part abides by the rules
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
    let workflows = parse_workflows(input);

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

    // sum all individual values from all accepted parts
    parts
        .par_iter()
        .filter(|part| is_accepted(part, &workflows))
        .map(|part| part.values().sum::<usize>())
        .sum()
}

type PartRange = HashMap<char, RangeInclusive<usize>>;

/// recursive function to find all possible accepted part ranges
fn find_accepted_ranges(
    mut part: PartRange,
    workflows: &HashMap<&str, Workflow>,
    label: String,
    accepted: &mut Vec<PartRange>,
) {
    // check if a part is either accepted ('A') or rejected ('R')
    if label == "A" {
        accepted.push(part.clone());
        return;
    } else if label == "R" {
        return;
    }

    for rule in workflows[label.as_str()].iter() {
        match rule.condition {
            '>' => {
                // partly accepted range
                if part[&rule.category].contains(&rule.value) {
                    // accepted range
                    let mut part_accepted = part.clone();
                    *part_accepted.get_mut(&rule.category).unwrap() =
                        rule.value + 1..=*part[&rule.category].end();

                    find_accepted_ranges(
                        part_accepted,
                        workflows,
                        rule.destination.clone(),
                        accepted,
                    );

                    // rejected range
                    *part.get_mut(&rule.category).unwrap() =
                        *part[&rule.category].start()..=rule.value;
                } else if *part[&rule.category].start() > rule.value {
                    // completely accepted range
                    find_accepted_ranges(
                        part.clone(),
                        workflows,
                        rule.destination.clone(),
                        accepted,
                    );
                }
            }
            '<' => {
                // partly accepted range
                if part[&rule.category].contains(&rule.value) {
                    // accepted range
                    let mut part_accepted = part.clone();
                    *part_accepted.get_mut(&rule.category).unwrap() =
                        *part[&rule.category].start()..=rule.value - 1;

                    find_accepted_ranges(
                        part_accepted,
                        workflows,
                        rule.destination.clone(),
                        accepted,
                    );

                    // rejected range
                    *part.get_mut(&rule.category).unwrap() =
                        rule.value..=*part[&rule.category].end();
                } else if *part[&rule.category].end() < rule.value {
                    // completely accepted range
                    find_accepted_ranges(
                        part.clone(),
                        workflows,
                        rule.destination.clone(),
                        accepted,
                    );
                }
            }
            _ => find_accepted_ranges(part.clone(), workflows, rule.destination.clone(), accepted),
        }
    }
}

fn part_2(input: &str) -> usize {
    let workflows = parse_workflows(input);

    // initial range conditions
    let part_range = vec![
        ('x', 1..=4000),
        ('m', 1..=4000),
        ('a', 1..=4000),
        ('s', 1..=4000),
    ]
    .into_iter()
    .collect::<PartRange>();

    let mut accepted = vec![];
    find_accepted_ranges(part_range, &workflows, "in".to_string(), &mut accepted);

    // calculate the number of distinct combinations of ratings that will be accepted
    accepted
        .par_iter()
        .map(|part_range| {
            part_range
                .values()
                .map(|range| range.clone().count())
                .product::<usize>()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = Input::new().to_string();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));

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

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 167409079868000);
    }
}
