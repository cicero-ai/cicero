
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use nix::libc;
use std::path::Path;
use std::{fs, env};
use super::sys;
use crate::Error;
use log::{info, warn, error};

#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use winapi::um::processthreadsapi::CreateProcessW;
#[cfg(windows)]
use winapi::um::winbase::DETACHED_PROCESS;
#[cfg(windows)]
use winapi::um::winbase::CREATE_NEW_PROCESS_GROUP;
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(windows)]
use winapi::um::processthreadsapi::STARTUPINFOW;
#[cfg(windows)]
use winapi::um::processthreadsapi::PROCESS_INFORMATION;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;


#[derive(Default, Serialize, Deserialize)]
pub struct ProcessManager(pub HashMap<String, Process>);

#[derive(Serialize, Deserialize)]
struct Process {
    pub pid: u32,
    pub cmd_name: String,
    pub cmd_args: Vec<String>
}

impl ProcessManager {
    pub fn new() -> Result<Self, Error> {

        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        if !Path::new(&filename).exists() {
            return Ok(Self::default());
        }

        // Load ymal file
        let yaml_code = fs::read_to_string(&filename)
            .map_err(|e| Error::IO( format!("Unable to read file file {}, error: {}", filename, e)) )?;

        serde_yaml::from_str(&yaml_code)
            .map_err(|e| Error::IO( format!("Unable to decode YAML file {}, error: {}", filename, e)) )
    }

    /// Start daemon process
    pub fn start(&mut self, cmd_name: &str, cmd_args: &[&str]) -> Result<(), Error> {

        // Get command to run
        let cmd_alias: String = if cmd_name.is_empty() {
            env::args().nth(0).expect("No 0 / self argument in environment!")
        } else {
            cmd_name.to_string()
        };

        // Define command to spawn child
        let mut cmd = Command::new(&cmd_alias.as_str());
        cmd.args(cmd_args.clone());
        let mut new_pid: u32 = 0;

        // Linux / Mac
        #[cfg(unix)]
        {

            // Spawn child process and start server
            let mut child = cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn()
                .map_err(|e| Error::ProcMgr( format!("Failed to start {} daemon, error: {}", cmd_alias, e)) )?;

            // Detach child process
            let detach_res = child.try_wait().map_err(|e| {
                error!("Unable to detach {} daemon child process with pid {}, error: {}", cmd_alias, child.id(), e);
                Error::ProcMgr(e.to_string())
            })?;

            // Check for error from daemon
            if let Some(status) = detach_res {
                return Err( Error::ProcMgr(format!("Received error from {} daemon, gave status: {}", cmd_alias, status)) );
            }

            // Detach
            unsafe { libc::setsid(); }
            new_pid = child.id();
        }

        // Windows
        #[cfg(windows)]
        {
            let command = OsString::from("your_command");
            let mut command_wide: Vec<u16> = command.encode_wide().collect();
            command_wide.push(0);

            let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
            startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

            let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

            let success = unsafe {
                CreateProcessW(
                    ptr::null(), // No module name (use command line)
                    command_wide.as_mut_ptr(),
                    ptr::null_mut(), // Process handle not inheritable
                    ptr::null_mut(), // Thread handle not inheritable
                    false, // Set handle inheritance to FALSE
                    DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP, // Detach and create new process group
                    ptr::null_mut(), // Use parent's environment block
                    ptr::null(), // Use parent's starting directory 
                    &mut startup_info,
                    &mut process_info,
                )
            };

            if success == 0 {
                let error_code = unsafe { GetLastError() };
                return Err( Error::ProcMgr(format!("Failed to start the daemon, error code: {}", error_code)) );
            } else {
                unsafe {
                    CloseHandle(process_info.hProcess);
                    CloseHandle(process_info.hThread);
                }
            }
        }

        // Add process
        self.0.insert(cmd_name.to_string(), Process {
            pid: new_pid,
            cmd_name: cmd_name.to_string(),
            cmd_args: cmd_args.iter().map(|a| a.to_string()).collect::<Vec<String>>()
        });

        // Save
        self.save()?;
        Ok(())
    }

    /// Stop service
    pub fn stop(&mut self, cmd_name: &str) -> Result<(), Error> {

        // Get process
        let process = match self.0.get(&cmd_name.to_string()) {
            Some(r) => r,
            None => return Ok(())
        };

        // Kill pid
        let output = Command::new("kill")
            .args(["-9", &process.pid.to_string()]) 
            .output().map_err(|e| Error::ProcMgr( format!("Unable to kill pid {}, error: {}", process.pid, e)) )?;

        // Check status
        if !output.status.success() {
            warn!("Did not receive successful status trying to kill pid {}.  Got {} with (stdout: {}), (stderr: {})", 
                process.pid, output.status, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }

        // Remove process, save config
        self.0.remove(&cmd_name.to_string());
        self.save()
    }

    /// Save pid.yml file
    fn save(&mut self) -> Result<(), Error> {

        // Get filename and yaml code
        let filename = format!("{}/config/pid.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(&self)
            .map_err(|e| Error::IO( format!("Unable to encode  YAML,  error: {}", e)) )?; 

        // Save file
        fs::write(&filename, &yaml_str)
            .map_err(|e| Error::IO(format!("Unable to write to config {}, error: {}", filename, e)) )?;

        Ok(())
    }

}


