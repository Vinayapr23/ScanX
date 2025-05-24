use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::ToSocketAddrs;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Serialize; 

#[derive(Debug, Serialize)] // Add Serialize derive macro
pub struct ScanResult {
    pub port: u16,
    pub open: bool,
    pub banner: Option<String>,
}

pub async fn scan_ports(
    target: &str,
    start: u16,
    end: u16,
    timeout_ms: u64,
    grab_banner: bool,
) -> Vec<ScanResult> {
    let mut results = Vec::new();

    let mut futures = FuturesUnordered::new();

    for port in start..=end {
        let addr = format!("{}:{}", target, port);
        let timeout_dur = Duration::from_millis(timeout_ms);
        let grab = grab_banner;

        futures.push(tokio::spawn(async move {
            match timeout(timeout_dur, TcpStream::connect(&addr)).await {
                Ok(Ok(mut stream)) => {
                    let banner = if grab {
                        
                        crate::banner::grab_banner(&mut stream, port).await
                    } else {
                        None
                    };
                    ScanResult {
                        port,
                        open: true,
                        banner,
                    }
                }
                _ => ScanResult {
                    port,
                    open: false,
                    banner: None,
                },
            }
        }));
    }

    while let Some(res) = futures.next().await {
        if let Ok(scan_res) = res {
            results.push(scan_res);
        }
    }

    results
}
