
use sysinfo::System;
use std::fmt;
use crate::llm::models::ModelSize;
use which::which;

pub struct HardwareProfile {
    pub cpu_model: String,
    pub num_threads: usize,
    pub os_name: String,
    pub ram: f32,
    pub has_gpu: bool,
    pub llm_size: ModelSize
}

impl HardwareProfile {

    pub fn new() -> Self {

        // Get system info
        let mut sysinfo = System::new();
        sysinfo.refresh_all();

        // Init
        let mut profile = HardwareProfile::default();
        profile.num_threads = sysinfo.cpus().len();

        // Get CPU model name
        if profile.num_threads > 0 {
            let cpu = &sysinfo.cpus()[0];
            profile.cpu_model = cpu.brand().to_string();
        }

        // OS name
        profile.os_name = match System::long_os_version() {
            Some(r) => r.to_string(),
            None => "Unknown".to_string()
        };

        // RAM
        profile.ram = (((sysinfo.total_memory()  / 1024 / 1024) as f32 / 1000.0).ceil() * 1000.0) / 1000.0;
        profile.has_gpu = if which("nvidia-smi").is_ok() { true } else { false };

        // LLM size
        profile.llm_size = match (profile.ram, profile.num_threads, profile.has_gpu) {
            (r, c, true) if r >= 24.0 => ModelSize::large,
            (r, c, _) if r >= 16.0 && c >= 4 => ModelSize::medium,
            (r, _, _) if r >= 8.0 => ModelSize::small,
            _ => ModelSize::tiny,
        };

        profile
    }
}

/// Rough and non-reliant check whether or not machine has a GPU

impl Default for HardwareProfile {
    fn default() -> HardwareProfile {
        HardwareProfile {
            cpu_model: "Unknown".to_string(),
            num_threads: 0,
            os_name: "Unknown".to_string(),
            ram: 0.0,
            has_gpu: false,
            llm_size: ModelSize::small
        }
    }
}

impl fmt::Display for HardwareProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gpu_info = if self.has_gpu {
            "plus a NVIDIA card with Cuda support.".to_string()
        } else {
            "and no NVIDIA GPU card".to_string()
        };
        write!(f, "{} on {} with {}GB RAM {}", self.cpu_model, self.os_name, self.ram, gpu_info)
    }
}


