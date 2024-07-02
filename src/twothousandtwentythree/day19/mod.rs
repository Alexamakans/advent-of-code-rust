use std::{collections::HashSet, str::FromStr};

use super::{super::utils::*, YEAR};

// Example input:
// px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// rfg{s<537:gd,x>2440:R,A}
// qs{s>3448:A,lnx}
// qkq{x<1416:A,crn}
// crn{x>2662:A,R}
// in{s<1351:px,qqz}
// qqz{s>2770:qs,m<1801:hdj,R}
// gd{a>3333:R,R}
// hdj{m>838:A,pv}
//
// {x=787,m=2655,a=1222,s=2876}
// {x=1679,m=44,a=2067,s=496}
// {x=2036,m=264,a=79,s=2244}
// {x=2461,m=1339,a=466,s=291}
// {x=2127,m=1623,a=2188,s=1013}

pub struct Solver {}
impl DaySolver<u128> for Solver {
    fn part_one_driver(&self, input: &str) -> u128 {
        let input_parts = input.split_once("\n\n").unwrap();

        let workflows = input_parts
            .0
            .lines()
            .map(|line| Workflow::from_str(line).unwrap())
            .collect::<Vec<_>>();

        let parts = input_parts
            .1
            .lines()
            .map(|line| Part::from_str(line).unwrap());

        // All parts start with the workflow named "in", so we can just run that workflow on each
        // part, moving to the target of the first rule that matches for each part until all parts
        // have reached either target "A" or target "R".
        //
        // The parts are put into lists based on which of "A" or "R" they reach. The answer is the
        // sum of the properties of all parts that reached "A".

        let mut parts_a = Vec::new();
        let mut parts_r = Vec::new();

        for part in parts {
            let mut workflow = workflows
                .iter()
                .find(|workflow| workflow.name == "in")
                .unwrap();

            loop {
                let target = workflow.run(&part);

                if target == "A" {
                    parts_a.push(part);
                    break;
                } else if target == "R" {
                    parts_r.push(part);
                    break;
                }

                workflow = workflows
                    .iter()
                    .find(|workflow| workflow.name == target)
                    .unwrap();
            }
        }

        parts_a
            .iter()
            .map(|part| (part.x + part.m + part.a + part.s) as u128)
            .sum::<u128>()
    }

    fn part_two_driver(&self, input: &str) -> u128 {
        let input_parts = input.split_once("\n\n").unwrap();

        let workflows = input_parts
            .0
            .lines()
            .map(|line| Workflow::from_str(line).unwrap())
            .collect::<Vec<_>>();

        solve_part_two(workflows)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 19)
    }
}

// Part has 4 properties, parsed from one line of the second part of the input.
// The properties are x, m, a, and s. They are all integers.
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(x: i32, m: i32, a: i32, s: i32) -> Part {
        Part { x, m, a, s }
    }
}

impl FromStr for Part {
    type Err = ();

    // Parses a string of the form "{x=787,m=2655,a=1222,s=2876}" into a Part.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::regex::Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}")
            .unwrap()
            .captures(s)
            .map(|cap| {
                Part::new(
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[4].parse().unwrap(),
                )
            })
            .ok_or(())
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_target: String,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>, default_target: String) -> Workflow {
        Workflow {
            name,
            rules,
            default_target,
        }
    }

    fn run(&self, part: &Part) -> String {
        let mut target = self.default_target.clone();

        for rule in &self.rules {
            if rule.matches(&part) {
                target = rule.target.clone();
                break;
            }
        }

        target
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = 0;
        let mut name = String::new();
        let mut rules = Vec::new();
        let mut default_target = String::new();
        let mut rule_str = String::new();

        for c in s.chars() {
            match c {
                '{' => {
                    state = 1;
                }
                ',' => {
                    rules.push(Rule::from_str(&rule_str).unwrap());
                    rule_str.clear();
                }
                '}' => {
                    let last_comma = s.rfind(',').unwrap();
                    default_target = s[last_comma + 1..s.len() - 1].to_string();
                    break;
                }
                _ => match state {
                    0 => name.push(c),
                    1 => {
                        rule_str.push(c);
                    }
                    _ => panic!("Invalid state: {}", state),
                },
            }
        }

        Ok(Workflow::new(name, rules, default_target))
    }
}

