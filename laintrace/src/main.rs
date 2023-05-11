use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::{UnixDatagram, UnixListener};
use std::time::{Duration, Instant};

const TTL: u32 = 64;
const PACKET_SIZE: usize = 64;

fn main() {
    let socket = UnixDatagram::unbound().unwrap();
    let listener = UnixListener::bind("/tmp/traceroute.sock").unwrap();
    let mut buffer = [0u8; PACKET_SIZE];

    for ttl in 1..=64 {
        socket.set_ttl(TTL).unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        let start_time = Instant::now();
        socket.send_to(&buffer, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 33434)).unwrap();

        let mut response_buffer = [0u8; PACKET_SIZE];
        match listener.accept() {
            Ok((mut stream, _)) => {
                stream.read_exact(&mut response_buffer).unwrap();
                println!("{}: {:?}", ttl, start_time.elapsed());
            }
            Err(_) => println!("{}: *", ttl),
        }
    }
}