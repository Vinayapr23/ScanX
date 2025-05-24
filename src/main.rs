mod arg;
mod scanner;
mod banner;
mod output;

use arg::Args;
use clap::Parser;
use tokio::net::lookup_host;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Resolve target to IP address using a dummy port (e.g., 80)
    let ip_addr = match lookup_host(format!("{}:80", args.target)).await {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr.ip(),
            None => {
                eprintln!("Failed to resolve target: {}", args.target);
                return;
            }
        },
        Err(e) => {
            eprintln!("DNS lookup failed for {}: {}", args.target, e);
            return;
        }
    };

    println!("Scanning {} (IP: {}) ports {}-{}", args.target, ip_addr, args.start, args.end);

    let results = scanner::scan_ports(&ip_addr.to_string(), args.start, args.end, args.timeout, args.banner).await;

    match args.output.as_str() {
       "json" => {
        let path = "scan_results.json";
        match output::print_json(&results, path) {
            Ok(_) => println!("JSON report saved to {}", path),
            Err(e) => eprintln!("Failed to write JSON report: {}", e),
        }
    }
         "html" => {
        if let Err(e) = output::export_html(&results, "scan_results.html") {
            eprintln!("Failed to write HTML report: {}", e);
        } else {
            println!("HTML report saved to scan_results.html");
        }
    }
        _ => output::print_table(&results),
    }
}
