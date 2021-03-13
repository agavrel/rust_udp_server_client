### Simple UDP server/client in Rust

#### About [User Data Protocol](https://en.wikipedia.org/wiki/User_Datagram_Protocol)

> UDP is suitable for purposes where error checking and correction are either not necessary or are performed in the application; UDP avoids the overhead of such processing in the protocol stack. Time-sensitive applications often use UDP because dropping packets is preferable to waiting for packets delayed due to retransmission, which may not be an option in a real-time system.[1]

##### UDP max data length

UDP packet is limited to 64kB (65536), however we need to take into account that each UDP's packet  
also has a UDP header of 8 bytes as well as an IP header of 20 bytes. Hence MAX_DATA_LENGTH is  
limited to 65,507 bytes, see the following code:

```rs
const UDP_HEADER: u16 = 8;
const IP_HEADER: u16 = 20;
const MAX_DATA_LENGTH: u16 = 64 * 1024 - UDP_HEADER - IP_HEADER;
```