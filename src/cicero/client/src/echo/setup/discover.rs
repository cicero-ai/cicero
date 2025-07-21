use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Perform discovery, try to find Cicero back-end on local network
pub fn run(local_ip: String) -> Option<(String, u16)> {

    // Check localhost
    if let Some((ip, port)) = check_local() {
        return Some((ip, port))
    }

    // Parse local IP
    let ip_parts: Vec<u8> = local_ip.split('.').filter_map(|s| s.parse().ok()).collect();
    if ip_parts.len() != 4 { return None; }

    // Assume /24 subnet (e.g., 192.168.0.x)
    let base_ip = Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], 0);
    let port = 7511;

    // Create Tokio runtime
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return None,
    };

    // Run the scan using the runtime
    let result = rt.block_on(scan(base_ip, port));

    // Process results
    if let Some(addr) = result {
        return Some((addr.ip().to_string(), addr.port()));
    }

    None
}

/// Check localhost first, then scan local network if needed
pub fn check_local() -> Option<(String, u16)> {

    let localhost = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7511);
    let timeout_duration = Duration::from_millis(100);

    // Try localhost synchronously
    if let Ok(_) = std::net::TcpStream::connect_timeout(&localhost, timeout_duration) {
        return Some(("127.0.0.1".to_string(), 7511));
    }
    None
}

/// Async function to scan the network
async fn scan(base_ip: Ipv4Addr, port: u16) -> Option<SocketAddr> {
    let timeout_duration = Duration::from_millis(100);
    let mut tasks = Vec::new();

    // Scan 1-254 (skip 0 and 255)
    for i in 1..255 {
        let ip = Ipv4Addr::new(base_ip.octets()[0], base_ip.octets()[1], base_ip.octets()[2], i);
        let addr = SocketAddr::new(IpAddr::V4(ip), port);
        tasks.push(tokio::spawn(async move {
            match timeout(timeout_duration, TcpStream::connect(addr)).await {
                Ok(Ok(_)) => Some(addr), // Port open
                _ => None,
            }
        }));
    }

    // Check results
    for task in tasks {
        match task.await {  // Handle the JoinHandle Result
            Ok(Some(addr)) => return Some(addr),  // Inner Some means we found a connection
            Ok(None) => continue,                // Inner None means no connection
            Err(_) => continue,                  // Join error, skip to next task
        }
    }

    None
}

