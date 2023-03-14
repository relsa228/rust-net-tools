mod input_validation_module;
mod network_module;

use std::env;
use crate::input_validation_module::parsing_input_ports;
use term;

fn main() {
    let mut terminal = term::stdout().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        terminal.fg(term::color::BRIGHT_RED).unwrap();
        println!("~~Error~~\nToo few args, use -h to get help");
        terminal.reset().unwrap();
        return;
    }
    let f_arg = args[1].clone();
    if f_arg == "-h" || f_arg == "--help" {
        terminal.fg(term::color::CYAN).unwrap();
        println!("DesuScan help:3\n");
        terminal.fg(term::color::MAGENTA).unwrap();
        println!("----- INPUT FORMAT: desuscan [ip (you can use 'local' to scan your local port)] -p .. \
           -op (optional) .. -----\
           \n\n-h or --help : for call help\
           \n-v or --version : for call info about version\
           \n-p or --port : for set the investigated ports (use 'all' to scan all 65535 ports)\
           \n-pro or --protocol : value is 'udp' or 'tcp' (default: tcp), use this flag to choose port \
           scanning port type\
           \n-op or --option : now you have two options - 'avo' and 'uno' it mean 'AVailable Only'
and 'UNavailable Only', by default desuscan print all results
           \n");
        terminal.fg(term::color::CYAN).unwrap();
        println!("Be careful and enjoy you use, desu >_<");
        terminal.reset().unwrap();
        return;
    }
    if f_arg == "-v" || f_arg == "--version" {
        terminal.fg(term::color::MAGENTA).unwrap();
        println!("DesuScan version check\nvesion: 0.1.0\ncargo version: 1.64.0");
        terminal.fg(term::color::YELLOW).unwrap();
        println!("\ndev by _relsa <3");
        terminal.reset().unwrap();
        return;
    }

    let mut host = "";
    if f_arg == "local" {
        host = "127.0.0.1";
    }
    else if input_validation_module::ip_check(f_arg.trim()) {
        host = f_arg.trim();
    }

    let mut option: String = String::from("");
    let mut protocol: String = String::from("");
    let mut ports: String = String::from("");
    let mut op_flag = false;
    let mut pro_flag = false;
    let mut port_flag = false;

    for arg in args {
        if arg == "-pro" || arg == "--protocol" {
            pro_flag = true;
            port_flag = false;
            op_flag = false;
        }
        else if arg == "-p" || arg == "--port" {
            op_flag = false;
            pro_flag = false;
            port_flag = true;
        }
        else if arg == "-op" || arg == "--option" {
            op_flag = true;
            pro_flag = false;
            port_flag = false;
        }
        else if op_flag {
            if arg != "avo" && arg != "uno" && !arg.is_empty() {
                terminal.fg(term::color::BRIGHT_RED).unwrap();
                println!("~~Error~~\nInvalid option, use -h to get help");
                terminal.reset().unwrap();
                return;
            }
            option = arg;
        }
        else if port_flag {
            if arg.clone().as_str().to_owned().trim() == "all" {
                ports = "0-65534,".to_string();
            }
            else if arg.clone().as_str().to_owned().trim().ends_with(",") {
                ports += (arg.clone().as_str().trim().to_owned() + " ".trim()).as_str()
            }
            else {
                ports += (arg.clone().as_str().trim().to_owned() + ", ".trim()).as_str()
            }
        }
        else if pro_flag {
            if arg != "tcp" && arg != "udp" {
                terminal.fg(term::color::BRIGHT_RED).unwrap();
                println!("~~Error~~\nInvalid protocol, use -h to get help");
                terminal.reset().unwrap();
                return;
            }
            protocol = arg;
        }
    }
    if ports.is_empty(){
        terminal.fg(term::color::BRIGHT_RED).unwrap();
        println!("~~Error~~\nBaka! You must enter some ports!");
        terminal.reset().unwrap();
    }
    let mut protocol_arg: u8 = 1;
    if protocol == "udp" {
        protocol_arg = 0;
    }
    let mut option_arg: u8 = 0;
    if option == "avo" {
        option_arg = 1
    }
    if option == "uno" {
        option_arg = 2;
    }
    network_module::scan_init(host,
                              parsing_input_ports(ports.trim()),
                              protocol_arg,
                              option_arg);
}
