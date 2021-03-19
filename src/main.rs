// use tiny_keccak::Sha3;
// use tiny_keccak::Hasher;
//use ring::ring;
//use ring::test;
//use std::collections::HashMap;
//use std::fs::File;
//use std::io;
//use std::io::Write;
//use std::path::Path;

// Thanks https://www.rosettacode.org/wiki/Extract_file_extension#Rust
fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}

// https://en.wikipedia.org/wiki/List_of_file_signatures
// NB: magic (number) means file signature
fn is_file_extension_matching_magic(filename: &str, bytes: Vec<u8>) -> bool {
    const WILD: u8 = 0xFC; // unspecified byte, could be anything, just make sure
                           // that it is not one of the already used byte among magic numbers
    let file_extension = extension(filename);

    // get supposed magic based on file extension
    let v = match file_extension {
        ".bmp" => [[0x42, 0x4D].to_vec()].to_vec(),
        ".jpg" => [[0xFF, 0xD8, 0xFF].to_vec()].to_vec(),
        ".png" => [[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A].to_vec()].to_vec(),
        ".gif" => [[0x47, 0x49, 0x46, 0x38].to_vec()].to_vec(),
        ".m4a" => [[
            0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x4D, 0x34, 0x41,
        ]
        .to_vec()]
        .to_vec(),
        ".pdf" => [[0x25, 0x50, 0x44, 0x46, 0x2d].to_vec()].to_vec(),
        ".avi" => [[
            0x52, 0x49, 0x46, 0x46, WILD, WILD, WILD, WILD, 0x41, 0x56, 0x49, 0x20,
        ]
        .to_vec()]
        .to_vec(),
        ".mp3" => [
            [0xFF, 0xFB].to_vec(),
            [0xFF, 0xF2].to_vec(),
            [0xFF, 0xF3].to_vec(),
        ]
        .to_vec(),
        ".webp" => [[
            0x52, 0x49, 0x46, 0x46, WILD, WILD, WILD, WILD, 0x57, 0x45, 0x42, 0x50,
        ]
        .to_vec()]
        .to_vec(),
        _ => return true,
    };
    // check that actual magic from bytes match its supposed magic
    'outer: for magic_bytes in v.iter() {
        for i in 0..magic_bytes.len() - 1 {
            //println!("{:x} ", magic_bytes[i]);
            if magic_bytes[i] ^ bytes[i] != 0 && magic_bytes[i] != WILD {
                continue 'outer;
            }
        }
        if magic_bytes[magic_bytes.len() - 1] ^ bytes[magic_bytes.len() - 1] == 0 {
            return true;
        }
    }
    println!(
        "{} with {} ext does not have magic {:x?} matching its extension",
        filename, file_extension, v
    );
    return false;
}

fn main() {
    let bytes_buf: Vec<u8> = [0xAA, 0xFB].to_vec();
    let check: bool = is_file_extension_matching_magic("test.mp3", bytes_buf);
    println!("{}", check);
}

//https://stackoverflow.com/questions/44575380/is-there-any-way-to-insert-multiple-entries-into-a-hashmap-at-once-in-rust

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
