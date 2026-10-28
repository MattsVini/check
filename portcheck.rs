// portcheck.rs - checks whether a TCP port is open on a host.
// Build: rustc -O portcheck.rs
// Usage: ./portcheck <host> <port> [timeout_seconds]
// Exit codes: 0 = open, 1 = closed/unreachable, 2 = invalid usage

use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::process::exit;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <host> <port> [timeout_seconds]", args[0]);
        exit(2);
    }

    let host = &args[1];

    // Port must be a number between 1 and 65535 (u16 already caps it at 65535).
    // Leading zeros are fine: "080" is read as 80.
    let port: u16 = match args[2].parse() {
        Ok(p) if p > 0 => p,
        _ => {
            eprintln!("Invalid port: {}", args[2]);
            exit(2);
        }
    };

    // Timeout is optional (default 3s), but if given it must be a number > 0.
    // An invalid value is an error, never silently replaced by the default.
    let timeout_secs: u64 = match args.get(3) {
        Some(t) => match t.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("Invalid timeout: {}", t);
                exit(2);
            }
        },
        None => 3,
    };
    let timeout = Duration::from_secs(timeout_secs);

    // Resolve the host name (DNS) into one or more socket addresses.
    let addrs = match (host.as_str(), port).to_socket_addrs() {
        Ok(a) => a,
        Err(e) => {
            println!("[FAIL] could not resolve {}: {}", host, e);
            exit(1);
        }
    };

    for addr in addrs {
        let start = Instant::now();
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            println!(
                "[OK]   {}:{} is open ({} ms)",
                host,
                port,
                start.elapsed().as_millis()
            );
            exit(0);
        }
    }

    println!("[FAIL] {}:{} is closed or unreachable", host, port);
    exit(1);
}
