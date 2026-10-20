use uuid::Uuid;

const UUID_VERSIONS: [&str; 7] = ["u1", "u3", "u4", "u5", "u6", "u7", "u8"];
const USAGE: &str = "usage: it-tools id [count] [-s | u1|u3|u4|u5|u6|u7|u8 [name]]
  count  number of ids to generate (default 1)
  -s     TSID as a string instead of a number
  uN     UUID of version N; u3 and u5 require a name (hashed in the DNS namespace)";

pub fn id(args: &[String]) -> Result<String, String> {
    let (count, rest) = match args.first().and_then(|a| a.parse::<u32>().ok()) {
        Some(count) => (count, &args[1..]),
        None => (1, args),
    };

    let mut as_string = false;
    let mut version = None;
    let mut name = None;
    for arg in rest {
        match arg.as_str() {
            "-s" => as_string = true,
            v if UUID_VERSIONS.contains(&v) && version.is_none() => version = Some(v),
            n if !n.starts_with('-') && name.is_none() => name = Some(n),
            _ => return Err(USAGE.into()),
        }
    }
    let needs_name = matches!(version, Some("u3" | "u5"));
    if needs_name != name.is_some() || (as_string && version.is_some()) {
        return Err(USAGE.into());
    }

    let ids: Vec<String> = (0..count)
        .map(|_| match version {
            Some(v) => uuid(v, name.unwrap_or_default()).to_string(),
            None if as_string => tsid::create_tsid().to_string(),
            None => tsid::create_tsid().number().to_string(),
        })
        .collect();
    Ok(ids.join("\n"))
}

fn uuid(version: &str, name: &str) -> Uuid {
    // v1/v6 normally embed a MAC address; a random node id is allowed by RFC 9562.
    let node_id: [u8; 6] = rand::random();
    match version {
        "u1" => Uuid::now_v1(&node_id),
        "u3" => Uuid::new_v3(&Uuid::NAMESPACE_DNS, name.as_bytes()),
        "u4" => Uuid::new_v4(),
        "u5" => Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()),
        "u6" => Uuid::now_v6(&node_id),
        "u7" => Uuid::now_v7(),
        "u8" => Uuid::new_v8(rand::random()),
        _ => unreachable!(),
    }
}
