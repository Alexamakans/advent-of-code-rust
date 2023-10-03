use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<usize> for Solver {
    fn part_one_driver(&self, input: &str) -> usize {
        let on_character = '#';
        let mut world = conway::World::from_str(input, on_character, '.').unwrap();
        for _ in 0..100 {
            world.update();
        }
        format!("{}", world)
            .chars()
            .filter(|e| *e == on_character)
            .count()
    }

    fn part_two_driver(&self, input: &str) -> usize {
        fn set_corners_to_on(world: &mut conway::World) {
            world.set_cell_state(0, 0, true).unwrap();
            world
                .set_cell_state(0, world.get_height() - 1, true)
                .unwrap();
            world
                .set_cell_state(world.get_width() - 1, 0, true)
                .unwrap();
            world
                .set_cell_state(world.get_width() - 1, world.get_height() - 1, true)
                .unwrap();
        }

        let on_character = '#';
        let mut world = conway::World::from_str(input, on_character, '.').unwrap();
        set_corners_to_on(&mut world);
        for _ in 0..100 {
            world.update();
            set_corners_to_on(&mut world);
        }
        format!("{}", world)
            .chars()
            .filter(|e| *e == on_character)
            .count()
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 18)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver {};
        let cases = vec![(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            r"......
......
..##..
..##..
......
......",
            4,
        )];

        for case in cases {
            let mut world = conway::World::from_str(case.0, '#', '.').unwrap();
            for _ in 0..case.2 {
                world.update();
            }
            assert_eq!(format!("{}", world), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_one(), 1061);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver {};
        let cases = vec![(
            r".#.#.#
...##.
#....#
..#...
#.#..#
####..",
            r"##.###
.##..#
.##...
.##...
#.#...
##...#",
            5,
        )];

        fn set_corners_to_on(world: &mut conway::World) {
            world.set_cell_state(0, 0, true).unwrap();
            world
                .set_cell_state(0, world.get_height() - 1, true)
                .unwrap();
            world
                .set_cell_state(world.get_width() - 1, 0, true)
                .unwrap();
            world
                .set_cell_state(world.get_width() - 1, world.get_height() - 1, true)
                .unwrap();
        }

        for case in cases {
            let mut world = conway::World::from_str(case.0, '#', '.').unwrap();
            set_corners_to_on(&mut world);
            for _ in 0..case.2 {
                world.update();
                set_corners_to_on(&mut world);
            }
            assert_eq!(format!("{}", world), case.1, "input = {}", case.0);
        }

        assert_eq!(solver.part_two(), 1006);
    }
}
