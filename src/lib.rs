fn hex2bytes(hex_str : &str) -> Vec<u8> {
    return hex::decode(hex_str).unwrap();
}
fn bytes2b64(bytes : Vec<u8>) -> String {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD as b64;
    return b64.encode(bytes);
}

#[test]
fn s1c1_hex2b64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";


    let out = 
    assert_eq!(bytes2b64(hex2bytes(input)), expected);
}
