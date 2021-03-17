use std::io::Read;
use std::net::UdpSocket;
use std::io;
use std::fs;
use tiny_keccak::Sha3; // to encrypt file
use tiny_keccak::Hasher;

const UDP_HEADER: usize = 8;
const IP_HEADER: usize = 20;
const AG_HEADER: usize = 4;
const MAX_DATA_LENGTH: usize = (64 * 1024 - 1) - UDP_HEADER - IP_HEADER;
const MAX_CHUNK_SIZE: usize = MAX_DATA_LENGTH - AG_HEADER;

pub fn get_chunks_from_file(mut filename: String,total_size: &mut usize) -> Result<Vec<Vec<u8>>, io::Error> {
    filename.pop(); // get read of the trailing '\n' in user input.
    let mut file = std::fs::File::open(filename)?;
    let mut list_of_chunks = Vec::new();

    loop {
        let mut chunk = Vec::with_capacity(MAX_CHUNK_SIZE);
        let n = file.by_ref().take(MAX_CHUNK_SIZE as u64).read_to_end(&mut chunk)?;
        *total_size += n;
        if n == 0 {
            break;
        }
        list_of_chunks.push(chunk);
        if n < MAX_CHUNK_SIZE {
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
    let mut nb = 0; // total number of chunks to be sent
    socket_addr.pop();
    println!("{}", socket_addr);
    socket.connect(socket_addr).expect("Could not connect to server");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        //println!("{}", input);
        let mut total_size: usize = 0;
        let result: Result<Vec<Vec<u8>>, io::Error> = get_chunks_from_file(input, &mut total_size); // set total_size at the same time
        match result {
            Ok(ref chunks) => {
               // socket.send(input.as_bytes()).expect("Failed to write to server"); // send file
                nb = chunks.len() as u16;
                //input.as_bytes();
                let header: &mut[u8;4] = &mut[0,0,(nb >> 8) as u8,(nb & 0xff) as u8];
                let mut index: u16 = 0;
                for chunk in chunks.iter() {
                    header[0] = (index >> 8) as u8;   // 0xFF..
                    header[1] = (index & 0xff) as u8; // 0x..FF
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
            Err(ref e) => println!("Error: {}", e),
        }

        match socket.recv_from(&mut buffer) {
            Ok((size, _src)) => {
                match result {
                    Ok(ref chunks) => {
                        unsafe {
                            let missing_indexes : Vec<u16> = (buffer[..size].align_to::<u16>().1).to_vec();
                            let header2: &mut[u8;4] = &mut[0,0,(nb >> 8) as u8,(nb & 0xff) as u8];
                            for missing_index in missing_indexes.iter() {
                                let index = missing_index >> 8 | (missing_index & 0xff) << 8; // need to switch bytes because of little endian
                                println!( "Chunk {} not received by peer, resending...", index);
                                header2[0] = (index >> 8) as u8;   // 0xFF..
                                header2[1] = (index & 0xff) as u8; // 0x..FF
                                let missing_chunk = &chunks[index as usize];
                                let data:Vec<u8> = [header2.as_ref(), missing_chunk].concat();
                                socket.send(&data).expect("Failed to write to server");
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e)
                }
                //print!( "{}",str::from_utf8(&buffer).expect("Could not write buffer as string"));
              //  println!( "Chunk not received by server {:?}", &buffer);
            },
            Err(e) => {
              eprintln!("couldn't read into buffer: {}", e);
            }

        }
    }

}
