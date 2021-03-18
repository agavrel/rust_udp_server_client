// use tiny_keccak::Sha3;
// use tiny_keccak::Hasher;
//use ring::ring;
//use ring::test;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

// thanks https://www.rosettacode.org/wiki/Extract_file_extension#Rust
fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}

fn check_file_type(filename: &str, bytes: Vec<u8>) -> bool {
    let map: HashMap<Vec<u8>, &str> = [
        ([0x42u8, 0x40u8].to_vec(), ".bmp"),
        ([0xFFu8, 0xD8u8, 0xFFu8].to_vec(), ".jpg"),
        ([0x89u8, 0x50u8, 0x4Eu8, 0x47u8].to_vec(), ".png"),
        ([0x47u8, 0x49u8, 0x46u8, 0x38u8].to_vec(), ".gif"),
    ].iter().cloned().collect();

    for (k, v) in& map {
        if bytes.eq(k) == true {
            return v == &extension(filename)
        }
        println!("{:#x?} bytes -> {} file", k, v);
    }
    return true;
}
//https://stackoverflow.com/questions/44575380/is-there-any-way-to-insert-multiple-entries-into-a-hashmap-at-once-in-rust

fn main() {
    let mut bytes_buf: Vec<u8> = [0x42u8, 0x40u8].to_vec();
    let check: bool = check_file_type("baba.bmp", bytes_buf);
    println!("{}", check);

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    //let mut map = HashMap::new();

    /*
    map.insert();
    map.insert([0x42u8, 0x40u8], "jpg");
    map.insert([0x42u8, 0x40u8], "png");
    map.insert([0x42u8, 0x40u8], "gif");*/


    //BMP 0x42, 0x4D
    // JPG : FF D8 FF
    // PNG :
    // GIF :
}

/*
test::run(test::test_file!("../2.m4a"), |section, test_case| {
    assert_eq!(section, ""); // This test doesn't use named sections.

    let digest_alg = test_case.consume_digest_alg("HMAC");
    let input = test_case.consume_bytes("Input");
    let key = test_case.consume_bytes("Key");
    let output = test_case.consume_bytes("Output");


    // Do the actual testing here
});*/
/* let expected = b"\
    \x64\x4b\xcc\x7e\x56\x43\x73\x04\x09\x99\xaa\xc8\x9e\x76\x22\xf3\
    \xca\x71\xfb\xa1\xd9\x72\xfd\x94\xa3\x1c\x3b\xfb\xf2\x4e\x39\x38\
";*/
/* let mut sha3 = Sha3::v512();
let mut output = [0u8; 32];

let expected:[u8; 32] = [132u8, 0u8, 6u8, 101u8, 62u8, 154u8, 201u8, 233u8, 81u8, 23u8, 161u8, 92u8, 145u8, 92u8, 170u8, 184u8, 22u8, 98u8, 145u8, 142u8, 146u8, 93u8, 233u8, 224u8, 4u8, 247u8, 116u8, 255u8, 130u8, 215u8, 7u8, 154u8];

sha3.update(b"hello");
sha3.update(b" ");
sha3.update(b"world");
sha3.finalize(&mut output);
assert_eq!(expected, output);

sha3 = Sha3::v512();
sha3.update(b"hello world");
sha3.finalize(&mut output);
assert_eq!(expected, output); */

// test_file!("");
