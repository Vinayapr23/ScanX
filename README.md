# ScanX - Simple Port Scanner

ScanX is a command-line tool written in Rust that scans a target hostname or IP address for open ports within a specified range. It supports outputting scan results as a table, JSON, or HTML report.

## Features

- Scan ports in a specified range asynchronously
- Optional banner grabbing on common ports
- Multiple output formats: table (console), JSON file, or HTML report

## Prerequisites

Make sure you have Rust and Cargo installed on your system. You can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Installation

Clone this repository and build the project:

```bash
git clone https://github.com/Vinayapr23/ScanX.git
cd ScanX
cargo build --release
```

## Usage

Run ScanX using Cargo with the following syntax:

```bash
cargo run -- --target <target> --start <start_port> --end <end_port> [--banner] [--output <format>]
```

### Arguments

- `--target`: Target hostname or IP to scan (e.g., `scanme.nmap.org` or `127.0.0.1`)
- `--start`: Starting port number
- `--end`: Ending port number
- `--banner`: Enable banner grabbing (optional)
- `--output`: Output format (`table` [default], `json`, `html`)

### Output Files

- If `--output json` is specified, results are saved to `scan_results.json` in the current directory
- If `--output html` is specified, results are saved to `scan_results.html` in the current directory

## Example Command

### Scan with HTML output and banner grabbing

Scan ports 20 to 100 on `scanme.nmap.org`, output as an HTML report with banner grabbing enabled:

```bash
cargo run -- --target scanme.nmap.org --start 20 --end 100 --output html --banner
```
### Save results as JSON

Save results as a JSON file:

```bash
cargo run -- --target scanme.nmap.org --start 20 --end 100 --output json
```

## How It Works

1. Resolves target to IP address
2. Scans the specified port range asynchronously
3. Detects open ports by attempting TCP connections
4. Optionally retrieves banners from some common services if enabled
5. Displays results or exports to the chosen format

## Disclaimer

This tool is intended for educational purposes and authorized security testing only. Users are responsible for ensuring they have permission to scan target systems.