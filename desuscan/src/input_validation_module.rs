use std::net::IpAddr;
use std::str::FromStr;


pub(crate) fn ip_check (ip_adress: &str) -> bool {
    let mut terminal = term::stdout().unwrap();
    return match IpAddr::from_str(ip_adress) {
        Ok(_) => true,
        Err(_) => {
            terminal.fg(term::color::BRIGHT_RED).unwrap();
            println!("~~Error~~\nIncorrect ip address! Please use IPv4 or IPv6, desu:3");
            terminal.reset().unwrap();
            false
        }
    };
}

pub(crate) fn parsing_input_ports(ports: &str) -> Vec<u16> {
    let mut terminal = term::stdout().unwrap();
    let mut result_vector: Vec<u16> = Vec::new();
    let mut buffer_f: String = String::from("");
    let mut buffer_s: String = String::from("");
    let mut buffer_change: bool = false;
    for symb in ports.chars() {
        if symb.is_numeric() {
            match !buffer_change {
                true => buffer_f += symb.encode_utf8(&mut [0; 4]),
                false => buffer_s += symb.encode_utf8(&mut [0; 4])
            }
        }
        else if symb == ',' || symb == '\n' {
            match !buffer_change {
                true => {
                    match buffer_f.parse::<u16>() {
                        Ok(s) => result_vector.push(s),
                        Err(_) => {
                            terminal.fg(term::color::BRIGHT_YELLOW).unwrap();
                            println!("~~Warning~~\nOne of the port is incorrect! It will be \
                        excluded from the search list -_-");
                            terminal.reset().unwrap();
                        }
                    };
                }
                false => {
                    match buffer_s.trim().parse::<u16>() {
                        Ok(mut s) => {
                            s += 1;
                            for val in buffer_f.trim().parse().unwrap()..s {
                                result_vector.push(val);
                            };
                        }
                        Err(_) => {
                            terminal.fg(term::color::BRIGHT_YELLOW).unwrap();
                            println!("~~Warning~~\nOne of the port is incorrect! It will be \
                        excluded from the search list -_-");
                            terminal.reset().unwrap();
                        }
                    };
                    buffer_change = false;
                    buffer_s.clear();
                }
            }
            buffer_f.clear();
        }
        else if symb == '-' {
            buffer_change = true;
        }
    };
    return result_vector;
}