use ::base64::Engine;
use ::base64::engine::general_purpose::{
    STANDARD, STANDARD_PAD_INDIFFERENT, URL_SAFE, URL_SAFE_PAD_INDIFFERENT,
};

const USAGE: &str = "usage: it-tools base64 <encode|decode> <text> [-u]
  -u  URL-safe alphabet (- and _ instead of + and /)";

pub fn base64(args: &[String]) -> Result<String, String> {
    let url_safe = args.iter().any(|a| a == "-u");
    let rest: Vec<&str> = args.iter().map(String::as_str).filter(|a| *a != "-u").collect();
    match rest[..] {
        ["encode", text] => {
            let engine = if url_safe { URL_SAFE } else { STANDARD };
            Ok(engine.encode(text))
        }
        ["decode", text] => {
            // Decoding accepts input with or without trailing '=' padding.
            let engine = if url_safe { URL_SAFE_PAD_INDIFFERENT } else { STANDARD_PAD_INDIFFERENT };
            let bytes = engine.decode(text).map_err(|err| format!("invalid base64: {err}"))?;
            String::from_utf8(bytes).map_err(|_| "decoded data is not valid UTF-8 text".into())
        }
        _ => Err(USAGE.into()),
    }
}
