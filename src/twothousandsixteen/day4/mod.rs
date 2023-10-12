use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut sector_id_sum = 0;
        for line in input.lines() {
            let room = Room::from(line);
            if room.is_valid() {
                sector_id_sum += room.sector_id;
            }
        }
        sector_id_sum
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        for line in input.lines() {
            let room = Room::from(line);
            if room.is_valid()
                && room
                    .decrypted_name()
                    .starts_with("northpole-object-storage")
            {
                return room.sector_id;
            }
        }
        unreachable!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 4)
    }
}

struct Room<'a> {
    raw_string: &'a str,
    sector_id: i32,
    letters: Vec<char>,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn is_valid(&self) -> bool {
        let mut letter_occurrences = extract_repeating_sequences(self.letters.iter().cloned())
            .into_iter()
            .map(|e| (e.0 .1 - e.0 .0, e.1))
            .collect::<Vec<(usize, char)>>();
        letter_occurrences.sort_by(|a, b| match b.0.cmp(&a.0) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        });

        let calculated_checksum = letter_occurrences
            .into_iter()
            .take(5)
            .map(|e| e.1)
            .collect::<String>();
        self.checksum == calculated_checksum
    }

    fn decrypted_name(&self) -> String {
        caesar_cipher(&self.raw_string, self.sector_id as usize)
    }
}

impl<'a> From<&'a str> for Room<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts = value.rsplit(&['-']);
        let (sector_id, mut checksum) = parts.next().unwrap().split_once(&['[']).unwrap();
        checksum = checksum.get(..checksum.len() - 1).unwrap();
        let mut letters = parts.map(|e| e.chars()).flatten().collect::<Vec<char>>();
        letters.sort();
        Room {
            raw_string: value,
            sector_id: sector_id.parse::<i32>().unwrap(),
            letters,
            checksum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![("aaaaa-bbb-z-y-x-123[abxyz]", true)];

        for case in cases {
            let room = Room::from(case.0);
            assert_eq!(room.is_valid(), true, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 361724);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        assert_eq!(solver.part_two(), 482);
    }
}
