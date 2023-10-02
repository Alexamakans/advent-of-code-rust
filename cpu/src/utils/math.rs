pub fn factorial(value: u32) -> u32 {
    if value == 0 {
        return 1;
    }

    let mut result = 1;
    for factor in 2..=value {
        result *= factor;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_works() {
        let cases = vec![
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 6),
            (10, 3628800),
        ];

        for case in cases {
            assert_eq!(factorial(case.0), case.1, "input = {}", case.0);
        }
    }
}