
use falcon_cli::*;
use std::process::Command;
use std::io::{self, Error};
use std::env;

// Check whether or not Ollama is installed, and if not, prompt to auto-install it.
pub fn run() -> io::Result<bool> {

    // Check if already installed
    if is_ollama_installed() {
        return Ok(true);
    }

    // Check if user wishes to install Ollama
    cli_header("Ollama");
    cli_send!("It is recommended to use Ollama for conversational output as it's local, private, free, and none of your data is sent to big tech.  If you would prefer not to use Ollama, you may select your preferred API provider on the next screen.\n\n");
    if !cli_confirm("Would you like to install Ollama now?") {
        return Ok(false);
    }
    cli_send!("Attempting to auto-install Ollama...\n");

    // Install 
    match install_ollama() {
        Ok(status) if status.success() => {
            cli_send!("Ollama installed successfully!");
            Ok(true)
        },
        Ok(_) => {
            cli_send!("Failed to install Ollama—check logs or install manually.");
            Ok(false)
        },
        Err(e) => {
            cli_send!("Error installing Ollama: {}", e);
            Err(e)
        }
    }
}

/// Check if Ollama installed
fn is_ollama_installed() -> bool {
    Command::new("ollama").arg("--version").status().map(|s| s.success()).unwrap_or(false)
}

fn install_ollama() -> io::Result<std::process::ExitStatus> {
    match env::consts::OS {
        "linux" => Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://ollama.com/install.sh | sh")
            .status(),
        "macos" => {
            if Command::new("brew").status().is_ok() {
                Command::new("brew").arg("install").arg("ollama").status()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("curl -fsSL https://ollama.com/install.sh | sh")
                    .status()
            }
        },
        "windows" => {
            cli_send!("Windows auto-install not supported. Please:");
            cli_send!("1. Download from https://ollama.com/download");
            cli_send!("2. Run the installer and ensure Ollama is in your PATH.");
            cli_send!("3. Restart this setup after installation.");
            Err(Error::new(io::ErrorKind::Unsupported, "Manual install required"))
        },
        os => Err(Error::new(io::ErrorKind::Unsupported, format!("OS not supported: {}", os))),
    }
}


