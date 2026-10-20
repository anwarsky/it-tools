use sha2::Digest;

pub fn hash(args: &[String]) -> Result<String, String> {
    let [text, algo] = args else {
        return Err("usage: it-tools hash <text> <MD5|SHA|SHA1|SHA224|SHA256|SHA384|SHA512|RIPEMD160>".into());
    };
    let text = text.as_bytes();
    let bytes = match algo.to_uppercase().as_str() {
        "MD5" => digest::<md5::Md5>(text),
        "SHA" | "SHA1" => digest::<sha1::Sha1>(text),
        "SHA224" => digest::<sha2::Sha224>(text),
        "SHA256" => digest::<sha2::Sha256>(text),
        "SHA384" => digest::<sha2::Sha384>(text),
        "SHA512" => digest::<sha2::Sha512>(text),
        "RIPEMD160" => digest::<ripemd::Ripemd160>(text),
        _ => return Err(format!("unknown hash algorithm: {algo}")),
    };
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn digest<D: Digest>(text: &[u8]) -> Vec<u8> {
    D::digest(text).to_vec()
}
