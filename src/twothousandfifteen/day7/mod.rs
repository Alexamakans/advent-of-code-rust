use std::{cell::RefCell, rc::Rc};

use super::{super::utils::*, YEAR};

pub struct Solver {}
impl DaySolver<i32> for Solver {
    fn part_one_driver(&self, input: &str) -> i32 {
        let wire_signals = get_wires(&input);
        for wire in wire_signals.borrow().iter() {
            if wire.borrow().key == "a" {
                return wire.borrow_mut().get_value() as i32;
            }
        }
        unreachable!();
    }

    fn part_two_driver(&self, input: &str) -> i32 {
        let wire_signals = get_wires(&input);

        let wire_a = 'found: {
            for wire in wire_signals.borrow().iter() {
                if wire.borrow().key == "a" {
                    break 'found wire.clone();
                }
            }
            unreachable!();
        };
        let wire_b = 'found: {
            for wire in wire_signals.borrow().iter() {
                if wire.borrow().key == "b" {
                    break 'found wire.clone();
                }
            }
            unreachable!();
        };

        let wire_a_value = wire_a.borrow_mut().get_value();
        wire_b.borrow_mut().input_a = Rc::new(RefCell::new(WireValue::LITERAL(wire_a_value)));
        wire_b.borrow_mut().operation = Operation::VALUE;
        
        // Reset cache
        for wire in wire_signals.borrow().iter() {
            wire.borrow_mut().cache = None;
        }

        for wire in wire_signals.borrow().iter() {
            if wire.borrow().key == "a" {
                return wire.borrow_mut().get_value() as i32;
            }
        }
        unreachable!();
    }

    fn read_input(&self) -> String {
        read_input(YEAR, 7)
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    VALUE,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::AND {},
            "OR" => Operation::OR {},
            "LSHIFT" => Operation::LSHIFT {},
            "RSHIFT" => Operation::RSHIFT {},
            "NOT" => Operation::NOT {},
            "VALUE" => Operation::VALUE {},
            _ => unreachable!(),
        }
    }
}

impl Operation {
    fn apply(&self, a: Rc<RefCell<WireValue>>, b: Rc<RefCell<WireValue>>) -> u16 {
        match self {
            Operation::AND => a.borrow_mut().get_value() & b.borrow_mut().get_value(),
            Operation::OR => a.borrow_mut().get_value() | b.borrow_mut().get_value(),
            Operation::LSHIFT => a.borrow_mut().get_value() << b.borrow_mut().get_value(),
            Operation::RSHIFT => a.borrow_mut().get_value() >> b.borrow_mut().get_value(),
            Operation::NOT => !a.borrow_mut().get_value(),
            Operation::VALUE => {
                let raw_value = a.borrow_mut().get_value();
                raw_value
            }
        }
    }
}

#[derive(Clone, Debug)]
enum WireValue {
    // From '123 -> x'
    LITERAL(u16),
    // All other cases
    CONNECTION(Rc<RefCell<Wire>>),
}

impl WireValue {
    fn get_value(&mut self) -> u16 {
        match self {
            WireValue::LITERAL(value) => *value,
            WireValue::CONNECTION(wire) => wire.borrow_mut().get_value(),
        }
    }
}

#[derive(Clone, Debug)]
struct Wire {
    key: String,
    input_a: Rc<RefCell<WireValue>>,
    input_b: Rc<RefCell<WireValue>>,
    operation: Operation,
    cache: Option<u16>,
}

impl Wire {
    fn new(key: &str) -> Self {
        Wire {
            key: key.to_owned(),
            input_a: Rc::new(RefCell::new(WireValue::LITERAL(0))),
            input_b: Rc::new(RefCell::new(WireValue::LITERAL(0))),
            operation: Operation::VALUE,
            cache: None,
        }
    }

    fn get_value(&mut self) -> u16 {
        match self.cache {
            Some(value) => value,
            None => {
                let value = self
                    .operation
                    .apply(self.input_a.clone(), self.input_b.clone());
                self.cache = Some(value);
                value
            }
        }
    }
}

