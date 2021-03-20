// use tiny_keccak::Sha3;
// use tiny_keccak::Hasher;
//use ring::ring;
//use ring::test;
//use std::collections::HashMap;
//use std::fs::File;
//use std::io;
//use std::io::Write;
//use std::path::Path;

//use sodiumoxide::crypto::hash;
//use sodiumoxide::crypto::shorthash;
use sodiumoxide::randombytes::randombytes;
use sodiumoxide::crypto::secretstream::{gen_key, Stream, Tag};

fn main() {
   // let key = shorthash::gen_key();
    let data: Vec<u8> = randombytes(0xffff);
    // println!("{:?}\n", data);

    // initialize encrypt secret stream
    let key = gen_key();
    let (mut enc_stream, header) = Stream::init_push(&key).unwrap();

    let ciphertext1 = enc_stream.push(&data, None, Tag::Final).unwrap();
    println!("{} {:?} \n", data.len(), &data[0..20]);
    println!("{} {:?} \n", ciphertext1.len(), &ciphertext1[17..37]); // header seems to be 17 bytes length

    // initialize decrypt secret stream
    let mut dec_stream = Stream::init_pull(&header, &key).unwrap();
    // decrypt last message.
    assert!(!dec_stream.is_finalized());
    let (decrypted3, tag3) = dec_stream.pull(&ciphertext1, None).unwrap();
    assert_eq!(tag3, Tag::Final);
    assert_eq!(data, &decrypted3[..]);

    // dec_stream is now finalized.
    assert!(dec_stream.is_finalized());
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
