#!/usr/bin/env bash
# portcheck.sh - checks whether a TCP port is open on a host.
# Usage: ./portcheck.sh <host> <port> [timeout_seconds]
# Exit codes: 0 = open, 1 = closed/unreachable, 2 = invalid usage

set -u

usage() {
    echo "Usage: $0 <host> <port> [timeout_seconds]" >&2
    exit 2
}

[[ $# -lt 2 ]] && usage

host=$1
port=$2
timeout_s=${3:-3}
if ! [[ $timeout_s =~ ^[0-9]+$ ]]; then
    echo "Invalid timeout: $timeout_s" >&2
    exit 2
fi

if ! [[ $port =~ ^[0-9]+$ ]] || (( 10#$port < 1 || 10#$port > 65535 )); then
    echo "Invalid port: $port" >&2
    exit 2
fi

# /dev/tcp is a Bash built-in feature: opening it creates a TCP connection.
if timeout "$timeout_s" bash -c 'exec 3<>"/dev/tcp/$1/$2"' _ "$host" "$port" 2>/dev/null; then
    echo "[OK]   $host:$port is open"
    exit 0
else
    echo "[FAIL] $host:$port is closed or unreachable"
    exit 1
fi
