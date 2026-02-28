mod args;
mod ports;
mod scanner;
mod style;
mod target;
mod banner;

use args::ScanArguments;
use ports::parse_ports;
use scanner::scan_ports;
use style::banner;
use target::resolve_target;

use crossterm::style::{Color, Stylize};
use std::net::Ipv4Addr;
use std::process::exit;

fn main() -> () {
    banner();
    let args: ScanArguments = ScanArguments::arg_parse();

    // Resolve Target
    println!("Resolving target: {}", args.target_ip);
    let ip: Ipv4Addr = match resolve_target(&args.target_ip) {
        Ok(ipaddr) => ipaddr,
        Err(e) => {
            eprintln!("[-] {}", e);
            exit(1);
        }
    };

    let ports = match parse_ports(args.ports.as_deref()) {
        Ok(rng) => rng,
        Err(e) => {
            eprintln!("[-] {}", e);
            exit(1);
        }
    };

    println!("{}", "[*] Starting Scan".with(Color::Cyan));

    let open_ports = scan_ports(ip, ports.clone(), args.threads, args.timeout);

    println!("    {:<10} {}", "PORTS", "STATUS");
    println!("    {}", "-".repeat(20));

    // handling closed ports
    if open_ports.is_empty() {
        // single port and closed
        if ports.start() == ports.end() {
            println!("    {:<10} {}", ports.start(), "CLOSED".red());
        } else {
            println!(
                "[-] No Open Ports in range {} - {}",
                ports.start(),
                ports.end()
            );
        }
    } else {
        for port in open_ports {
            println!("[+] {:<10}    {}", port, "OPEN".green());
        }
    }

    println!("\n{}", "[+] Scan Complete".with(Color::Green));
}
