
use falcon_cli::*;
use indexmap::IndexMap;
use cicero::utils::sys;
use crate::apollo::config::NetworkMode;
use log::warn;

/// Get network mode and Apollo host
pub fn determine() -> (NetworkMode, String) {

    // Start options
    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Local Machine - This machine only (127.0.0.1".to_string());

    // Local network address
    if let Some(local_ip) = sys::determine_ip_address("192.168.0.1:80") {
        options.insert("2".to_string(), format!("Local Network - Anyone on this local network / router ({})", local_ip));
    } else {
        warn!("Unable to determine local router IP address.");
    }
    options.insert("3".to_string(), "Public Internet - Connect from anywhere (0.0.0.0)".to_string());

    // Get network mode
    cli_header("Network Mode");
    let mode = cli_get_option("Where will people be connecting to and using Cicero from? ", &options);

    let (mut network_mode, mut ipaddr) = (NetworkMode::local, "127.0.0.1".to_string());
    if mode == "2".to_string() {
        network_mode = NetworkMode::lan;
        ipaddr = sys::determine_ip_address("192.168.0.1:80").unwrap();
    } else if mode == "3".to_string() {
        network_mode = NetworkMode::internet;
        ipaddr = "0.0.0.0".to_string();
    }

    (network_mode, ipaddr)
}



