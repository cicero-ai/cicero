
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use nix::libc;
use std::path::Path;
use std::{fs, env};
use crate::utils::sys;
use log::{info, warn, error};
use crate::server::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessManager {
    pub echo: u32,
    pub apollo: Vec<u32>,
    pub helios: HashMap<String, Vec<u32>>
}

impl ProcessManager {

    pub fn new() -> Self {

        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        if !Path::new(&filename).exists() {
            return Default::default();
        }

        // Load ymal file
        let yaml_code = fs::read_to_string(&filename).unwrap();
        let mut process_config: ProcessManager = match serde_yaml::from_str(&yaml_code) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to load pid.yml settings file, invalid YAML code.  Error: {}", e.to_string());
                std::process::exit(1);
            }
        };

        process_config
    }

    /// Start server
    pub fn start(&mut self, server_type: &str, port: i16, num_threads: i8) {

        // Initial checks
        if !vec!["apollo", "helios", "echo"].contains(&server_type) {
            error!("Invalid server type '{}'.  Supported types are apollo, helios, http", server_type);
            std::process::exit(1);
        }

        // Stop echo server, if needed
        if server_type == "echo" && self.echo > 0 {
            self.stop("echo");
        }

        // Get start port
        let mut server_port = port;
        if server_port == 0 && server_type == "apollo" {
            server_port = CONFIG.daemons.apollo.1 as i16;
        } else if server_port == 0 && server_type == "echo" {
            server_port = CONFIG.daemons.echo.1 as i16;
        } else if server_port == 0 && server_type == "helios" {
            server_port = CONFIG.daemons.echo.1 as i16;
        }
        info!("Starting {} daemon...", server_type);

        // Start server daemons
        let mut res_ports: Vec<i16> = Vec::new();
        for x in 0..num_threads {
            let pid = self.start_daemon(&server_type, &server_port);

            if server_type == "apollo" {
                self.apollo.push(pid.clone());
            } else if server_type == "echo" {
                self.echo = pid.clone();
            }
            res_ports.push(server_port.clone());
            server_port += 1;
        }

        // Save config
        self.save();

        // Get ip address
        let ipaddr = match server_type {
            "helios" => CONFIG.daemons.helios.0.clone(),
            "echo" => CONFIG.daemons.echo.0.clone(),
            _ => CONFIG.daemons.apollo.0.clone()
        };

        // User message
        let message = if res_ports.len() > 1 {
            format!("Started {} instances of {} daemon on {}, listening on ports {}...", num_threads, server_type, ipaddr, res_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ").to_string())
        } else {
            format!("Started {} daemon, listening on {}:{}", server_type, ipaddr, res_ports[0])
        };
        info!("{}", &message);
    }

    /// Stop service
    pub fn stop(&mut self, server_type: &str) {

        // Initial chec
        if !vec!["apollo","helios","echo"].contains(&server_type) {
            error!("Invalid server type, '{}'.  Supported types are: apollo, helios, http", server_type);
            std::process::exit(1);
        }
        info!("Stopping {} daemon...", server_type);

        // Echo
        if server_type == "echo" {
            self.kill(&self.echo.clone());
            self.echo = 0;
        } else if server_type == "apollo" {
            for pid in self.apollo.clone().into_iter() {
                self.kill(&pid);
            }
            self.apollo = Vec::new();

        } else if server_type == "helios" {

            let mut pids = Vec::new();
            for (alias, p) in self.helios.iter() {
                pids.extend(p);
            }

            for pid in pids {
                self.kill(&pid);
            }
            self.helios.clear();
        }
        info!("Stopped {} daemon...", server_type);

        // save config yaml file
        self.save();
    }

    /// Kill process by pid
    fn kill(&mut self, pid: &u32) {

        let output = Command::new("kill")
            .args(["-9", &pid.to_string()]) 
            .output()
            .expect("Unable to run 'kill' command");

        // Check status
        if !output.status.success() {
            warn!("Did not receive successful status trying to kill pid {}.  Got {} with (stdout: {}), (stderr: {})", 
                pid, output.status, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }

    }

    /// Start daemon process
    fn start_daemon(&mut self, server_type: &str, port: &i16) -> u32 {

        // Define command to spawn child
        let cicero_cmd = env::args().nth(0).unwrap();
        let mut cmd = Command::new(&cicero_cmd.to_string());
        cmd.args(["-d", "-t", server_type, "-p", port.to_string().as_str()]);

        // Spawn child process and start server
        let mut child = match cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to start {} daemon, error: {}", server_type, e);
                std::process::exit(1);
            }
        };

        // Detach child process
        match child.try_wait() {
            Ok(None) => {
                unsafe { libc::setsid(); }
            },
            Ok(Some(status)) => {
                error!("Received error from {} daemon, gave status: {}", server_type, status);
                std::process::exit(1);
            },
            Err(e) => { 
                error!("Unable to detach {} daemon child process with pid {}, error: {}", server_type, child.id(), e);
                std::process::exit(1);
            }
        };

        child.id()
    }

    /// Save pid.yml file
    fn save(&mut self) {

        // Get filename and yaml code
        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(&self).unwrap();

        // Save file
        match fs::write(&filename, &yaml_str) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to write to config {}, error: {}", filename, e);
                std::process::exit(1);
            }
        };

    }

}

impl Default for ProcessManager {
    fn default() -> ProcessManager {
        ProcessManager {
            echo: 0,
            apollo: Vec::new(),
            helios: HashMap::new()
        }
    }

}


