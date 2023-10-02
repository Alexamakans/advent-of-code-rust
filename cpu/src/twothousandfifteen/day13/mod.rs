// cba researching it but this feels like an optimization problem using doubly linked list with weights on the edges

use std::collections::HashSet;

use crate::scanf;

use super::{super::utils::*, YEAR};

type Interaction = (String, i32, String);

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut interactions = Vec::new();
        let mut people_set = HashSet::new();
        for line in input.lines() {
            let interaction = parse_line(line);
            people_set.insert(interaction.0.clone());
            interactions.push(interaction);
        }

        let people = people_set.into_iter().collect::<Vec<String>>();
        get_optimal_happiness(people, &interactions)
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut interactions = Vec::new();
        let mut people_set = HashSet::new();
        for line in input.lines() {
            let interaction = parse_line(line);
            people_set.insert(interaction.0.clone());
            interactions.push(interaction);
        }
        
        let mut people = people_set.into_iter().collect::<Vec<String>>();
        people.push(String::from("Me"));
        get_optimal_happiness(people, &interactions)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 13)
    }
}

fn parse_line(s: &str) -> Interaction {
    let modified = s
        .replace(" would lose ", ",-")
        .replace(" would gain ", ",")
        .replace(" happiness units by sitting next to ", ",")
        .replace(".", "");
    scanf!(modified, ",", String, i32, String)
}

fn get_optimal_happiness(people: Vec<String>, interactions: &Vec<Interaction>) -> i32 {
    let mut best_happiness = i32::MIN;
    for permutation in people.into_permutations() {
        let first = permutation.first().unwrap();
        let last = permutation.last().unwrap();
        let mut happiness = get_happiness(first, last, &interactions);

        let pairs = permutation.iter().take(permutation.len() - 1).zip(permutation.iter().skip(1));
        for pair in pairs {
            happiness += get_happiness(pair.0, pair.1, &interactions);
        }

        if happiness > best_happiness {
            best_happiness = happiness;
        }
    }

    best_happiness
}

fn get_happiness(
    a: &str,
    b: &str,
    interactions: &Vec<Interaction>,
) -> i32 {
    if a == "Me" || b == "Me" {
        return 0;
    }
    let happiness_a = 'found: {
        for interaction in interactions {
            if interaction.0 == a && interaction.2 == b {
                break 'found interaction.1;
            }
        }
        unreachable!();
    };
    let happiness_b = 'found: {
        for interaction in interactions {
            if interaction.0 == b && interaction.2 == a {
                break 'found interaction.1;
            }
        }
        unreachable!();
    };

    happiness_a + happiness_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![
            (r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.", 330)
        ];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 618);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        assert_eq!(solver.part_two(), 601);
    }
}
