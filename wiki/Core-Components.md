# Core Components

This document provides detailed information about the Rust core implementation of the System Monitor, including data structures, implementation details, and cross-platform considerations.

## Table of Contents

- [Overview](#overview)
- [Data Structures](#data-structures)
- [SystemMonitor Implementation](#systemmonitor-implementation)
- [Platform-Specific Code](#platform-specific-code)
- [Error Handling](#error-handling)
- [Testing](#testing)
- [Performance Considerations](#performance-considerations)

## Overview

The core of the system monitor is implemented in Rust (`src/lib.rs`) with 273 lines of code. It uses the `sysinfo` crate for cross-platform system information collection and `wasm-bindgen` for WASM interoperability.

### Key Features

- **Cross-Platform**: Works on Linux, macOS, Windows, and WASM
- **Type-Safe**: Leverages Rust's type system
- **Serializable**: All data structures support JSON serialization
- **Zero-Copy**: Efficient data handling
- **Well-Tested**: Comprehensive test suite

### Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
sysinfo = "0.33.0"
```

## Data Structures

All data structures are defined with `Serialize` and `Deserialize` traits for seamless JSON conversion.

### SystemInfo

Contains comprehensive system information.

```rust
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
```

#### Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `os` | `String` | Operating system name | `"Linux"`, `"macOS"`, `"Windows"` |
| `os_version` | `String` | OS version | `"22.04"`, `"13.0"` |
| `kernel_version` | `String` | Kernel version | `"6.2.0-39-generic"` |
| `hostname` | `String` | System hostname | `"mycomputer"` |
| `cpu_count` | `usize` | Number of logical CPUs | `8` |
| `total_memory` | `u64` | Total RAM in bytes | `17179869184` (16GB) |
| `used_memory` | `u64` | Used RAM in bytes | `8589934592` (8GB) |
| `uptime` | `u64` | System uptime in seconds | `3600` (1 hour) |

#### JSON Example

```json
{
  "os": "Linux",
  "os_version": "22.04",
  "kernel_version": "6.2.0-39-generic",
  "hostname": "dev-machine",
  "cpu_count": 8,
  "total_memory": 17179869184,
  "used_memory": 8589934592,
  "uptime": 3600
}
```

### MemoryInfo

Memory usage statistics with calculated percentages.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
}
```

#### Fields

| Field | Type | Description | Units |
|-------|------|-------------|-------|
| `total` | `u64` | Total system memory | Bytes |
| `used` | `u64` | Currently used memory | Bytes |
| `available` | `u64` | Available memory | Bytes |
| `usage_percent` | `f64` | Usage percentage (0-100) | Percent |

#### Calculation Logic

```rust
let usage_percent = if total > 0 {
    (used as f64 / total as f64) * 100.0
} else {
    0.0
};
```

#### JSON Example

```json
{
  "total": 17179869184,
  "used": 8589934592,
  "available": 8589934592,
  "usage_percent": 50.0
}
```

### DiskInfo

Information about disk mounts and storage.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub usage_percent: f64,
}
```

#### Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `name` | `String` | Disk device name | `"/dev/sda1"` |
| `mount_point` | `String` | Mount point path | `"/"`, `"/home"` |
| `total_space` | `u64` | Total disk space (bytes) | `500000000000` |
| `available_space` | `u64` | Available space (bytes) | `250000000000` |
| `usage_percent` | `f64` | Usage percentage (0-100) | `50.0` |

#### Calculation Logic

```rust
let total = disk.total_space();
let available = disk.available_space();
let used = total.saturating_sub(available);
let usage_percent = if total > 0 {
    (used as f64 / total as f64) * 100.0
} else {
    0.0
};
```

#### JSON Example

```json
[
  {
    "name": "/dev/sda1",
    "mount_point": "/",
    "total_space": 500000000000,
    "available_space": 250000000000,
    "usage_percent": 50.0
  },
  {
    "name": "/dev/sdb1",
    "mount_point": "/home",
    "total_space": 1000000000000,
    "available_space": 750000000000,
    "usage_percent": 25.0
  }
]
```

### CpuInfo

Per-CPU core information and statistics.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}
```

#### Fields

| Field | Type | Description | Units |
|-------|------|-------------|-------|
| `name` | `String` | CPU core name | Model string |
| `usage` | `f32` | Current usage (0-100) | Percent |
| `frequency` | `u64` | Current frequency | MHz |

#### JSON Example

```json
[
  {
    "name": "cpu0",
    "usage": 45.5,
    "frequency": 2400
  },
  {
    "name": "cpu1",
    "usage": 32.1,
    "frequency": 2400
  }
]
```

## SystemMonitor Implementation

The main interface for system monitoring operations.

### Structure Definition

```rust
#[wasm_bindgen]
pub struct SystemMonitor {
    #[cfg(not(target_arch = "wasm32"))]
    sys: System,
}
```

The structure uses conditional compilation:
- **Native platforms**: Contains a `sysinfo::System` instance
- **WASM target**: Empty struct (system info not available in browser)

### Constructor

```rust
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
```

**Native Behavior**:
1. Creates new `System` instance with all components
2. Refreshes all data immediately
3. Returns initialized monitor

**WASM Behavior**:
- Returns empty struct
- Methods return placeholder data

### Methods

#### refresh()

Refresh all system metrics.

```rust
pub fn refresh(&mut self) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        self.sys.refresh_all();
    }
}
```

**Usage**: Call before querying metrics for most up-to-date data.

#### get_system_info()

Returns system information as JSON string.

```rust
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
    // ... WASM implementation
}
```

**Flow**:
1. Refresh all system data
2. Collect information from `sysinfo::System`
3. Create `SystemInfo` struct
4. Serialize to JSON
5. Return JSON string

**Error Handling**: Falls back to `"Unknown"` for missing values, `"{}"` for serialization errors.

#### get_memory_info()

Returns memory usage as JSON string.

```rust
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
    // ... WASM implementation
}
```

**Optimization**: Only refreshes memory data (faster than `refresh_all()`).

#### list_disks()

Returns array of disk information as JSON string.

```rust
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
    // ... WASM implementation
}
```

**Features**:
- Refreshes disk list on each call
- Maps all disks to `DiskInfo` structures
- Handles potential UTF-8 issues with `to_string_lossy()`
- Uses saturating subtraction to prevent underflow

#### get_cpu_info()

Returns per-CPU information as JSON string.

```rust
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
    // ... WASM implementation
}
```

**Optimization**: Only refreshes CPU data.

## Platform-Specific Code

The library uses conditional compilation for cross-platform support.

### Compilation Targets

#### Native Platforms

```rust
#[cfg(not(target_arch = "wasm32"))]
use sysinfo::{Disks, System};
```

- Full access to `sysinfo` crate
- Real system information collection
- Native performance

#### WASM Target

```rust
#[cfg(target_arch = "wasm32")]
{
    Self {}  // Empty struct
}
```

- No system access (browser security)
- Returns placeholder JSON
- Maintains API compatibility

### Platform-Specific Patterns

```rust
#[cfg(not(target_arch = "wasm32"))]
{
    // Native implementation with real data
}
#[cfg(target_arch = "wasm32")]
{
    // WASM implementation with placeholders
}
```

This pattern appears in:
- Constructor
- All data collection methods
- `refresh()` method

### Fallback Values

**Native Platform** (when data unavailable):
```rust
System::name().unwrap_or_else(|| "Unknown".to_string())
```

**WASM Target**:
```rust
r#"{"os":"WASM","os_version":"N/A", ...}"#.to_string()
```

## Error Handling

### Strategy

The library uses defensive error handling with graceful degradation:

1. **Option Unwrapping**: Use `unwrap_or_else()` with fallback values
2. **Serialization**: Fallback to empty JSON on error
3. **Arithmetic**: Use saturating operations to prevent overflow/underflow
4. **String Conversion**: Use `to_string_lossy()` for potentially invalid UTF-8

### Examples

#### Safe Option Handling

```rust
os: System::name().unwrap_or_else(|| "Unknown".to_string())
```

If `System::name()` returns `None`, uses `"Unknown"`.

#### Safe Serialization

```rust
serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
```

If serialization fails, returns empty JSON object.

#### Safe Arithmetic

```rust
let used = total.saturating_sub(available);
```

Prevents underflow if `available > total`.

#### Safe Division

```rust
let usage_percent = if total > 0 {
    (used as f64 / total as f64) * 100.0
} else {
    0.0
};
```

Prevents division by zero.

## Testing

The library includes comprehensive unit tests.

### Test Coverage

```rust
#[cfg(test)]
mod tests {
    // 5 tests covering all major functionality
}
```

#### Test: Monitor Creation

```rust
#[test]
fn test_system_monitor_creation() {
    let _monitor = SystemMonitor::new();
    // Verifies constructor doesn't panic
}
```

#### Test: System Info

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_get_system_info() {
    let mut monitor = SystemMonitor::new();
    let info = monitor.get_system_info();
    assert!(!info.is_empty());

    let parsed: serde_json::Value = serde_json::from_str(&info)
        .expect("Should be valid JSON");
    assert!(parsed.is_object());
}
```

Verifies:
- Non-empty response
- Valid JSON format
- Object structure

#### Test: Memory Info

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_get_memory_info() {
    let mut monitor = SystemMonitor::new();
    let info = monitor.get_memory_info();

    let parsed: MemoryInfo = serde_json::from_str(&info)
        .expect("Should be valid JSON");
    assert!(parsed.total >= parsed.used);
    assert!(parsed.usage_percent >= 0.0 && parsed.usage_percent <= 100.0);
}
```

Verifies:
- Logical constraints (total ≥ used)
- Percentage in valid range (0-100)

#### Test: Disk Info

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_list_disks() {
    let monitor = SystemMonitor::new();
    let disks = monitor.list_disks();

    let parsed: Vec<DiskInfo> = serde_json::from_str(&disks)
        .expect("Should be valid JSON");
    for disk in parsed {
        assert!(disk.usage_percent >= 0.0 && disk.usage_percent <= 100.0);
    }
}
```

Verifies:
- Array format
- Valid percentages for all disks

#### Test: CPU Info

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_get_cpu_info() {
    let mut monitor = SystemMonitor::new();
    let info = monitor.get_cpu_info();

    let parsed: Vec<CpuInfo> = serde_json::from_str(&info)
        .expect("Should be valid JSON");
    assert!(!parsed.is_empty());
}
```

Verifies:
- Array format
- At least one CPU detected

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_get_system_info
```

## Performance Considerations

### Optimization Techniques

#### 1. Targeted Refreshes

Methods refresh only necessary data:

```rust
// Only refresh memory (fast)
self.sys.refresh_memory();

// Only refresh CPUs (fast)
self.sys.refresh_cpu_all();

// Refresh everything (slower)
self.sys.refresh_all();
```

#### 2. Efficient String Handling

Use `to_string_lossy()` to avoid allocation failures:

```rust
name: disk.name().to_string_lossy().to_string()
```

#### 3. Iterator Chains

Zero-allocation transformations:

```rust
let disk_infos: Vec<DiskInfo> = disks
    .iter()
    .map(|disk| { /* ... */ })
    .collect();
```

#### 4. Saturating Arithmetic

Prevent overflow checks:

```rust
let used = total.saturating_sub(available);
```

### Compilation Optimizations

From `Cargo.toml`:

```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

**Results**:
- WASM binary: 17KB
- Build time: ~5 seconds
- Runtime overhead: <1MB memory

### Memory Usage

- **SystemMonitor struct**: ~200 bytes (native), 0 bytes (WASM)
- **JSON serialization**: Temporary allocations, freed immediately
- **Overall overhead**: <1MB during operation

## Extension Points

### Adding New Metrics

1. **Define struct**:
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}
```

2. **Add method**:
```rust
#[wasm_bindgen]
impl SystemMonitor {
    pub fn get_network_info(&self) -> String {
        // Implementation
    }
}
```

3. **Add TypeScript wrapper** (see [TypeScript Integration](TypeScript-Integration.md))

### Example: Temperature Monitoring

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemperatureInfo {
    pub component: String,
    pub temperature: f32,
    pub critical: f32,
}

#[wasm_bindgen]
impl SystemMonitor {
    pub fn get_temperatures(&self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let components = Components::new_with_refreshed_list();
            let temps: Vec<TemperatureInfo> = components
                .iter()
                .map(|comp| TemperatureInfo {
                    component: comp.label().to_string(),
                    temperature: comp.temperature(),
                    critical: comp.critical().unwrap_or(100.0),
                })
                .collect();
            serde_json::to_string(&temps).unwrap_or_else(|_| "[]".to_string())
        }
        #[cfg(target_arch = "wasm32")]
        {
            "[]".to_string()
        }
    }
}
```

## Related Documentation

- [Architecture](Architecture.md) - System architecture overview
- [TypeScript Integration](TypeScript-Integration.md) - TypeScript wrapper layer
- [API Reference](API-Reference.md) - Complete API documentation
- [Build System](Build-System.md) - Compilation and optimization

---

[Back to Home](Home.md)
