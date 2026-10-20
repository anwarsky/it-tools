pub fn id(args: &[String]) {
    let as_string = args.iter().any(|a| a == "-s");
    let count = 2;
    for _ in 0..count {
        if as_string {
            println!("{}", tsid::create_tsid());
        } else {
            println!("{}", tsid::create_tsid().number());
        }
    }
    println!()
}