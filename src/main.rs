pub mod bcrypt;
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
        Some("hash") => match hash::hash(&args[1..]) {
            Ok(hash) => println!("{hash}"),
            Err(err) => eprintln!("{err}"),
        },
        _ => eprintln!("usage: it-tools <token|id|hash|bcrypt> [options]"),
    }
}
