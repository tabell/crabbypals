use std::cmp;
use std::str;
use std::collections::BTreeMap;
use std::collections::HashMap;
use ordered_float::NotNan;

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

//fn weight_of(c : impl 

fn weight_of(c: char) -> Option<i32> {
    let letter = c.to_ascii_uppercase();
    match letter {
        'A'..='Z' => Some(WEIGHTS[letter as usize - 'A' as usize]),
        _ => None,
    }
}

fn lookup_letter_weight(letters: impl IntoIterator<Item = char>) -> i32 {
    letters.into_iter().filter_map(weight_of).sum()
}

fn make_golden_hist() -> HashMap<char, f32> {
    let mut golden = HashMap::new();

    golden.insert(' ', 2);
    golden.insert('A', 9);
    golden.insert('B', 2);
    golden.insert('C', 2);
    golden.insert('D', 4);
    golden.insert('E', 12);
    golden.insert('F', 2);
    golden.insert('G', 3);
    golden.insert('H', 2);
    golden.insert('I', 9);
    golden.insert('J', 1);
    golden.insert('K', 1);
    golden.insert('L', 4);
    golden.insert('M', 2);
    golden.insert('N', 6);
    golden.insert('O', 8);
    golden.insert('P', 2);
    golden.insert('Q', 1);
    golden.insert('R', 6);
    golden.insert('S', 4);
    golden.insert('T', 6);
    golden.insert('U', 4);
    golden.insert('V', 2);
    golden.insert('W', 2);
    golden.insert('X', 1);
    golden.insert('Y', 2);
    golden.insert('Z', 1);
    golden.insert('~', -9);

    return norm_hist(golden);
}

const OTHER_BUCKET: char = '~'; // this can be anything outside A-Z, even if it appears in input
                                // str it won't interfere with calculation

fn make_hist(input : Vec<u8>) -> HashMap<char, f32> {
    let mut counter = HashMap::new();

    for c in input.into_iter() {
        let letter = c.to_ascii_uppercase() as char;
        match letter {
            ' '..='z' => match counter.get(&letter) {
                Some(&count) => counter.insert(letter, count + 1),
                _ => counter.insert(letter, 1),
            }
            // All other chars including unprintable go in a single bucket
            _ => match counter.get(&OTHER_BUCKET) {
                Some(&count) => counter.insert(OTHER_BUCKET, count + 1),
                _ => counter.insert(OTHER_BUCKET, 1),
            }
        };
    };

    // Normalize by length
    return norm_hist(counter);
}

fn norm_hist(raw: HashMap<char, i32>) -> HashMap<char, f32> {
    let inner_product = (raw.iter().fold(0, |acc, (_, score)| acc + score * score) as f32).sqrt();
    return raw.into_iter().map(|(k,v)| (k, v as f32 / inner_product as f32)).collect();
}

fn hist_lookup(letter: char, hist: HashMap<char, f32>) -> f32 {
    let maybe =hist.get(&letter);
    match maybe {
        None => 0.0,
        Some(x) => *x,
    }
}

fn score_string_hist(input : Vec<u8>) -> f32 {
    let test = make_hist(input);
    let golden = make_golden_hist();

    let r: f32 = test.into_iter().fold(0.0, |acc, (test_letter, test_score)| acc + test_score * hist_lookup(test_letter, golden.clone()));
    return r;
}

#[test] // Set 1 challenge 3
fn test_s1c3() {
    let golden = make_golden_hist();
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut potential_keys = BTreeMap::new();
    //println!("string={:?}, score = {score}", input);
    for x in 1u8..127 {
        let test = xor(hex2bytes(input), vec![x as u8]);
        let score = NotNan::new(score_string_hist(test.clone())).unwrap();
        potential_keys.insert(score, x as char);
    }
    let best_match = potential_keys.last_key_value().unwrap().1;
    println!("top match is {:?}: decoding...", best_match);
    let decoded = xor(hex2bytes(input), vec![*best_match as u8]);
    println!("decoded: {:?}", String::from_utf8(decoded));

}

