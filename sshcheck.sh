#!/usr/bin/env bash
# sshcheck.sh - connects to a server over SSH (key authentication only)
# and prints a quick health report: hostname, uptime, disk and memory.


set -u

usage() {
    echo "Usage: $0 <user@host> [port] [identity_file]" >&2
    exit 2
}

[[ $# -lt 1 ]] && usage

target=$1
port=${2:-22}

if ! [[ $port =~ ^[0-9]+$ ]] || (( port < 1 || port > 65535 )); then
    echo "Invalid port: $port" >&2
    exit 2
fi

ssh_opts=(
    -p "$port"
    -o BatchMode=yes                    
    -o ConnectTimeout=5                 
    -o StrictHostKeyChecking=accept-new 
    -o PasswordAuthentication=no
)

if [[ -n ${3:-} ]]; then
    ssh_opts+=(-i "$3")
fi

echo "Connecting to $target:$port ..."

# The remote commands run on the server; the output comes back here.
if ssh "${ssh_opts[@]}" "$target" '
    echo "Host:   $(hostname)"
    echo "Uptime: $(uptime -p)"
    echo "Disk:   $(df -h / | awk "NR==2 {print \$3\" used of \"\$2\" (\"\$5\")\"}")"
    echo "Memory: $(free -h | awk "/^Mem:/ {print \$3\" used of \"\$2}")"
'; then
    echo "[OK]   SSH connection to $target succeeded"
    exit 0
else
    echo "[FAIL] could not connect to $target (host down, wrong port, or key not authorized)"
    exit 1
fi
