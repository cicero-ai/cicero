use super::HttpClientConfig;
use crate::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use url::Url;

/// Connect to SOCKS5 proxy
pub fn connect(
    sock: &mut TcpStream,
    config: &HttpClientConfig,
    uri: &Url,
    port: &u16,
) -> Result<(), Error> {
    // Hello
    self::hello(sock, config)?;

    // Send request to connect
    self::request(sock, uri, port)?;

    Ok(())
}

/// Send hello to SOCKS5 proxy
fn hello(sock: &mut TcpStream, config: &HttpClientConfig) -> Result<(), Error> {
    // Send greeting
    sock.write_all(&[0x05, 0x01, 0x00]).unwrap();
    sock.flush().unwrap();

    // Read response
    let mut buffer = [0u8; 2];
    sock.read(&mut buffer).unwrap();

    // Check response
    if buffer[1] == 0xFF {
        return Err(Error::Custom(
            "SOCKS5 gave invalid response after initial greeting, no auth methods available."
                .to_string(),
        ));
    } else if buffer[1] == 0x02 {
        self::authenticate(sock, config)?;
        return Err( Error::Custom("Authentication required, but not developed in for atlas-http.  Please raise issue on Github and bug developer, https://github.com/mdizak/rust-atlas-http".to_string()) );
    }

    Ok(())
}

/// Authenticate
fn authenticate(sock: &mut TcpStream, config: &HttpClientConfig) -> Result<(), Error> {
    // Start request
    let mut request = vec![0x01];

    // Username
    request.push(config.proxy_user.len() as u8);
    for c in config.proxy_user.chars() {
        request.push(c as u8);
    }

    // Password
    request.push(config.proxy_password.len() as u8);
    for c in config.proxy_password.chars() {
        request.push(c as u8);
    }

    // Send request
    sock.write_all(&request).unwrap();
    sock.flush().unwrap();

    // Read response
    let mut buffer = [0u8; 2];
    sock.read(&mut buffer).unwrap();

    // Check response
    if buffer[1] != 0x00 {
        return Err(Error::Custom(
            "SOCKS5 proxy authentication failed.  Please check proxy user / pass, and try again."
                .to_string(),
        ));
    }

    Ok(())
}

/// Send request to connect to remote server
fn request(sock: &mut TcpStream, uri: &Url, port: &u16) -> Result<(), Error> {
    // Get addr
    let hostname = format!("{}:{}", uri.host_str().unwrap(), port);
    let mut address = hostname.to_socket_addrs().unwrap();
    let addr = address.next().unwrap();

    // Set request
    let mut request = vec![0x05, 0x01, 0x00];

    // Append IP address to request
    if let SocketAddr::V6(h) = addr {
        request.push(0x04);
        for byte in h.ip().octets() {
            request.push(byte);
        }
    } else if let SocketAddr::V4(h) = addr {
        request.push(0x01);
        for byte in h.ip().octets() {
            request.push(byte);
        }
    } else {
        let host = uri.host_str().unwrap();
        request.push(0x03);
        request.push(host.len() as u8);

        for c in host.chars() {
            request.push(c as u8);
        }
    }

    // Add port
    request.push((addr.port() >> 8) as u8);
    request.push((addr.port() & 0x00FF) as u8);

    // Send request
    sock.write_all(&request).unwrap();
    sock.flush().unwrap();

    // Read response
    let mut buffer = [0u8; 10];
    sock.read(&mut buffer).unwrap();

    // Ipv6, get rid of extra bytes
    if buffer[3] == 0x04 {
        let mut tmp_buffer = [0u8; 12];
        sock.read(&mut tmp_buffer).unwrap();
    }

    // Check response
    if buffer[1] != 0x00 {
        return Err(Error::Custom(
            "Invalid response from SOCKS5 proxy after 'connect' command.".to_string(),
        ));
    }

    Ok(())
}
