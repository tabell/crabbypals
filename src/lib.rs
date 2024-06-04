use std::cmp;

fn hex2bytes(hex_str : &str) -> Vec<u8> {
    return hex::decode(hex_str).unwrap();
}
fn bytes2b64(bytes : Vec<u8>) -> String {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD as b64;
    return b64.encode(bytes);
}

pub fn hex2b64(hex_str:&str) -> String {
    return bytes2b64(hex2bytes(hex_str));
}

fn xor(a : Vec<u8>, b : Vec<u8>) -> Vec<u8> {
    
    let mut out = Vec::new();

    for i in 0..cmp::min(a.len(), b.len()) {
        out.push(a[i] ^ b[i]);
    }
    return out;
}

pub fn hexor(a: &str, b: &str) -> String {
    return hex::encode(xor(hex2bytes(a), hex2bytes(b)));
}


#[test] // Set 1 challenge 1
fn s1c1_hex2bytes() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(hex2b64(input), expected);
}

#[test] // Set 1 challenge 2
fn s1c2_fixed_xor() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(hexor(a,b),expected);
}
