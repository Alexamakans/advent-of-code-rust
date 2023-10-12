/// Only rotates alphabet characters, and is case insensitive.
pub fn caesar_cipher(s: &str, amount: usize) -> String {
    static ALPHANUMERICS: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut result = String::new();
    'next_letter: for c in s.chars() {
        let index = 'found: {
            for x in ALPHANUMERICS.iter().enumerate() {
                if x.1 == &c {
                    break 'found x.0;
                }
            }
            result.push(c);
            continue 'next_letter;
        };

        let new_index = (index + amount) % ALPHANUMERICS.len();
        result.push(ALPHANUMERICS[new_index]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher_works() {
        let alphanumerics = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(caesar_cipher(alphanumerics, 29), "defghijklmnopqrstuvwxyzabc");
    }
}
