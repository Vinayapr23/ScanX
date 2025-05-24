use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn grab_banner(stream: &mut TcpStream, port: u16) -> Option<String> {
    match port {
        80 | 8080 | 8000 => {
            // HTTP - send simple GET request
            let _ = stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await;
            let mut buf = [0; 512];
            match tokio::time::timeout(std::time::Duration::from_secs(2), stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => Some(String::from_utf8_lossy(&buf[..n]).to_string()),
                _ => None,
            }
        }
        22 => {
            // SSH - server sends banner immediately
            let mut buf = [0; 128];
            match tokio::time::timeout(std::time::Duration::from_secs(2), stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => Some(String::from_utf8_lossy(&buf[..n]).to_string()),
                _ => None,
            }
        }
        6379 => {
            // Redis - send ping command
            let _ = stream.write_all(b"*1\r\n$4\r\nPING\r\n").await;
            let mut buf = [0; 128];
            match tokio::time::timeout(std::time::Duration::from_secs(2), stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => Some(String::from_utf8_lossy(&buf[..n]).to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}
