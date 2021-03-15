const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const AG_HEADER: usize = 4;
const MAX_CHUNK_SIZE: usize = (64 * 1024 - 1) - UDP_HEADER - IP_HEADER - AG_HEADER;

use std::net::UdpSocket;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::{mem::MaybeUninit};
use std::thread;
use std::net::SocketAddr;
// cmp -l 1.jpg 2.jpg

#[inline(always)]
fn memcpy(dst_ptr:*mut u8, src_ptr:*const u8, len:usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
    }
}

#[inline(always)]
fn next_power_of_two(n:u32) -> u32 {
    return 32 - (n - 1).leading_zeros();
}

#[inline(always)]
fn write_chunks_to_file(filename: &str, bytes:&[u8]) -> Result<bool, io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;
    Ok(true)
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    let filename = "2.jpg";
    let mut count = 0;
    let mut chunks_cnt:u32 = 0xffff;
    let mut total_size:usize = 0;
   // let mut s;
    let mut missing_indexes : Vec<u16> = Vec::new();
    let mut layout;
    let mut peer_addr;
    let mut bytes_buf;
    unsafe {
        peer_addr = MaybeUninit::<SocketAddr>::uninit();
        layout = MaybeUninit::<Layout>::uninit().assume_init();
        bytes_buf = MaybeUninit::<*mut u8>::uninit().assume_init();
    };

    loop {
        let mut buf = [0u8; MAX_CHUNK_SIZE + AG_HEADER];
        //let mut missing_indexes = [0u16; 0x10000];

        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => { // thanks https://doc.rust-lang.org/beta/std/net/struct.UdpSocket.html#method.recv_from
                total_size += size;

                let packet_index:usize = (buf[0] as usize) << 8 | buf[1] as usize;
                if count == 0 {

                    chunks_cnt = (buf[2] as u32) << 8 | buf[3] as u32;
                    let n:usize = 0x10000 << next_power_of_two(chunks_cnt);
                   // assert_eq!(n.count_ones(), 1); // can check with this function that n is aligned on power of 2
                    unsafe {
                         layout = Layout::from_size_align_unchecked(n, mem::align_of::<u8>());
                         bytes_buf = alloc(layout);
                         peer_addr.as_mut_ptr().write(src);
                    }
                    let a:Vec<u16> = (0..chunks_cnt).map(|x| x as  u16).collect(); // create a sorted vector with all the required indexes
                    missing_indexes = a;


                }
                missing_indexes.retain(|&x| x != packet_index as u16);

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
                count+=1;

            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }

        }

         if count == chunks_cnt { // all chunks have been collected, write bytes to file
            let bytes = unsafe { std::slice::from_raw_parts(bytes_buf, total_size) };
            let result = write_chunks_to_file(filename, &bytes);
            match result {
                Ok(true) => println!("Succesfully created file: {}", true),
                Ok(false) => println!("Could not create file: {}", false),
                Err(e) => println!("Error: {}", e),
            }
            count = 0;
            total_size = 0;
            unsafe { dealloc(bytes_buf, layout); }
        }
        else {
            unsafe {
                let missing_bytes = missing_indexes.align_to::<u8>().1;
                peer_addr.assume_init();
                sock.send_to(&missing_bytes, &peer_addr.assume_init() ).expect("Failed to send a response");
            }
        }
        /*for e in missing_indexes.iter() {
            println!("{}", e);
        }*/
    }
}

