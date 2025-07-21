
use cicero_sdk::{CiceroPlugin, CiceroPluginMeta};
use cicero_core::CiceroCore;
use std::collections::HashMap;
use libloading::{Library, Symbol};
use serde_derive::{Serialize, Deserialize};
use std::path::{Path, MAIN_SEPARATOR};
use crate::error::Error;
use crate::utils::sys;
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs;
use std::process::Command;
use log::{debug, error, info, warn};
use crate::server::CONFIG;

pub struct PluginManager {
    pub plugins: HashMap<String, Box<dyn CiceroPlugin>>
}


impl PluginManager {

    pub fn new() -> Self {

        // Load core plugin
        let core_plugin: Box<dyn CiceroPlugin> = Box::new(CiceroCore::new());

        // Instantiate mgs
        let mut mgr = Self {
            plugins: HashMap::new()
        };
        mgr.plugins.insert("core".to_string(), core_plugin);

        // Scan installed plugins
        mgr.scan_installed_plugins();

        // Scan developer plugins
        mgr.scan_dev_plugins();

        // Return
        info!("Successfully loaded {} plugins", mgr.plugins.len() - 1);
        mgr
    }

    /// Load plugin
    fn load_plugin(&mut self, lib_path: &Path) -> Result<(), Error> {

        if !lib_path.exists() {
            error!("Trying to load plugin that does not exist, {}", lib_path.display());
            return Err(Error::Generic(format!("Plugin does not exist at, {}", lib_path.display()    )));
        }

        // Load plugin
        unsafe {

            let lib = match Library::new(lib_path) {
                Ok(r) => r,
                Err(e) => return Err(Error::Generic(format!("Unable to load plugin file at {}, error: {}", lib_path.display(), e)))
            };

            // Init plugin
            let plugin_init_func: Symbol<unsafe extern fn() -> Box<dyn CiceroPlugin>> = match lib.get(b"init_plugin") {
                Ok(r) => r,
                Err(e) => return Err(Error::Generic(format!("Unable to obtain init_plugin() function from plugin {}, error: {}", lib_path.display(), e)))
            };

            // Add to loaded plugins
            let plugin: Box<dyn CiceroPlugin> = plugin_init_func();
            let meta = plugin.get_meta();
            self.plugins.insert(meta.slug.clone(), plugin);
            info!("Successfully loaded plugin, {}", meta.name);
        }

        Ok(())
    }

    /// Scan local plugin directory
    fn scan_installed_plugins(&mut self) {

        // Ensure directoy exists
        let dirname = format!("{}/plugins", CONFIG.general.libdir);
        if !Path::new(&dirname).exists() {
            warn!("No plugin directory exists at {}, skipping plugin initialization", dirname);
            return;
        }
        let ext = self.get_lib_extension();

        // Go through sub-directories
        let entries = fs::read_dir(&dirname).unwrap();
        for entry in entries {

            // Check if Rust crate
            let path = entry.unwrap().path();
            if (!path.is_file()) || (!path.ends_with(&ext)) {
                continue;
            }

            // Load plugin
            self.load_plugin(&path);
        }

    }