struct Rule {
    property: String,
    operator: Operator,
    value: i32,
    target: String,
}

impl Rule {
    fn new(property: String, operator: Operator, value: i32, target: String) -> Rule {
        Rule {
            property,
            operator,
            value,
            target,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        match self.property.as_str() {
            "x" => self.operator.matches(part.x, self.value),
            "m" => self.operator.matches(part.m, self.value),
            "a" => self.operator.matches(part.a, self.value),
            "s" => self.operator.matches(part.s, self.value),
            _ => panic!("Invalid property: {}", self.property),
        }
    }
}

enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    fn from_str(s: &str) -> Operator {
        match s {
            "<" => Operator::LessThan,
            ">" => Operator::GreaterThan,
            _ => panic!("Invalid operator: {}", s),
        }
    }

    fn matches<T: PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Operator::LessThan => a < b,
            Operator::GreaterThan => a > b,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ::regex::Regex::new(r"(\w+)([<>])(\d+):(\w+)")
            .unwrap()
            .captures(s)
            .map(|cap| {
                Rule::new(
                    cap[1].to_string(),
                    Operator::from_str(&cap[2]),
                    cap[3].parse().unwrap(),
                    cap[4].to_string(),
                )
            })
            .ok_or(())
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct PartGroup {
    x_min: i32,
    x_max: i32,
    m_min: i32,
    m_max: i32,
    a_min: i32,
    a_max: i32,
    s_min: i32,
    s_max: i32,
}

impl PartGroup {
    fn new(
        x_min: i32,
        x_max: i32,
        m_min: i32,
        m_max: i32,
        a_min: i32,
        a_max: i32,
        s_min: i32,
        s_max: i32,
    ) -> PartGroup {
        PartGroup {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        }
    }

