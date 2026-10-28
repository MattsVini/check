# Connectivity Checks

Small learning project: command-line tools that check whether a server is reachable, written in **Bash** and in **Rust**. I'm building them while rebuilding my homelab, to practice Linux, networking and automation.

## What's inside

| Tool | What it does | Bash | Rust |
|---|---|---|---|
| **portcheck** | Checks if a TCP port is open on a host | `portcheck.sh` | `portcheck.rs` |
| **sshcheck** | Connects over SSH with a key and prints hostname, uptime, disk and memory | `sshcheck.sh` | `sshcheck.rs` |

## Usage

```bash
# Bash
chmod +x portcheck.sh sshcheck.sh
./portcheck.sh 192.168.0.10 22
./sshcheck.sh user@192.168.0.10

# Rust (only needs rustc)
rustc -O portcheck.rs && ./portcheck 192.168.0.10 22
rustc -O sshcheck.rs && ./sshcheck user@192.168.0.10
```

Both tools exit with `0` on success, `1` on failure and `2` on invalid usage, so they can be used inside other scripts.

## How it works

- **portcheck** opens a TCP connection with a timeout (default 3 seconds). In Bash it uses the built-in `/dev/tcp`; in Rust it uses `TcpStream::connect_timeout`.
- **sshcheck** calls the system `ssh` client with safe options: `BatchMode=yes` (never asks for a password, so only keys work), `ConnectTimeout=5`, and `StrictHostKeyChecking=accept-new` (accepts a new host once, but fails if its key changes later).

## What I'm learning with this

- How TCP connections and timeouts work
- SSH key authentication and why it is safer than passwords
- Writing Bash scripts with argument validation and exit codes
- The same tool in a shell script and in a compiled language
- Reading error messages and fixing edge cases (for example, invalid ports and timeouts)


