use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<String> for Solver {
    fn part_one_driver(&self, input: &str) -> String {
        let mut i = 0;
        let mut password = String::new();
        loop {
            let h = md5::calculate_hash_bytes(format!("{}{}", input, i).into_bytes());
            if h[0] == 0 && h[1] == 0 && h[2] & 0xF0 == 0 {
                password.push(format!("{:01x}", h[2] & 0xF).chars().nth(0).unwrap());
                if password.len() == 8 {
                    return password;
                }
            }
            i += 1;
        }
    }

    fn part_two_driver(&self, input: &str) -> String {
        let mut i = 0;
        let mut password = ['\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];
        let mut characters_found = 0;
        loop {
            let h = md5::calculate_hash_bytes(format!("{}{}", input, i).into_bytes());
            if h[0] == 0 && h[1] == 0 && h[2] & 0xF0 == 0 {
                let index = (h[2] & 0xF) as usize;
                if index < 8 && password[index] == '\0' {
                    characters_found += 1;
                    password[index] = format!("{:01x}", (h[3] & 0xF0) >> 4)
                        .chars()
                        .nth(0)
                        .unwrap();
                    if characters_found == 8 {
                        return password.iter().collect::<String>();
                    }
                }
            }
            i += 1;
        }
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![("abc", "18f47a30")];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), "2414bc77");
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![("abc", "05ace8e3")];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), "437e60fc");
    }
}