    /// Scan developer plugins directory
    pub fn scan_dev_plugins (&mut self) {

        // Get current checksums
        let mut checksums: HashMap<String, String> = HashMap::new();
        let checksum_filename = format!("{}/config/dev-plugins.yml", sys::get_datadir());
        if Path::new(&checksum_filename).exists() {
            let yaml_str = fs::read_to_string(&checksum_filename).unwrap();
            checksums = serde_yaml::from_str(&yaml_str).unwrap();
        }

        // Check directory exists
        if CONFIG.general.plugin_dev_dir.is_empty() {
            return;
        } else if !Path::new(&CONFIG.general.plugin_dev_dir).exists() {
            warn!("Developer plugin directory defined, but doesn't exist at {}", CONFIG.general.plugin_dev_dir);
            return;
        }

        // Go through sub-directories
        let entries = fs::read_dir(&CONFIG.general.plugin_dev_dir.clone()).unwrap();
        for entry in entries {

            // Check if Rust crate
            let path = entry.unwrap().path();
            if (!path.is_dir()) || (!path.join("Cargo.toml").exists()) {
                continue;
            }

            // Load Cargo.toml manifest
            let cargo_toml = match cargo_toml::Manifest::from_path(path.join("Cargo.toml")) {
                Ok(r) => r,
                Err(e) => {
                    warn!("Unable to parse Cargo.toml file at {}/Cargo.toml, error: {}", path.display(), e);
                    continue;
                }
            };

            // Get package name
            if cargo_toml.lib.is_none() {
                continue;
            } else if cargo_toml.lib.clone().unwrap().name.is_none() {
                continue;
            }
            let pkg_name = cargo_toml.lib.unwrap().name.unwrap();

            // Ensure Cargo.toml file has 'cicero-sdk' dependency
            if !cargo_toml.dependencies.contains_key("cicero-sdk") {
                warn!("Found crate '{}' within plugin dev directory, but no 'cicero-sdk' dependency, skipping.", pkg_name);
                continue;
            }
            let plugin_file = format!("{}/target/release/lib{}.{}", CONFIG.general.plugin_dev_dir, pkg_name, self.get_lib_extension());

            // Get metadata and checksum of last time plugin compiled
            let mut checksum = String::new();
            if let Ok(metadata) = fs::metadata(&path) {
                let mtime = metadata.modified().expect("Failed to get modified time of plugin root dir");
                let datetime: DateTime<Utc> = mtime.into();
                checksum = datetime.format("%d/%m/%Y %T").to_string();
            } else { 
                warn!("Unable to read metadata from plugin '{}', not re-compiling.", pkg_name);
            }

            // Check checksums, re-compile if needed
            let prev_checksum = match checksums.get(&pkg_name.clone()) {
                Some(r) => r.to_string(),
                None => "".to_string()
            };
            if (*prev_checksum != checksum.to_string() && !checksum.is_empty()) || !Path::new(&plugin_file).exists() {
                self.compile_plugin(&path, &pkg_name.as_str());
            }
            *checksums.entry(pkg_name.clone()).or_default() = checksum.clone();

            // Load plugin
            info!("Found in-development plugin '{}', loading...", pkg_name);
            self.load_plugin(Path::new(&plugin_file));
        }
        debug!("Done checking developer plugin directory");

        // Save plugin file
        let yaml_str = serde_yaml::to_string(&checksums).unwrap();
        fs::write(&checksum_filename, yaml_str).unwrap();
    }

    /// Compile plugin
    fn compile_plugin(&self, path: &Path, name: &str) {

        // Set variables
        let manifest_path = format!("{}", path.join("Cargo.toml").display());
        let target_dir = format!("{}/target", CONFIG.general.plugin_dev_dir);

        // Run build command
        let mut cmd = Command::new("cargo".to_string());
        cmd.args(["build", "--release", "--manifest-path", &manifest_path.as_str(), "--target-dir", &target_dir.as_str()]);
        let output = match cmd.output() {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to build plugin '{}' via cargo, error: {}", name, e.to_string());
                return;
            }
        };

        // Check out status
        if !output.status.success() {
            warn!("Did not receive successful response status from cargo build.  Got {} with (stdout: {}), (stderr: {})", 
                output.status, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        } else {
            info!("Detected changes in plugin '{}', compiled via cargo", name);
        }
    }

    /// Get file extension of plugins based on OS
    fn get_lib_extension(&self) -> &'static str {
        if cfg!(target_os = "linux") {
            return "so";
        } else if cfg!(target_os = "windows") {
            return "dll";
        } else if cfg!(target_os = "macos") {
            return "dylib";
        } else {
            panic!("Unsupported operating system");
        }
    }

}


