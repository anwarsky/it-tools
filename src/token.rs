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

    let has = |flag: &str| args.iter().any(|s| s == flag);

    let mut alphabet = String::new();
    if has("-c") {
        alphabet += CAPITAL_LETTERS;
    }
    if has("-l") {
        alphabet += LOWER_CASE_LETTERS;
    }
    if has("-d") {
        alphabet += DIGITS;
    }
    if has("-s") {
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
