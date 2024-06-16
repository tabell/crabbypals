use std::cmp;
use std::collections::HashMap;


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
    let upper = cmp::max(a.len(), b.len());

    for i in 0..upper {
        out.push(a[i % a.len()] ^ b[i % b.len()]);
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
fn s1c2_xor() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(hexor(a,b),expected);
}

#[test]
fn test_fixed_key_xor() {
    let input = "414243444546";
    let key   = "41";
    assert_eq!(hexor(input,key), "000302050407")
}

const WEIGHTS: [f64; 26] = [8.55, 1.6 , 3.16, 3.87, 12.10, 2.18, 2.09, 4.96, 7.33, 0.22, 0.81, 4.21, 2.53, 7.17, 7.47, 2.07, 0.10, 6.33, 6.73, 8.94, 2.68, 1.06, 1.83, 0.19, 1.72, 0.11 ];
fn score_string(input : &str) -> f64 {
    for (i, c) in input.chars().enumerate() {
    }
    return 0.0;
}

#[test] // Set 1 challenge 3
fn s1c2() {
    let _input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let score = score_string(_input);

}

