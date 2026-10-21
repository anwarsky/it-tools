pub mod base64;
pub mod bcrypt;
pub mod decrypt;
pub mod encrypt;
pub mod hash;
pub mod id;
pub mod token;
pub mod vec_utils;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("token") => println!("{}", token::token(&args[1..])),
        Some("id") => match id::id(&args[1..]) {
            Ok(ids) => println!("{ids}"),
            Err(err) => eprintln!("{err}"),
        },
        Some("bcrypt") => match bcrypt::bcrypt(&args[1..]) {
            Ok(hash) => println!("{hash}"),
            Err(err) => eprintln!("{err}"),
        },
        Some("base64") => match base64::base64(&args[1..]) {
            Ok(text) => println!("{text}"),
            Err(err) => eprintln!("{err}"),
        },
        Some("encrypt") => match encrypt::encrypt(&args[1..]) {
            Ok(text) => println!("{text}"),
            Err(err) => eprintln!("{err}"),
        },
        Some("decrypt") => match decrypt::decrypt(&args[1..]) {
            Ok(text) => println!("{text}"),
            Err(err) => eprintln!("{err}"),
        },
        Some("hash") => match hash::hash(&args[1..]) {
            Ok(hash) => println!("{hash}"),
            Err(err) => eprintln!("{err}"),
        },
        _ => eprintln!("usage: it-tools <token|id|hash|bcrypt|base64|encrypt|decrypt> [options]"),
    }
}
