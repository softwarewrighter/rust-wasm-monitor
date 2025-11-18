//! System Monitor - Rust/WASM Alternative to MCP Servers
//!
//! This library demonstrates the code-first approach described in Anthropic's
//! "Code execution with MCP" paper, achieving 98.7% reduction in token usage
//! by having AI agents write code instead of calling tools directly.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use sysinfo::{Disks, System};

/// System information structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uptime: u64,
}

/// Memory usage information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
}

/// Disk mount information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub usage_percent: f64,
}

/// CPU usage information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

/// Main monitor interface exposed to WASM
#[wasm_bindgen]
pub struct SystemMonitor {
    #[cfg(not(target_arch = "wasm32"))]
    sys: System,
}

#[wasm_bindgen]
impl SystemMonitor {
    /// Create a new system monitor instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut sys = System::new_all();
            sys.refresh_all();
            Self { sys }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self {}
        }
    }

    /// Refresh system information
    pub fn refresh(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.sys.refresh_all();
        }
    }

    /// Get basic system information as JSON string
    pub fn get_system_info(&mut self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.sys.refresh_all();

            let info = SystemInfo {
                os: System::name().unwrap_or_else(|| "Unknown".to_string()),
                os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
                kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
                hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
                cpu_count: self.sys.cpus().len(),
                total_memory: self.sys.total_memory(),
                used_memory: self.sys.used_memory(),
                uptime: System::uptime(),
            };

            serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            r#"{"os":"WASM","os_version":"N/A","kernel_version":"N/A","hostname":"browser","cpu_count":0,"total_memory":0,"used_memory":0,"uptime":0}"#.to_string()
        }
    }

    /// Get memory usage information as JSON string
    pub fn get_memory_info(&mut self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.sys.refresh_memory();

            let total = self.sys.total_memory();
            let used = self.sys.used_memory();
            let available = self.sys.available_memory();
            let usage_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            let info = MemoryInfo {
                total,
                used,
                available,
                usage_percent,
            };

            serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            r#"{"total":0,"used":0,"available":0,"usage_percent":0.0}"#.to_string()
        }
    }

    /// List all disk mounts as JSON string
    pub fn list_disks(&self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let disks = Disks::new_with_refreshed_list();
            let disk_infos: Vec<DiskInfo> = disks
                .iter()
                .map(|disk| {
                    let total = disk.total_space();
                    let available = disk.available_space();
                    let used = total.saturating_sub(available);
                    let usage_percent = if total > 0 {
                        (used as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    DiskInfo {
                        name: disk.name().to_string_lossy().to_string(),
                        mount_point: disk.mount_point().to_string_lossy().to_string(),
                        total_space: total,
                        available_space: available,
                        usage_percent,
                    }
                })
                .collect();

            serde_json::to_string(&disk_infos).unwrap_or_else(|_| "[]".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            "[]".to_string()
        }
    }

    /// Get CPU information as JSON string
    pub fn get_cpu_info(&mut self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.sys.refresh_cpu_all();

            let cpu_infos: Vec<CpuInfo> = self
                .sys
                .cpus()
                .iter()
                .map(|cpu| CpuInfo {
                    name: cpu.name().to_string(),
                    usage: cpu.cpu_usage(),
                    frequency: cpu.frequency(),
                })
                .collect();

            serde_json::to_string(&cpu_infos).unwrap_or_else(|_| "[]".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            "[]".to_string()
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let _monitor = SystemMonitor::new();
        // Monitor created successfully - test passes if no panic
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_get_system_info() {
        let mut monitor = SystemMonitor::new();
        let info = monitor.get_system_info();
        assert!(!info.is_empty());

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&info).expect("Should be valid JSON");
        assert!(parsed.is_object());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_get_memory_info() {
        let mut monitor = SystemMonitor::new();
        let info = monitor.get_memory_info();
        assert!(!info.is_empty());

        let parsed: MemoryInfo = serde_json::from_str(&info).expect("Should be valid JSON");
        assert!(parsed.total >= parsed.used);
        assert!(parsed.usage_percent >= 0.0 && parsed.usage_percent <= 100.0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_list_disks() {
        let monitor = SystemMonitor::new();
        let disks = monitor.list_disks();
        assert!(!disks.is_empty());

        let parsed: Vec<DiskInfo> = serde_json::from_str(&disks).expect("Should be valid JSON");
        for disk in parsed {
            assert!(disk.usage_percent >= 0.0 && disk.usage_percent <= 100.0);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_get_cpu_info() {
        let mut monitor = SystemMonitor::new();
        let info = monitor.get_cpu_info();
        assert!(!info.is_empty());

        let parsed: Vec<CpuInfo> = serde_json::from_str(&info).expect("Should be valid JSON");
        assert!(!parsed.is_empty());
    }
}
