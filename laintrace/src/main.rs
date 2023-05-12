use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::process;
use std::str::FromStr;
use std::time::Duration;

extern crate pnet;
use pnet::packet::icmp::{echo_request, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType};

const DEFAULT_TTL: u8 = 64;
const DEFAULT_TIMEOUT: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <destination>", args[0]);
        process::exit(1);
    }

    let destination = IpAddr::from_str(&args[1]).unwrap_or_else(|_| {
        eprintln!("Invalid destination address");
        process::exit(1);
    });

    let (mut sender, mut receiver) = transport_channel(
        1024,
        TransportChannelType::Layer4(pnet::transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp)),
    )
    .unwrap_or_else(|_| {
        eprintln!("Failed to open transport channel");
        process::exit(1);
    });

    let mut ttl = 1;
    loop {
        sender.set_ttl(ttl).unwrap();

        let mut packet = [0u8; 64];
        let mut echo_packet = echo_request::MutableEchoRequestPacket::new(&mut packet).unwrap();
        echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
        echo_packet.set_sequence_number(ttl as u16);
        let checksum = pnet::util::checksum(echo_packet.packet(), 1);
        echo_packet.set_checksum(checksum);

        sender.send_to(echo_packet, destination).unwrap();

        let start_time = std::time::Instant::now();
        let mut iter = icmp_packet_iter(&mut receiver);
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT);

        if let Some((packet, addr)) = iter.next_with_timeout(timeout).unwrap() {
            if packet.get_icmp_type() == IcmpTypes::EchoReply && addr == destination {
                println!("{} hops max, {} ms", ttl, start_time.elapsed().as_millis());
                break;
            } else if packet.get_icmp_type() == IcmpTypes::TimeExceeded {
                println!("{} {}", ttl, addr);
            }
        } else {
            println!("{} *", ttl);
        }

        ttl += 1;
        if ttl > DEFAULT_TTL {
            break;
        }
    }
}