use std::net::UdpSocket;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::{mem::MaybeUninit};
use std::thread;
use std::net::SocketAddr;

const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const AG_HEADER: usize = 4;
const MAX_DATA_LENGTH: usize = (64 * 1024 - 1) - UDP_HEADER - IP_HEADER;
const MAX_CHUNK_SIZE: usize = MAX_DATA_LENGTH - AG_HEADER;
const MAX_DATAGRAM_SIZE: usize = 0x10000;
// cmp -l 1.jpg 2.jpg

/// A wrapper for [ptr::copy_nonoverlapping] with different argument order (same as original memcpy)
/// Safety: see `std::ptr::copy_nonoverlapping`.
#[inline(always)]
unsafe fn memcpy(dst_ptr:*mut u8, src_ptr:*const u8, len:usize) {
    std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
}

#[inline(always)]
// Different from https://doc.rust-lang.org/std/primitive.u32.html#method.next_power_of_two
// Returns the [exponent] from the smallest power of two greater than or equal to n.
const fn next_power_of_two_exponent(n:u32) -> u32 {
    return 32 - (n - 1).leading_zeros();
}

#[inline(always)]
fn write_chunks_to_file(filename: &str, bytes:&[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    Ok(file.write_all(bytes)?)
}
/*
fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}

fn check_file_type_integrity(filename: &str, bytes: Vec<u8>) -> bool {
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
        //println!("{:#x?} bytes -> {} file", k, v);
    }
    return true;
}
*/
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    let filename = "3.m4a";
    let mut total_size:usize = 0;
    let mut missing_indexes : Vec<u16> = Vec::new();
    let mut layout = MaybeUninit::<Layout>::uninit();
    let mut peer_addr = MaybeUninit::<SocketAddr>::uninit();
    let mut bytes_buf = std::ptr::null_mut();
    let mut buf = [0u8; MAX_DATA_LENGTH];

    loop {

        //let mut missing_indexes = [0u16; 0x10000];

        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => { // thanks https://doc.rust-lang.org/beta/std/net/struct.UdpSocket.html#method.recv_from


                let packet_index:usize = (buf[0] as usize) << 8 | buf[1] as usize;
                if missing_indexes.is_empty() {
                    let chunks_cnt:u32 = (buf[2] as u32) << 8 | buf[3] as u32;
                    let n:usize = MAX_DATAGRAM_SIZE << next_power_of_two_exponent(chunks_cnt);
                    debug_assert_eq!(n.count_ones(), 1); // can check with this function that n is aligned on power of 2
                    unsafe {
                         // SAFETY: layout.as_mut_ptr() is valid for writing and properly aligned
                         // SAFETY: align_of<u8>() is nonzero and a power of two thanks to previous function
                         // SAFETY: no shift amount will make 0x10000 << x round up to usize::MAX
                         layout.as_mut_ptr().write(Layout::from_size_align_unchecked(n, mem::align_of::<u8>()));
                         // SAFETY: layout is initialized right before calling assume_init()
                         bytes_buf = alloc(layout.assume_init());
                         peer_addr.as_mut_ptr().write(src);
                    }
                    let a:Vec<u16> = (0..chunks_cnt).map(|x| x as  u16).collect(); // create a sorted vector with all the required indexes
                    missing_indexes = a;


                }
                if missing_indexes.iter().any(|&i| i==packet_index as u16) {
                    total_size += size;
                    missing_indexes.retain(|&x| x != packet_index as u16);
                }


                unsafe {
                    let dst_ptr = bytes_buf.offset((packet_index*MAX_CHUNK_SIZE) as isize);
                    memcpy(dst_ptr, &buf[AG_HEADER], size-AG_HEADER);
                };
                thread::spawn(move || {
                   // let s = String::from_utf8_lossy(&buf);
                  // println!("receiving packet {} from: {} : {:?}", packet_index, src, &buf);
                    println!("receiving packet {} from: {}", packet_index, src);
                 //   sock.send_to(&buf, &src).expect("Failed to send a response");
                });
               // println!("count: {}", count);

            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }

        }
        if !missing_indexes.is_empty() {
            unsafe {
                let missing_chunks = missing_indexes.align_to::<u8>().1;
                sock.send_to(&missing_chunks, &peer_addr.assume_init() ).expect("Failed to send a response");
            }
        }
        else {// all chunks have been collected, write bytes to file
            let bytes = unsafe { std::slice::from_raw_parts(bytes_buf, total_size) };
        //    if check_file_type_integrity(filename, bytes) == true {
                let result = write_chunks_to_file(filename, &bytes);
                match result {
                    Ok(()) => println!("Succesfully created file: {}", true),
                    Err(e) => println!("Error: {}", e),
                }
          /*  }
            else {
                println!("file  {} does not match his true type", filename),
            }*/
            unsafe { dealloc(bytes_buf, layout.assume_init()); }

        }

        /*for e in missing_indexes.iter() {
            println!("{}", e);
        }*/
    }
}

