use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::net::{SocketAddrV4, TcpStream};
use std::time::Duration;

pub fn banner_grab(target: Ipv4Addr, port: u16, timeout_ms: u64) -> Option<String> {
    let timeout = Duration::from_millis(timeout_ms);
    let addr = SocketAddrV4::new(target, port);

    let mut stream = TcpStream::connect_timeout(&addr.into(), timeout).ok()?;

    stream.set_read_timeout(Some(timeout)).ok()?;
    stream.set_write_timeout(Some(timeout)).ok()?;

    if matches!(port, 80 | 443 | 8000 | 8001 | 8888 | 3000 | 5000) {
        let request: String = format!(
            "HEAD / HTTP/1.0\r\nHost: {}\r\nUser-Agent: rustmap/1.0\r\n\r\n",
            target
        );
        stream.write_all(request.as_bytes()).ok()?;
    };

    let mut buffer = [0u8; 1024];
    let buffer_size = stream.read(&mut buffer).ok()?;

    if buffer_size == 0 {
        return None;
    }

    let raw_data = &buffer[..buffer_size];

    parse_banner(raw_data, port)
}

fn parse_banner(data: &[u8], port: u16) -> Option<String> {
    if matches!(port, 3306 | 5432 | 6379 | 27017) {
        return Some(service_hint(port));
    }

    let text = String::from_utf8_lossy(data);

    if text.starts_with("HTTP/") {
        for line in text.lines() {
            let lower = line.to_lowercase();
            if lower.starts_with("server:") {
                let server = line[7..].trim().to_string();
                return Some(format!("HTTP | {}", server));
            }
        }

        let fallback = text.lines().next().unwrap_or("HTTP").trim();
        return Some(fallback.to_string());
    }

    if text.starts_with("SSH-") {
        let version = text.lines().next().unwrap_or("SSH").trim();
        return Some(version.to_string());
    }

    let first_line = text.lines().map(|l| l.trim()).find(|l| !l.is_empty())?;

    let cleaned: String = first_line
        .chars()
        .filter(|c| c.is_ascii_graphic() || *c == ' ')
        .take(80)
        .collect();

    if cleaned.is_empty() || cleaned.len() < 3 {
        return Some(service_hint(port));
    }

    Some(cleaned)
}

fn service_hint(port: u16) -> String {
    match port {
        3306 => "MySQL".to_string(),
        5432 => "PostgreSQL".to_string(),
        6379 => "Redis".to_string(),
        27017 => "MongoDB".to_string(),
        443 | 8443 => "TLS/HTTPS (no plaintext banner)".to_string(),
        _ => format!("No banner (port {})", port),
    }
}
