use std::net::{TcpStream, UdpSocket, SocketAddr, TcpListener, IpAddr, Ipv4Addr};


pub(crate) fn scan_init(host: &str, ports: Vec<u16>, protocol: u8, option: u8) {
    match protocol {
        0 => port_scan_udp(host, ports, option),
        1 => port_scan_tcp(host, ports, option),
        _ => println!("WTF input")
    }
}

fn port_scan_tcp(host: &str, ports: Vec<u16>, option: u8) {
    let mut terminal = term::stdout().unwrap();

    if ports.is_empty() == false {
        terminal.fg(term::color::MAGENTA).unwrap();
        println!("Say UwU to DesuScan");
        terminal.reset().unwrap();
    }

    for port in ports {
        let mut ava_port: bool = false;
        let mut uno_port: bool = false;
        let mut ttl: u32 = 0;
        let mut full_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1);

        match TcpStream::connect((host, port)) {
            Ok(s) => {
                ttl = s.ttl().unwrap();
                full_addr = s.local_addr().unwrap();
                if option == 1 || option == 0 {
                    ava_port = true;
                }
            },
            Err(_) => if option == 2 || option == 0 {
                uno_port = true;
            }
        }

        if !ava_port {
            match TcpListener::bind((host, port)) {
                Ok(s) => {
                    ttl = s.ttl().unwrap();
                    full_addr = s.local_addr().unwrap();
                    if option == 1 || option == 0 {
                        ava_port = true;
                    }
                },
                Err(_) => if option == 2 || option == 0 {
                    uno_port = true;
                }
            }
        }

        if ava_port {
            terminal.fg(term::color::BRIGHT_GREEN).unwrap();
            println!("Port {} is available, full adress = {}, type = TCP, \
                TTL = {}", port, full_addr, ttl);
            terminal.reset().unwrap();
        } 
        else if uno_port {
            terminal.fg(term::color::BRIGHT_RED).unwrap();
            println!("Port {} is busy or unavailable", port);
            terminal.reset().unwrap();
        }
    }
}

fn port_scan_udp(host: &str, ports: Vec<u16>, option: u8) {
    let mut terminal = term::stdout().unwrap();

    if ports.is_empty() == false {
        terminal.fg(term::color::MAGENTA).unwrap();
        println!("Say UwU to DesuScan");
        terminal.reset().unwrap();
    }
    
    for port in ports {
        match UdpSocket::bind((host, port)) {
            Ok(s) => {
                let ttl = s.ttl().unwrap();
                let full_addr = s.local_addr().unwrap();
                if option == 1 || option == 0 {
                    terminal.fg(term::color::BRIGHT_GREEN).unwrap();
                    println!("Port {} is available, full adress = {}, type = UDP, \
                        TTL = {}", port, full_addr, ttl);
                    terminal.reset().unwrap();
                }
            },
            Err(_) => if option == 2 || option == 0 {
                    terminal.fg(term::color::BRIGHT_RED).unwrap();
                    println!("Port {} is busy or unavailable", port);
                    terminal.reset().unwrap();
            }
        }
    }
}