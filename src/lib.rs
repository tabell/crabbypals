use std::cmp;
use bytes::Bytes;

//use std::collections::HashMap;


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

fn xor(a : impl AsRef<[u8]>, b : impl AsRef<[u8]>) -> Vec<u8> {
    let a = a.as_ref();
    let b = b.as_ref();
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

const WEIGHTS: [i32; 26] = [855, 160 , 316, 387, 1210, 218, 209, 496, 733, 22, 81, 421, 253, 717, 747, 207, 10, 633, 673, 894, 268, 106, 183, 19, 172, 11 ];
fn score_string(input : impl AsRef<[u8]>) -> f32 {
    let input = input.as_ref();
    let total = input.iter().map(|c| c.to_ascii_uppercase() - 65).filter(|c| *c < 26 as u8).fold(0, |acc,idx| acc + WEIGHTS[idx as usize]);

    total as f32 / input.len() as f32
}
//#[test] // Test scoring
//fn test_scoring() {
//    let score = score_string(<Vec<u8>>::from("ghosts"));
//    println!("score = {score}");
//}

#[test] // Set 1 challenge 3
fn test_s1c3() {
    //let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //let score = score_string(xor(hex2bytes(input), vec!['a' as u8]));
    //let score = score_string(xor(<Vec<u8>>::from("ffff"), vec!['a' as u8]));
    let test_str = b"eeeeEEEE";
    //let test_str = xor(b"abcdABCD", b"b");
    let score = score_string(&test_str);
    //let t_as_ref = test_str.as_ref();
    //let b = Bytes::from(test_str);
    println!("string={:?}, score = {score}", test_str.escape_ascii().to_string());
}

