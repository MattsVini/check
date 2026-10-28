// sshcheck.rs - connects to a server over SSH (key authentication only)
// and prints a quick health report: hostname, uptime, disk and memory.
// It calls the system's `ssh` client, so it uses your existing keys and config.
//
// Build: rustc -O sshcheck.rs
// Usage: ./sshcheck <user@host> [port] [identity_file]
// Exit codes: 0 = ok, 1 = connection/auth failed, 2 = invalid usage

use std::env;
use std::process::{exit, Command};

const REMOTE_REPORT: &str = r#"
echo "Host:   $(hostname)"
echo "Uptime: $(uptime -p)"
echo "Disk:   $(df -h / | awk 'NR==2 {print $3" used of "$2" ("$5")"}')"
echo "Memory: $(free -h | awk '/^Mem:/ {print $3" used of "$2}')"
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <user@host> [port] [identity_file]", args[0]);
        exit(2);
    }

    let target = &args[1];
    let port: u16 = match args.get(2) {
        Some(p) => match p.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("Invalid port: {}", p);
                exit(2);
            }
        },
        None => 22,
    };

    let mut cmd = Command::new("ssh");
    cmd.arg("-p").arg(port.to_string())
        .args(["-o", "BatchMode=yes"]) 
        .args(["-o", "ConnectTimeout=5"])
        .args(["-o", "StrictHostKeyChecking=accept-new"])
        .args(["-o", "PasswordAuthentication=no"]);

    if let Some(identity) = args.get(3) {
        cmd.arg("-i").arg(identity);
    }

    cmd.arg(target).arg(REMOTE_REPORT);

    println!("Connecting to {}:{} ...", target, port);

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("[OK]   SSH connection to {} succeeded", target);
        }
        Ok(_) => {
            println!(
                "[FAIL] could not connect to {} (host down, wrong port, or key not authorized)",
                target
            );
            exit(1);
        }
        Err(e) => {
            eprintln!("Could not run the `ssh` client: {}", e);
            exit(1);
        }
    }
}
