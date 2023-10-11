mod cpu;

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut cpu = cpu::Cpu::new();
        cpu.run_program(input);
        cpu.b as i32
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut cpu = cpu::Cpu::new();
        cpu.a = 1;
        cpu.run_program(input);
        cpu.b as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 23)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r"inc a
jio a, +2
tpl a
inc a",
            2,
        )];

        for case in cases {
            let mut cpu = cpu::Cpu::new();
            cpu.run_program(case.0);
            assert_eq!(cpu.a, case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 255);
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
