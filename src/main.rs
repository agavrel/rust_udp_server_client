use tiny_keccak::Sha3;
use tiny_keccak::Hasher;

fn main() {
    let mut sha3 = Sha3::v512();
    let mut output = [0u8; 32];
   /* let expected = b"\
        \x64\x4b\xcc\x7e\x56\x43\x73\x04\x09\x99\xaa\xc8\x9e\x76\x22\xf3\
        \xca\x71\xfb\xa1\xd9\x72\xfd\x94\xa3\x1c\x3b\xfb\xf2\x4e\x39\x38\
    ";*/
    let expected:[u8; 32] = [132u8, 0u8, 6u8, 101u8, 62u8, 154u8, 201u8, 233u8, 81u8, 23u8, 161u8, 92u8, 145u8, 92u8, 170u8, 184u8, 22u8, 98u8, 145u8, 142u8, 146u8, 93u8, 233u8, 224u8, 4u8, 247u8, 116u8, 255u8, 130u8, 215u8, 7u8, 154u8];

    sha3.update(b"hello");
    sha3.update(b" ");
    sha3.update(b"world");
    sha3.finalize(&mut output);
    assert_eq!(expected, output);

    sha3 = Sha3::v512();
    sha3.update(b"hello world");
    sha3.finalize(&mut output);
    assert_eq!(expected, output);
}