const DEF_COST: u32 = 10;
const USAGE: &str = "usage: it-tools bcrypt <text> [salt-count]
       it-tools bcrypt <text> <hash>
  salt-count  cost factor, 4-31 (default 10); the salt itself is random
  hash        bcrypt hash to check the text against; prints true or false";

pub fn bcrypt(args: &[String]) -> Result<String, String> {
    let (text, cost) = match args {
        [text] => (text, DEF_COST),
        [text, hash] if hash.starts_with('$') => {
            return ::bcrypt::verify(text, hash)
                .map(|matches| matches.to_string())
                .map_err(|err| format!("{err}\n{USAGE}"));
        }
        [text, cost] => (text, cost.parse().map_err(|_| USAGE.to_string())?),
        _ => return Err(USAGE.into()),
    };
    ::bcrypt::hash(text, cost).map_err(|err| format!("{err}\n{USAGE}"))
}
