# Large file transfer over UDP, server/client in Rust

## About [User Data Protocol](https://en.wikipedia.org/wiki/User_Datagram_Protocol)

> UDP is suitable for purposes where error checking and correction are either not necessary or are performed in the application; UDP avoids the overhead of such processing in the protocol stack. Time-sensitive applications often use UDP because dropping packets is preferable to waiting for packets delayed due to retransmission, which may not be an option in a real-time system.[1]

#### UDP max data length

UDP packet is limited to 64kB (65536), however we need to take into account that each UDP's packet  
also has a UDP header of 8 bytes as well as an IP header of 20 bytes. Hence `MAX_DATA_LENGTH` is limited to 65,507 bytes.

```rs
const UDP_HEADER: u16 = 8;
const IP_HEADER: u16 = 20;
const MAX_DATA_LENGTH: u16 = 64 * 1024 - UDP_HEADER - IP_HEADER;
```

We have to remove bytes from our custom header which decrease our `MAX_CHUNK_SIZE` further by a few bytes (storing total number, index and filename):
```rs
const MAX_CHUNK_SIZE = MAX_DATA_LENGTH - AG_HEADER
```

**The following Program can send file up to about 4Gb** *(65535 (u16) chunks multiplied by chunk_size and divided by 1024^3 to convert from bytes to Gb).*  
This can be extended to much higher limits simply by using extra bytes in the custom header `AG_HEADER`.  

It also provides mechanisms to resend chunks that have failed to be received.

## Setting up server over internet

#### Get public address

```
dig +short myip.opendns.com @resolver1.opendns.com
```

#### Get private address

```
hostname -I | awk -F' ' '{print $1}'
```

#### Setup router to forward port `Port` on our server private address

Go to your router homepage for settings, you will need to enter username and password
```
http://192.168.0.1/
```

---

## TODO List
* Add filename information and detect true fileType based on file magic number
* Make it peer-to-peer
* build tracker server
* GUI for the program, ability to visualize data (vulkan? flutter?)
* Compression of file before being sent *(not sure if worth if for jpg, only 5% saved, 2% for m4a...)*
* Ability to encrypt data before sending (TwoFish?)

---

## Add crates to path
```
export PATH="/home/st4ck/.cargo/bin:$PATH"
```
