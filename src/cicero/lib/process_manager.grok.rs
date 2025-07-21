use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use std::path::Path;
use std::{fs, env, io};
use super::sys;
use log::{error};

#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use winapi::um::processthreadsapi::{CreateProcessW, STARTUPINFOW, PROCESS_INFORMATION};
#[cfg(windows)]
use winapi::um::winbase::{DETACHED_PROCESS, CREATE_NO_WINDOW};
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[derive(Serialize, Deserialize)]
pub struct ProcessManager(pub HashMap<String, Process>);

#[derive(Serialize, Deserialize)]
struct Process {
    pub pid: u32,
    pub cmd_name: String,
    pub cmd_args: Vec<String>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        if !Path::new(&filename).exists() {
            return Self::default();
        }

        let yaml_code = fs::read_to_string(&filename).unwrap_or_else(|e| {
            error!("Unable to read {}: {}", filename, e);
            std::process::exit(1);
        });

        serde_yaml::from_str(&yaml_code).unwrap_or_else(|e| {
            error!("Invalid pid.yml: {}", e);
            std::process::exit(1);
        })
    }

    pub fn start(&mut self, cmd_name: &str, port: u16, threads: u16) -> u32 {
        let cmd_alias = if cmd_name.is_empty() {
            env::args().next().expect("No self arg!")
        } else {
            cmd_name.to_string()
        };
        let cmd_args = vec!["-id".to_string(), port.to_string(), threads.to_string()];

        let pid = if cfg!(unix) {
            let mut cmd = Command::new(&cmd_alias);
            cmd.args(&cmd_args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());

            let child = cmd.spawn().unwrap_or_else(|e| {
                error!("Failed to start {}: {}", cmd_name, e);
                std::process::exit(1);
            });

            // Detach (simplified—could use daemonize crate for full daemon)
            child.id()
        } else if cfg!(windows) {
            let cmd_line = format!("{} {}", cmd_alias, cmd_args.join(" "));
            let mut cmd_wide: Vec<u16> = OsString::from(cmd_line).encode_wide().chain(Some(0)).collect();

            let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
            startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

            let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

            let success = unsafe {
                CreateProcessW(
                    ptr::null_mut(),           // No module name
                    cmd_wide.as_mut_ptr(),     // Command line
                    ptr::null_mut(),           // Process handle not inheritable
                    ptr::null_mut(),           // Thread handle not inheritable
                    0,                         // No handle inheritance
                    DETACHED_PROCESS | CREATE_NO_WINDOW, // No console
                    ptr::null_mut(),           // Parent env
                    ptr::null_mut(),           // Parent dir
                    &mut startup_info,
                    &mut process_info,
                )
            };

            if success == 0 {
                error!("Failed to start {} on Windows: {}", cmd_name, io::Error::last_os_error());
                std::process::exit(1);
            } else {
                unsafe {
                    CloseHandle(process_info.hProcess);
                    CloseHandle(process_info.hThread);
                }
                process_info.dwProcessId
            }
        } else {
            panic!("Unsupported OS");
        };

        self.0.insert(cmd_name.to_string(), Process {
            pid,
            cmd_name: cmd_name.to_string(),
            cmd_args,
        });
        self.save();
        pid
    }

    pub fn stop(&mut self, cmd_name: &str) {
        let process = match self.0.get(cmd_name) {
            Some(p) => p,
            None => return,
        };

        #[cfg(unix)]
        Command::new("kill")
            .args(["-9", &process.pid.to_string()])
            .output()
            .map(|o| if !o.status.success() {
                error!("Failed to kill {} (PID {}): {}", cmd_name, process.pid, String::from_utf8_lossy(&o.stderr));
            })
            .unwrap_or_else(|e| error!("Kill error: {}", e));

        #[cfg(windows)]
        Command::new("taskkill")
            .args(["/PID", &process.pid.to_string(), "/F"])
            .output()
            .map(|o| if !o.status.success() {
                error!("Failed to kill {} (PID {}): {}", cmd_name, process.pid, String::from_utf8_lossy(&o.stderr));
            })
            .unwrap_or_else(|e| error!("Taskkill error: {}", e));

        self.0.remove(cmd_name);
        self.save();
    }

    fn save(&self) {
        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(self).unwrap_or_else(|e| {
            error!("Failed to serialize pid.yml: {}", e);
            std::process::exit(1);
        });
        fs::write(&filename, yaml_str).unwrap_or_else(|e| {
            error!("Failed to write {}: {}", filename, e);
            std::process::exit(1);
        });
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self(HashMap::new())
    }
}


