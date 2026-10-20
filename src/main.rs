pub mod id;
pub mod token;
pub mod vec_utils;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("token") => println!("{}", token::token(&args[1..])),
        Some("id") => id::id(&args[1..]),
        _ => eprintln!("usage: it-tools <token|id> [options]"),
    }
}
