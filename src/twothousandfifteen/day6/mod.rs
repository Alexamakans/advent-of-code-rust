use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let mut commands = Vec::new();
        let mut smallest_x = i32::MAX;
        let mut smallest_y = i32::MAX;
        let mut largest_x = i32::MIN;
        let mut largest_y = i32::MIN;

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let mut cmd = parts.next().unwrap().to_string();
            if cmd != "toggle" {
                cmd = format!("{} {}", cmd, parts.next().unwrap());
            }
            let start_xy = parts.next().unwrap();
            let end_xy = parts.skip(1).next().unwrap();

            let start_pos = {
                let tmp = start_xy
                    .split_terminator(',')
                    .map(|e| e.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (*tmp.get(0).unwrap(), *tmp.get(1).unwrap())
            };
            let end_pos = {
                let tmp = end_xy
                    .split_terminator(',')
                    .map(|e| e.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (*tmp.get(0).unwrap(), *tmp.get(1).unwrap())
            };

            smallest_x = *vec![smallest_x, start_pos.0, end_pos.0]
                .iter()
                .min()
                .unwrap();
            smallest_y = *vec![smallest_y, start_pos.1, end_pos.1]
                .iter()
                .min()
                .unwrap();
            largest_x = *vec![largest_x, start_pos.0, end_pos.0]
                .iter()
                .max()
                .unwrap();
            largest_y = *vec![largest_y, start_pos.1, end_pos.1]
                .iter()
                .max()
                .unwrap();

            let cmd_number = match cmd.as_str() {
                "turn on" => 0,
                "turn off" => 1,
                "toggle" => 2,
                _ => unreachable!(),
            };

            commands.push((start_pos, end_pos, cmd_number));
        }

        let offset_x = -smallest_x;
        let offset_y = -smallest_y;

        let width = largest_x - smallest_x + 1;
        let height = largest_y - smallest_y + 1;

        let mut world: Vec<Vec<bool>> = Vec::with_capacity(width as usize);
        for _ in 0..width {
            let mut column = Vec::with_capacity(height as usize);
            for _ in 0..height {
                column.push(false);
            }
            world.push(column);
        }

        fn do_thing(cell: bool, c: i32) -> bool {
            match c {
                0 => true,
                1 => false,
                2 => !cell,
                _ => unreachable!(),
            }
        }

        for command in commands {
            let start_x = command.0 .0;
            let start_y = command.0 .1;
            let end_x = command.1 .0;
            let end_y = command.1 .1;
            let cmd = command.2;

            for ix in start_x..=end_x {
                for iy in start_y..=end_y {
                    let x = (ix + offset_x) as usize;
                    let y = (iy + offset_y) as usize;
                    let column = world.get_mut(x).unwrap();
                    let cell = column.get_mut(y).unwrap();
                    *cell = do_thing(*cell, cmd);
                }
            }
        }

        world.iter().flatten().filter(|e| **e).count() as i32
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let mut commands = Vec::new();
        let mut smallest_x = i32::MAX;
        let mut smallest_y = i32::MAX;
        let mut largest_x = i32::MIN;
        let mut largest_y = i32::MIN;

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let mut cmd = parts.next().unwrap().to_string();
            if cmd != "toggle" {
                cmd = format!("{} {}", cmd, parts.next().unwrap());
            }
            let start_xy = parts.next().unwrap();
            let end_xy = parts.skip(1).next().unwrap();

            let start_pos = {
                let tmp = start_xy
                    .split_terminator(',')
                    .map(|e| e.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (*tmp.get(0).unwrap(), *tmp.get(1).unwrap())
            };
            let end_pos = {
                let tmp = end_xy
                    .split_terminator(',')
                    .map(|e| e.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (*tmp.get(0).unwrap(), *tmp.get(1).unwrap())
            };

            smallest_x = *vec![smallest_x, start_pos.0, end_pos.0]
                .iter()
                .min()
                .unwrap();
            smallest_y = *vec![smallest_y, start_pos.1, end_pos.1]
                .iter()
                .min()
                .unwrap();
            largest_x = *vec![largest_x, start_pos.0, end_pos.0]
                .iter()
                .max()
                .unwrap();
            largest_y = *vec![largest_y, start_pos.1, end_pos.1]
                .iter()
                .max()
                .unwrap();

            let cmd_number = match cmd.as_str() {
                "turn on" => 0,
                "turn off" => 1,
                "toggle" => 2,
                _ => unreachable!(),
            };

            commands.push((start_pos, end_pos, cmd_number));
        }

        let offset_x = -smallest_x;
        let offset_y = -smallest_y;

        let width = largest_x - smallest_x + 1;
        let height = largest_y - smallest_y + 1;

        let mut world: Vec<Vec<u32>> = Vec::with_capacity(width as usize);
        for _ in 0..width {
            let mut column = Vec::with_capacity(height as usize);
            for _ in 0..height {
                column.push(0_u32);
            }
            world.push(column);
        }

        fn do_thing(cell: u32, c: i32) -> u32 {
            match c {
                0 => cell + 1,
                1 => {
                    if cell >= 1 {
                        cell - 1
                    } else {
                        0
                    }
                }
                2 => cell + 2,
                _ => unreachable!(),
            }
        }

        for command in commands {
            let start_x = command.0 .0;
            let start_y = command.0 .1;
            let end_x = command.1 .0;
            let end_y = command.1 .1;
            let cmd = command.2;

            for ix in start_x..=end_x {
                for iy in start_y..=end_y {
                    let x = (ix + offset_x) as usize;
                    let y = (iy + offset_y) as usize;
                    let column = world.get_mut(x).unwrap();
                    let cell = column.get_mut(y).unwrap();
                    *cell = do_thing(*cell, cmd);
                }
            }
        }

        world.iter().flatten().fold(0_u32, |acc, v| acc + *v) as i32
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r#"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500"#,
            1000 * 1000 - 1000 - 4,
        )];

        for case in cases {
            assert_eq!(solver.part_one_driver(case.0), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 569999);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r#"turn on 0,0 through 0,0
toggle 0,0 through 999,999"#,
            1 + 2000000,
        )];

        for case in cases {
            assert_eq!(solver.part_two_driver(case.0), case.1, "input = {}", case.0);
        }

        // assert_eq!(solver.part_two(), 123);
    }
}
