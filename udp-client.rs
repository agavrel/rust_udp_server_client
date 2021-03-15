const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const AG_HEADER: usize = 4;
const MAX_DATA_LENGTH: usize = (64 * 1024 - 1) - UDP_HEADER - IP_HEADER;

use std::io::Read;
use std::net::UdpSocket;
use std::io;
use std::fs;
use std::net::SocketAddr;

pub fn get_chunks_from_file(mut filename: String,total_size: &mut usize) -> Result<Vec<Vec<u8>>, io::Error> {
    filename.pop(); // get read of the trailing '\n' in user input.
    let mut file = std::fs::File::open(filename)?;
    let mut list_of_chunks = Vec::new();
    let chunk_size = MAX_DATA_LENGTH - AG_HEADER;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)?;
        *total_size += n;
        if n == 0 {
            break;
        }
        list_of_chunks.push(chunk);
        if n < chunk_size {
            break;
        }
    }
    Ok(list_of_chunks)
}

fn main()  -> Result<(), Box<dyn std::error::Error + 'static>>  {
    // let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    let socket = UdpSocket::bind("0.0.0.0:8000").expect("Could not bind client socket");
    let mut buffer = [0u8; MAX_DATA_LENGTH];

    // socket.connect("127.0.0.1:8888").expect("Could not connect to server");
    let mut socket_addr: String = fs::read_to_string("address.txt")?;
    socket_addr.pop();
    println!("{}", socket_addr);
    socket.connect(socket_addr).expect("Could not connect to server");
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        //println!("{}", input);
        let mut total_size: usize = 0;
        let result: Result<Vec<Vec<u8>>, io::Error> = get_chunks_from_file(input, &mut total_size); // : Result<u8:u8>


        match result {
            Ok(chunks) => {
               // socket.send(input.as_bytes()).expect("Failed to write to server"); // send file
                let nb: u16 = chunks.len() as u16;
                let mut index: u16 = 0;
                let header: &mut[u8;4] = &mut[
                    (index >> 8) as u8,
                    (index & 0xff) as u8,
                    (nb >> 8) as u8,
                    (nb & 0xff) as u8,

                ]; //input.as_bytes();

                for chunk in chunks.iter() {
                    header[1] = (index & 0xff) as u8;
                    header[0] = (index >> 8) as u8;
                    let data:Vec<u8> = [header.as_ref(), chunk].concat();
                   // println!("Chunk {} BYTES\n {:?}", index, chunk);
                    println!("Chunk {} sent", index);
               /*     println!(
                        "size: {} FILE {:?} of {} BYTES\n {:?}",
                        total_size,
                        (header[0] as u16) << 8 | header[1] as u16,
                        nb - 1,
                        [0]
                    );*/
                    socket.send(&data).expect("Failed to write to server");

                    index += 1;
                }
            }
            Err(e) => println!("Error: {}", e),
        }

        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                println!( "Chunk not received by server {:?}", &buffer[..size]);
                //print!( "{}",str::from_utf8(&buffer).expect("Could not write buffer as string"));
              //  println!( "Chunk not received by server {:?}", &buffer);
            },
            Err(e) => {
              eprintln!("couldn't read into buffer: {}", e);
            }

        }
    }

}
