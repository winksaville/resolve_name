/*!
 * Resolve a name to an IP address.
 * Written by ChatGPT4o and Wink Saville
 */
use std::env;
use std::net::ToSocketAddrs;

const VERSION: &str = "0.1.0";

fn print_usage() {
    println!("Usage: resolve_name <name>");
    println!("Version: {}", VERSION);
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there is exactly one argument (the program name is always the first argument)
    if args.len() != 2 {
        print_usage();
        return;
    }

    let name = &args[1];

    // Resolve the hostname
    match (name.as_str(), 0).to_socket_addrs() {
        Ok(addrs) => {
            for addr in addrs {
                println!("Resolved address: {}", addr);
            }
        }
        Err(e) => {
            eprintln!("Failed to resolve address: {}", e);
        }
    }
}
