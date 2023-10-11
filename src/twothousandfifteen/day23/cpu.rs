pub struct Cpu {
    pub a: usize,
    pub b: usize,
    pub pc: i32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { a: 0, b: 0, pc: 0 }
    }

    /// Outputs registers a and b as (a, b).
    pub fn run_program(&mut self, input: &str) {
        let program = input.lines().collect::<Vec<&str>>();
        loop {
            if self.pc < 0 || self.pc >= program.len() as i32 {
                break;
            }

            let line = program[self.pc as usize];
            let mut parts = line.split_terminator(char::is_whitespace);
            match parts.next().unwrap() {
                "hlf" => match parts.next().unwrap() {
                    "a" => self.a /= 2,
                    "b" => self.b /= 2,
                    _ => unreachable!(),
                },
                "tpl" => match parts.next().unwrap() {
                    "a" => self.a *= 3,
                    "b" => self.b *= 3,
                    _ => unreachable!(),
                },
                "inc" => match parts.next().unwrap() {
                    "a" => self.a += 1,
                    "b" => self.b += 1,
                    _ => unreachable!(),
                },
                "jmp" => {
                    let offset = parts.next().unwrap().parse::<i32>().unwrap();
                    self.pc += offset - 1; // -1 because we increment after the match block
                }
                "jie" => {
                    let r = match parts.next().unwrap() {
                        "a," => self.a,
                        "b," => self.b,
                        _ => unreachable!(),
                    };

                    if r & 0x1 == 0 {
                        let offset = parts.next().unwrap().parse::<i32>().unwrap();
                        self.pc += offset - 1; // -1 because we increment after the match block
                    }
                }
                "jio" => {
                    let r = match parts.next().unwrap() {
                        "a," => self.a,
                        "b," => self.b,
                        _ => unreachable!(),
                    };

                    if r == 1 {
                        let offset = parts.next().unwrap().parse::<i32>().unwrap();
                        self.pc += offset - 1; // -1 because we increment after the match block
                    }
                }
                _ => unreachable!(),
            }

            self.pc += 1;
        }
    }
}
