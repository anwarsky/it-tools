use crate::vec_utils::has;

fn capital_letters() -> String {
    ('A'..='Z').collect()
}

fn lower_case_letters() -> String {
    ('a'..='z').collect()
}

fn digits() -> String {
    ('0'..='9').collect()
}

const SYMBOLS: &str = "+-*/()[]{}$@#%&";
const DEF_LENGTH: u32 = 12;

pub fn token(args: &[String]) -> String {
    let len = args
        .iter()
        .find_map(|s| s.parse::<u32>().ok())
        .unwrap_or(DEF_LENGTH);

    let mut alphabet = String::new();
    if has("-c", args) {
        alphabet += &capital_letters();
    }
    if has("-l", args) {
        alphabet += &lower_case_letters();
    }
    if has("-d", args) {
        alphabet += &digits();
    }
    if has("-s", args) {
        alphabet += SYMBOLS;
    }
    if alphabet.is_empty() {
        alphabet = [capital_letters(), lower_case_letters(), digits()].concat();
    }

    let chars: Vec<char> = alphabet.chars().collect();
    (0..len)
        .map(|_| chars[rand::random_range(0..chars.len())])
        .collect()
}
