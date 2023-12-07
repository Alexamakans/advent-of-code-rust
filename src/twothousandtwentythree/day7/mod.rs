use std::{cmp::Ordering, str::FromStr};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        solve_part_one(input)
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        solve_part_two(input)
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 7)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ParseCardError;
impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(ParseCardError {}),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Combo {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_combo(hand: &Vec<Card>) -> Combo {
    let mut counts = Vec::new();
    let cards = vec![
        Card::A,
        Card::K,
        Card::Q,
        Card::J,
        Card::T,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two,
    ];

    for card in cards {
        let count = hand.iter().filter(|&e| *e == card).count();
        if count > 0 {
            counts.push(count);
        }
    }

    counts.sort();
    if *counts.last().unwrap() == 5 {
        return Combo::FiveOfAKind;
    } else if *counts.last().unwrap() == 4 {
        return Combo::FourOfAKind;
    } else if *counts.last().unwrap() == 3 && *counts.first().unwrap() == 2 {
        return Combo::FullHouse;
    } else if *counts.last().unwrap() == 3 {
        return Combo::ThreeOfAKind;
    } else if *counts.last().unwrap() == 2 && *counts.iter().skip(1).next().unwrap() == 2 {
        return Combo::TwoPair;
    } else if *counts.last().unwrap() == 2 {
        return Combo::OnePair;
    } else if counts.len() == 5 {
        return Combo::HighCard;
    }

    panic!("unhandled combo case for {:#?},   {:#?}", hand, counts);
}
fn get_combo_part_two(hand: &Vec<Card>) -> Combo {
    let mut counts = Vec::new();
    let cards = vec![
        Card::A,
        Card::K,
        Card::Q,
        Card::T,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two,
    ];

    let num_jokers = hand.iter().filter(|&e| *e == Card::J).count();
    for card in cards.clone() {
        let count = hand.iter().filter(|&e| *e == card).count();
        if count > 0 {
            counts.push(count);
        }
    }

    counts.sort();
    if num_jokers > 0 {
        if counts.len() == 0 {
            counts.push(0);
        }
        *counts.last_mut().unwrap() += num_jokers;
    }
    if *counts.last().unwrap() == 5 {
        return Combo::FiveOfAKind;
    } else if *counts.last().unwrap() == 4 {
        return Combo::FourOfAKind;
    } else if *counts.last().unwrap() == 3 && *counts.first().unwrap() == 2 {
        return Combo::FullHouse;
    } else if *counts.last().unwrap() == 3 {
        return Combo::ThreeOfAKind;
    } else if *counts.last().unwrap() == 2 && *counts.iter().skip(1).next().unwrap() == 2 {
        return Combo::TwoPair;
    } else if *counts.last().unwrap() == 2 {
        return Combo::OnePair;
    } else if counts.len() == 5 {
        return Combo::HighCard;
    }

    panic!("unhandled combo case for {:#?},   {:#?}", hand, counts);
}

fn get_high_card_winner(a: &Vec<Card>, b: &Vec<Card>) -> i32 {
    for i in 0..5 {
        let ac = a.get(i).unwrap();
        let bc = b.get(i).unwrap();

        if ac > bc {
            return 0;
        } else if ac < bc {
            return 1;
        }
    }

    panic!("identical hands {:#?} and {:#?}", a, b);
}

fn get_high_card_winner_part_two(a: &Vec<Card>, b: &Vec<Card>) -> i32 {
    for i in 0..5 {
        let ac = *a.get(i).unwrap();
        let bc = *b.get(i).unwrap();

        if ac == Card::J && bc != Card::J {
            return 1;
        }

        if ac != Card::J && bc == Card::J {
            return 0;
        }

        if ac > bc {
            return 0;
        } else if ac < bc {
            return 1;
        }
    }

    panic!("identical hands {:#?} and {:#?}", a, b);
}
fn parse_input(s: &str) -> Vec<(Vec<Card>, i32)> {
    let mut result = Vec::new();
    for line in s.lines() {
        let mut parts = line.split_whitespace();
        let (cards, bid) = (
            parts
                .next()
                .unwrap()
                .chars()
                .map(|e| e.to_string().parse::<Card>().unwrap()),
            parts.next().unwrap().parse::<i32>().unwrap(),
        );
        let hand = cards.collect::<Vec<Card>>();

        result.push((hand, bid));
    }

    result
}

fn solve_part_one(s: &str) -> i32 {
    let a = parse_input(s);
    let ranks = sort_by_rank(a);

    let mut result = 0;
    for (index, hand) in ranks.iter().enumerate() {
        result += (index as i32 + 1) * hand.1;
    }

    result
}

fn solve_part_two(s: &str) -> i32 {
    let a = parse_input(s);
    let ranks = sort_by_rank_part_two(a);

    let mut result = 0;
    for (index, hand) in ranks.iter().enumerate() {
        result += (index as i32 + 1) * hand.1;
    }

    result
}
fn sort_by_rank_part_two(mut hands: Vec<(Vec<Card>, i32)>) -> Vec<(Vec<Card>, i32)> {
    hands.sort_by(|a, b| -> Ordering {
        let acombo = get_combo_part_two(&a.0);
        let bcombo = get_combo_part_two(&b.0);
        if acombo > bcombo {
            Ordering::Greater
        } else if acombo < bcombo {
            Ordering::Less
        } else {
            let high_card_winner = get_high_card_winner_part_two(&a.0, &b.0);
            if high_card_winner == 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });

    hands
}
fn sort_by_rank(mut hands: Vec<(Vec<Card>, i32)>) -> Vec<(Vec<Card>, i32)> {
    hands.sort_by(|a, b| -> Ordering {
        let acombo = get_combo(&a.0);
        let bcombo = get_combo(&b.0);
        if acombo > bcombo {
            Ordering::Greater
        } else if acombo < bcombo {
            Ordering::Less
        } else {
            let high_card_winner = get_high_card_winner(&a.0, &b.0);
            if high_card_winner == 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });

    hands
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            253933213,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }
        assert_eq!(solver.part_one(), 123);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            5905,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }
        assert_eq!(solver.part_two(), 253473930);
    }

    #[test]
    fn get_combo_works() {
        let solver = Solver {};
        let cases = vec![
            ("32T3K 1", Combo::OnePair),
            ("T55J5 1", Combo::ThreeOfAKind),
            ("KK677 1", Combo::TwoPair),
            ("KTJJT 1", Combo::TwoPair),
            ("QQQJA 1", Combo::ThreeOfAKind),
            ("22222 1", Combo::FiveOfAKind),
            ("22223 1", Combo::FourOfAKind),
            ("22233 1", Combo::FullHouse),
        ];

        for case in cases {
            let hands = parse_input(case.0);
            let combo = hands
                .iter()
                .map(|e| get_combo(&e.0))
                .collect::<Vec<Combo>>();
            let got_combo = combo.first().unwrap().to_owned();
            assert_eq!(combo.len(), 1);
            assert_eq!(got_combo, case.1, "input = {:#?}", case.0);
        }

        // assert_eq!(solver.part_one(), 123);
    }

    #[test]
    fn sort_combo_works() {
        let mut combos = vec![Combo::OnePair, Combo::FiveOfAKind, Combo::ThreeOfAKind];

        let want = vec![Combo::FiveOfAKind, Combo::ThreeOfAKind, Combo::OnePair];

        combos.sort_by(|acombo, bcombo| -> Ordering {
            if acombo > bcombo {
                Ordering::Less
            } else if acombo < bcombo {
                Ordering::Greater
            } else {
                unreachable!();
            }
        });
        assert_eq!(combos, want);
    }

    #[test]
    fn get_high_card_winner_works() {
        let a = vec![Card::A, Card::Nine, Card::Three];
        let b = vec![Card::Q, Card::Nine, Card::Three];

        assert_eq!(get_high_card_winner(&a, &b), 0);
        assert_eq!(get_high_card_winner(&b, &a), 1);
    }

    #[test]
    fn combo_orderings() {
        assert!(Combo::HighCard < Combo::OnePair);
        assert!(Combo::HighCard < Combo::TwoPair);
        assert!(Combo::HighCard < Combo::ThreeOfAKind);
        assert!(Combo::HighCard < Combo::FullHouse);
        assert!(Combo::HighCard < Combo::FourOfAKind);
        assert!(Combo::HighCard < Combo::FiveOfAKind);

        assert!(Combo::OnePair > Combo::HighCard);
        assert!(Combo::OnePair < Combo::TwoPair);
        assert!(Combo::OnePair < Combo::ThreeOfAKind);
        assert!(Combo::OnePair < Combo::FullHouse);
        assert!(Combo::OnePair < Combo::FourOfAKind);
        assert!(Combo::OnePair < Combo::FiveOfAKind);
    }
}
