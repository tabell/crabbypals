const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

fn main() {
    println!("hex: {}", INPUT);
    let bytes = hex::decode(INPUT).unwrap();
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD as b64;
    let out = b64.encode(bytes);

    println!("base64: {}", out);
}
