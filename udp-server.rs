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

// cmp -l 1.jpg 2.jpg

#[inline(always)]
fn memcpy(dst_ptr:*mut u8, src_ptr:*const u8, len:usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
    }
}

fn next_power_of_two(n:u32) -> u32 {
    return 32 - (n - 1).leading_zeros();
}

fn write_chunks_to_file(filename: &str, bytes:&[u8]) -> Result<bool, io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;
    Ok(true)
}

use std::thread;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    let filename = "2.jpg";
    let mut count = 0;
    let mut chunks_cnt:u32 = 0xffff;
    let mut total_size:usize = 0;
   // let mut bytes_buf:*mut u8;
    let mut layout;
    unsafe { layout = MaybeUninit::<Layout>::uninit().assume_init(); };

    let mut bytes_buf;
    unsafe { bytes_buf = MaybeUninit::<*mut u8>::uninit().assume_init(); };
    //let bytes_buf = unsafe {let mut bytes_buf = MaybeUninit::<*mut u8>::uninit();};


    loop {
        let mut buf = [0u8; MAX_CHUNK_SIZE];
        let sock = socket.try_clone().expect("Failed to clone socket");

        //bytes_buf = Vec::new();
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => { // thanks https://doc.rust-lang.org/beta/std/net/struct.UdpSocket.html#method.recv_from
                total_size += size;
                let packet_index:usize = (buf[0] as usize) << 8 | buf[1] as usize;
                if count == 0 {
                    chunks_cnt = (buf[2] as u32) << 8 | buf[3] as u32;
                    //bytes_buf = vec![0; MAX_CHUNK_SIZE * chunks_cnt as usize];
                    let n:usize = 0x10000 << next_power_of_two(chunks_cnt);
                   // assert_eq!(n.count_ones(), 1); // can check with this function that n is aligned on power of 2
                    unsafe {
                         layout = Layout::from_size_align_unchecked(n, mem::align_of::<u8>());
                         bytes_buf = alloc(layout);



                        //*(bytes_buf as *mut u8) = 42;
                       // assert_eq!(*(ptr as *mut u16), 42);
                    }

                    println!("Memory allocated: {:x}", n);

                /*   bytes_buf = Vec::with_capacity(n);
                    unsafe { bytes_buf.set_len(n); }*/
                }

                 unsafe { let dst_ptr = bytes_buf.offset((packet_index*MAX_CHUNK_SIZE) as isize);
                      //     let src_ptr = buf.offset((AG_HEADER) as isize);
                        memcpy(dst_ptr, &buf[AG_HEADER], size-AG_HEADER); // TODO: convert &buf[AG_HEADER] to PTR
                  };

                /*
                if packet_index == 0 { // make sure got the very first chunk
                    println!("First packet"); //
                    unsafe {
                        let bytes =  std::slice::from_raw_parts(bytes_buf, total_size);
                        println!("File bytes{:?}", &bytes); //
                    };

                 }*/
               // let bytes = unsafe { std::slice::from_raw_parts(bytes_buf, total_size) };
                 //           println!("File bytes{:?}", bytes);
                /*
                unsafe {
                    let dst_ptr = &mut bytes_buf[packet_index*MAX_CHUNK_SIZE] as *mut u8;
                    let src_ptr = &buf[AG_HEADER] as *const u8;
                    std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, size-AG_HEADER);
                }*/
                //bytes_buf[packet_index*MAX_CHUNK_SIZE..packet_index*MAX_CHUNK_SIZE+size-AG_HEADER].copy_from_slice(&buf[AG_HEADER..size]);

                thread::spawn(move || {
                    //let s = String::from_utf8_lossy(&buf);
                    println!("receiving packet {} from: {}", packet_index, src);
                    sock.send_to(&buf, &src).expect("Failed to send a response");
                });
                println!("count: {}", count);
                count+=1;
            }
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
         if count == chunks_cnt { // all chunks have been collected, write bytes to file
            let bytes = unsafe { std::slice::from_raw_parts(bytes_buf, total_size) };
            //println!("File bytes{:?}", bytes); ///
            let result = write_chunks_to_file(filename, &bytes); // [0x34,0x32].to_vec()
            match result {
                Ok(true) => println!("Succesfully created file: {}", true),
                Ok(false) => println!("Could not create file: {}", false),
                Err(e) => println!("Error: {}", e),
            }
            count = 0;
            total_size = 0;
            unsafe { dealloc(bytes_buf, layout); }
        }
    }
}