fn get_wires(input: &str) -> Rc<RefCell<Vec<Rc<RefCell<Wire>>>>> {
    fn get_or_make_wire(
        wires: Rc<RefCell<Vec<Rc<RefCell<Wire>>>>>,
        key: &str,
    ) -> Rc<RefCell<Wire>> {
        for wire in wires.borrow().iter() {
            if wire.borrow().key == key {
                return wire.clone();
            }
        }

        wires
            .borrow_mut()
            .push(Rc::new(RefCell::new(Wire::new(key))));
        get_or_make_wire(wires, key)
    }

    let wires: Rc<RefCell<Vec<Rc<RefCell<Wire>>>>> = Rc::new(RefCell::new(Vec::new()));
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let mut left_parts = left.split_whitespace();
        let right_wire = get_or_make_wire(wires.clone(), right);
        match left_parts.clone().count() {
            1 => {
                let part = left_parts.next().unwrap();
                right_wire.borrow_mut().operation = Operation::VALUE;
                match part.trim().parse::<u16>() {
                    Ok(literal) => {
                        right_wire.borrow_mut().input_a =
                            Rc::new(RefCell::new(WireValue::LITERAL(literal)));
                    }
                    Err(_) => {
                        let left_wire = get_or_make_wire(wires.clone(), part);
                        right_wire.borrow_mut().input_a = Rc::new(RefCell::new(
                            WireValue::CONNECTION(left_wire),
                        ));
                    }
                }
            }
            2 => {
                let not_operation = Operation::from(left_parts.next().unwrap());
                right_wire.borrow_mut().operation = not_operation;

                let part = left_parts.next().unwrap();
                match part.parse::<u16>() {
                    Ok(literal) => {
                        right_wire.borrow_mut().input_a =
                            Rc::new(RefCell::new(WireValue::LITERAL(literal)));
                    }
                    Err(_) => {
                        let left_wire = get_or_make_wire(wires.clone(), part);
                        right_wire.borrow_mut().input_a = Rc::new(RefCell::new(
                            WireValue::CONNECTION(left_wire),
                        ));
                    }
                }
            }
            3 => {
                let a = left_parts.next().unwrap();
                let operation = Operation::from(left_parts.next().unwrap());
                right_wire.borrow_mut().operation = operation;
                let b = left_parts.next().unwrap();
                match a.parse::<u16>() {
                    Ok(literal) => {
                        right_wire.borrow_mut().input_a =
                            Rc::new(RefCell::new(WireValue::LITERAL(literal)));
                    }
                    Err(_) => {
                        let left_wire = get_or_make_wire(wires.clone(), a);
                        right_wire.borrow_mut().input_a = Rc::new(RefCell::new(
                            WireValue::CONNECTION(left_wire),
                        ));
                    }
                }
                match b.parse::<u16>() {
                    Ok(literal) => {
                        right_wire.borrow_mut().input_b =
                            Rc::new(RefCell::new(WireValue::LITERAL(literal)));
                    }
                    Err(_) => {
                        let left_wire = get_or_make_wire(wires.clone(), b);
                        right_wire.borrow_mut().input_b = Rc::new(RefCell::new(
                            WireValue::CONNECTION(left_wire),
                        ));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    wires
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let solver = Solver{};
        let cases = vec![(
            r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#,
            [
                (String::from("d"), 72),
                (String::from("e"), 507),
                (String::from("f"), 492),
                (String::from("g"), 114),
                (String::from("h"), 65412),
                (String::from("i"), 65079),
                (String::from("x"), 123),
                (String::from("y"), 456),
            ],
        )];

        for case in cases {
            let mut checks_passed = 0;
            let num_checks = case.1.len();
            let wires = get_wires(case.0);
            for (key, want) in case.1.iter() {
                for wire in wires.borrow().iter() {
                    if key.eq(&wire.borrow().key) {
                        assert_eq!(wire.borrow_mut().get_value(), *want as u16, "input = {}", case.0);
                        checks_passed += 1;
                    }
                }
            }

            assert_eq!(checks_passed, num_checks);
        }

        assert_eq!(solver.part_one(), 46065);
    }

    #[test]
    fn part_two_works() {
        let solver = Solver{};
        assert_eq!(solver.part_two(), 14134);
    }
}
