use uuid::Uuid;

const UUID_VERSIONS: [&str; 7] = ["u1", "u3", "u4", "u5", "u6", "u7", "u8"];

pub fn id(args: &[String]) {
    let as_string = args.iter().any(|a| a == "-s");
    let uuid_version = args.iter().find(|a| UUID_VERSIONS.contains(&a.as_str()));
    let count = 2;
    for _ in 0..count {
        if let Some(version) = uuid_version {
            match uuid(version, args) {
                Some(uuid) => println!("{uuid}"),
                None => {
                    eprintln!("{version} needs a name: it-tools id {version} <name>");
                    return;
                }
            }
        } else if as_string {
            println!("{}", tsid::create_tsid());
        } else {
            println!("{}", tsid::create_tsid().number());
        }
    }
    println!()
}

fn uuid(version: &str, args: &[String]) -> Option<Uuid> {
    // v3/v5 hash a name within the DNS namespace; the name is the first non-flag argument.
    let name = args
        .iter()
        .find(|a| !a.starts_with('-') && !UUID_VERSIONS.contains(&a.as_str()));
    // v1/v6 normally embed a MAC address; a random node id is allowed by RFC 9562.
    let node_id: [u8; 6] = rand::random();
    Some(match version {
        "u1" => Uuid::now_v1(&node_id),
        "u3" => Uuid::new_v3(&Uuid::NAMESPACE_DNS, name?.as_bytes()),
        "u4" => Uuid::new_v4(),
        "u5" => Uuid::new_v5(&Uuid::NAMESPACE_DNS, name?.as_bytes()),
        "u6" => Uuid::now_v6(&node_id),
        "u7" => Uuid::now_v7(),
        "u8" => Uuid::new_v8(rand::random()),
        _ => unreachable!(),
    })
}
