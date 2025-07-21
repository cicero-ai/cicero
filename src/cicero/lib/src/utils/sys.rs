
use dirs;
use std::{env, fs};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::net::UdpSocket;
use local_ip_address::local_ip;
use log::error;


/// Retrieves the application's data directory.
/// Prefers the `CICERO_DATADIR` environment variable if set, otherwise falls back to
/// `<user_data_dir>/cicero`. Returns an owned string with trailing separators removed.
pub fn get_datadir() -> String {
    env::var("CICERO_DATADIR")
        .unwrap_or_else(|_| {
            let data_dir = dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .to_string_lossy()
                .into_owned();
            format!("{}/cicero", data_dir.trim_end_matches(MAIN_SEPARATOR))
        })
        .trim_end_matches(MAIN_SEPARATOR)
        .to_string()
}

/// Finds the library directory containing `nlu/en/nlu.dat`.
/// Checks `CICERO_LIBDIR`, current directory, and common system paths.
/// Returns `None` if no valid directory is found.
pub fn get_libdir() -> Option<String> {
    let nlu_file = "nlu/en/nlu.dat";

    // Check env var
    if let Ok(dir) = env::var("CICERO_LIBDIR") {
        let path = Path::new(&dir).join(nlu_file);
        if path.exists() {
            return Some(dir.trim_end_matches(MAIN_SEPARATOR).to_string());
        }
        error!("Invalid CICERO_LIBDIR, missing {}: {}", nlu_file, dir);
    }

    // Check cwd and cwd/lib
    if let Ok(cwd) = env::current_dir() {
        for dir in [cwd.clone(), cwd.join("lib")] {
            let path = dir.join(nlu_file);
            if path.exists() {
                return Some(path.to_string_lossy().trim_end_matches(MAIN_SEPARATOR).to_string());
            }
        }
    }

    // System paths (platform-aware)
    let system_dirs = if cfg!(unix) {
        vec!["/usr/lib/cicero", "/usr/local/lib/cicero", "/var/lib/cicero"]
    } else {
        vec![] // Add Windows/macOS paths if needed
    };

    for dir in system_dirs {
        if Path::new(dir).join(nlu_file).exists() {
            return Some(dir.to_string());
        }
    }

    None
}

/// Determines the local IP address by attempting a UDP connection to a remote IP.
/// Returns `None` if binding or connecting fails (e.g., network unavailable).
pub fn determine_ip_address(remote_ip: &str) -> Option<String> {
    if let Ok(ip) = local_ip() { return Some(ip.to_string()); }




    const ANY_ADDR: &str = "0.0.0.0:0";
    let socket = UdpSocket::bind(ANY_ADDR).ok()?;
    socket.connect(remote_ip).ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}


/// Ensures the parent directory of the given file path exists.
/// Creates it if missing; logs and exits on failure.
pub fn prepare_parent_dir(filename: &str) {
    let parent = match Path::new(filename).parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => return, // No parent or root path, nothing to do
    };

    if parent.exists() {
        return;
    }

    if let Err(e) = fs::create_dir_all(parent) {
        error!("Failed to create parent dir for {}: {}", filename, e);
        std::process::exit(1);
    }
}


/// Retrieves the application version from Cargo environment variables.
/// Returns a tuple of (major, minor, patch) or panics if not in a Cargo build context.
pub fn get_version() -> (String, String, String) {
    let major = env::var("CARGO_PKG_VERSION_MAJOR").expect("Missing CARGO_PKG_VERSION_MAJOR");
    let minor = env::var("CARGO_PKG_VERSION_MINOR").expect("Missing CARGO_PKG_VERSION_MINOR");
    let patch = env::var("CARGO_PKG_VERSION_PATCH").expect("Missing CARGO_PKG_VERSION_PATCH");
    (major, minor, patch)
}


