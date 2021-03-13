const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const AGAVREL_HEADER: usize = 4;
const MAX_CHUNK_SIZE: usize = (64 * 1024 - 1) - UDP_HEADER - IP_HEADER - AGAVREL_HEADER;

use std::net::UdpSocket;
use std::io;
use std::fs::File;
use std::io::prelude::*;

pub fn write_chunks_to_file(filename: &str, bytes:Vec<u8>) -> Result<&str, io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(&bytes)?; // write too many bytes at the end (the content of NULL bytes in the last buffer)
    Ok(filename)
}

use std::thread;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    let filename = "2.jpg";
    let mut count = 0;
    let mut chunks_cnt:u16 = 0xffff;
    let mut  bytes_buf:Vec<u8> = Vec::new();

    loop {
        let mut buf = [0u8; MAX_CHUNK_SIZE];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => { // thanks https://doc.rust-lang.org/beta/std/net/struct.UdpSocket.html#method.recv_from
                if count == 0 {
                    chunks_cnt = (buf[2] as u16) << 8 | buf[3] as u16;
                    bytes_buf = vec![0; MAX_CHUNK_SIZE * chunks_cnt as usize];
                }
                let packet_index:usize = (buf[0] as usize) << 8 | buf[1] as usize;
                bytes_buf[packet_index*MAX_CHUNK_SIZE..packet_index*MAX_CHUNK_SIZE+size-AGAVREL_HEADER].copy_from_slice(&buf[AGAVREL_HEADER..size]);
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
            let result = write_chunks_to_file(filename, bytes_buf); // [0x34,0x32].to_vec()
            match result {
                Ok(filename) => println!("Succesfully created file: {}", filename),
                Err(e) => println!("Error: {}", e),
            }
            count = 0;
        }
    }
}