    // Splits this PartGroup into two PartGroups, based on the given property and value.
    // The property must be one of "x", "m", "a", or "s".
    // The value must be between the min and max of the given property.
    // The two resulting PartGroups will have the same min and max for all properties except the
    // given property, which will be split into two ranges.
    fn split(
        &self,
        property: &String,
        value: i32,
    ) -> Option<(Option<PartGroup>, Option<PartGroup>)> {
        // Assert value is valid for the part, return None if not.
        match property.as_str() {
            "x" => {
                if value < self.x_min || value > self.x_max {
                    return None;
                }
            }
            "m" => {
                if value < self.m_min || value > self.m_max {
                    return None;
                }
            }
            "a" => {
                if value < self.a_min || value > self.a_max {
                    return None;
                }
            }
            "s" => {
                if value < self.s_min || value > self.s_max {
                    return None;
                }
            }
            _ => panic!("Invalid property: {}", property),
        }

        // Create the two new PartGroups.
        let mut left = self.clone();
        let mut right = self.clone();

        let mut left_res = None;
        let mut right_res = None;

        // Set the min and max of the given property for each PartGroup.
        match property.as_str() {
            "x" => {
                left.x_max = value;
                right.x_min = value + 1;

                if left.x_max >= left.x_min {
                    left_res = Some(left);
                }
                if right.x_max >= right.x_min {
                    right_res = Some(right);
                }
                // Set to none if outside valid range of min 1 max 4000 inclusive.
                if left.x_min < 1 || left.x_max > 4000 {
                    left_res = None;
                }
                if right.x_min < 1 || right.x_max > 4000 {
                    right_res = None;
                }
            }
            "m" => {
                left.m_max = value;
                right.m_min = value + 1;

                if left.m_max >= left.m_min {
                    left_res = Some(left);
                }
                if right.m_max >= right.m_min {
                    right_res = Some(right);
                }
                // Set to none if outside valid range of min 1 max 4000 inclusive.
                if left.m_min < 1 || left.m_max > 4000 {
                    left_res = None;
                }
                if right.m_min < 1 || right.m_max > 4000 {
                    right_res = None;
                }
            }
            "a" => {
                left.a_max = value;
                right.a_min = value + 1;

                if left.a_max >= left.a_min {
                    left_res = Some(left);
                }
                if right.a_max >= right.a_min {
                    right_res = Some(right);
                }
                // Set to none if outside valid range of min 1 max 4000 inclusive.
                if left.a_min < 1 || left.a_max > 4000 {
                    left_res = None;
                }
                if right.a_min < 1 || right.a_max > 4000 {
                    right_res = None;
                }
            }
            "s" => {
                left.s_max = value;
                right.s_min = value + 1;

                if left.s_max >= left.s_min {
                    left_res = Some(left);
                }
                if right.s_max >= right.s_min {
                    right_res = Some(right);
                }

                // Set to none if outside valid range of min 1 max 4000 inclusive.
                if left.s_min < 1 || left.s_max > 4000 {
                    left_res = None;
                }
                if right.s_min < 1 || right.s_max > 4000 {
                    right_res = None;
                }
            }
            _ => panic!("Invalid property: {}", property),
        }

        Some((left_res, right_res))
    }
}

fn solve_part_two(workflows: Vec<Workflow>) -> u128 {
    let mut result = 0;

    let part_group = PartGroup::new(1, 4000, 1, 4000, 1, 4000, 1, 4000);
    let mut to_process = vec![(
        workflows.iter().find(|w| w.name == "in").unwrap(),
        part_group,
    )];

    let mut accepted = HashSet::new();

    while let Some(item) = to_process.pop() {
        let (workflow, part_group) = item;

        let mut group = Some(part_group);
        for rule in workflow.rules.iter() {
            if group.is_none() {
                break;
            }

            if let Some(g) = group {
                // Instead of using the matches function, we directly split the groups based on the
                // operator and value of the rule.

                let val = rule.value
                    + match rule.operator {
                        Operator::LessThan => -1,
                        Operator::GreaterThan => 0,
                    };

                let groups = g.split(&rule.property, val);
                if let Some(gs) = groups {
                    match rule.operator {
                        Operator::LessThan => {
                            if let Some(left) = gs.0 {
                                if rule.target == "A" {
                                    accepted.insert(left);
                                } else if rule.target != "R" {
                                    to_process.push((
                                        workflows.iter().find(|w| w.name == rule.target).unwrap(),
                                        left,
                                    ));
                                }
                            }
                            group = gs.1;
                        }
                        Operator::GreaterThan => {
                            if let Some(right) = gs.1 {
                                if rule.target == "A" {
                                    accepted.insert(right);
                                } else if rule.target != "R" {
                                    to_process.push((
                                        workflows.iter().find(|w| w.name == rule.target).unwrap(),
                                        right,
                                    ));
                                }
                            }
                            group = gs.0;
                        }
                    }
                }
            }
        }

        if let Some(g) = group {
            if workflow.default_target == "A" {
                accepted.insert(g);
            } else if workflow.default_target != "R" {
                to_process.push((
                    workflows
                        .iter()
                        .find(|w| w.name == workflow.default_target)
                        .unwrap(),
                    g,
                ));
            }
        }
    }

    // Sum the number of combinations possible per part group.
    for part_group in accepted {
        let x = part_group.x_max - part_group.x_min + 1;
        let m = part_group.m_max - part_group.m_min + 1;
        let a = part_group.a_max - part_group.a_min + 1;
        let s = part_group.s_max - part_group.s_min + 1;

        // Assert they are within a valid range
        assert!(x >= 1 && x <= 4000);
        assert!(m >= 1 && m <= 4000);
        assert!(a >= 1 && a <= 4000);
        assert!(s >= 1 && s <= 4000);

        result += (x * m * a * s) as u128;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_one(), 123);
    }

    #[test]
    fn part_two_works() {
        // let solver = Solver{};
        // let cases = vec![];

        // for case in cases {
        //     assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        // }

        // assert_eq!(solver.part_two(), 123);
    }
}
