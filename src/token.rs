use crate::vec_utils::has;

const CAPITAL_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "+-*/()[]{}$@#%&";
const DEF_LENGTH: u32 = 12;

pub fn token(args: &[String]) -> String {
    let len = args
        .iter()
        .find_map(|s| s.parse::<u32>().ok())
        .unwrap_or(DEF_LENGTH);



    let mut alphabet = String::new();
    if has("-c", args) {
        alphabet += CAPITAL_LETTERS;
    }
    if has("-l", args) {
        alphabet += LOWER_CASE_LETTERS;
    }
    if has("-d", args) {
        alphabet += DIGITS;
    }
    if has("-s", args) {
        alphabet += SYMBOLS;
    }
    if alphabet.is_empty() {
        alphabet = [CAPITAL_LETTERS, LOWER_CASE_LETTERS, DIGITS].concat();
    }

    let chars: Vec<char> = alphabet.chars().collect();
    (0..len)
        .map(|_| chars[rand::random_range(0..chars.len())])
        .collect()
}
