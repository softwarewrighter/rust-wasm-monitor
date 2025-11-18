# API Reference

Complete API documentation for the Rust/WASM System Monitor. This reference covers both the TypeScript wrapper API and the underlying WASM API.

## Table of Contents

- [TypeScript API](#typescript-api)
  - [Functions](#functions)
  - [Types](#types)
- [WASM API](#wasm-api)
  - [SystemMonitor Class](#systemmonitor-class)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)
- [Platform Notes](#platform-notes)

## TypeScript API

The TypeScript API is the recommended way to use this library. It provides type-safe, ergonomic wrappers around the WASM implementation.

**Import**:
```typescript
import {
  getSystemInfo,
  getMemoryInfo,
  listDisks,
  getCpuInfo,
  refresh,
  // Types
  SystemInfo,
  MemoryInfo,
  DiskInfo,
  CpuInfo
} from './tools/system-monitor';
```

### Functions

#### init()

Initialize the WASM module. Called automatically by other functions.

```typescript
function init(): Promise<void>
```

**Returns**: `Promise<void>`

**Description**: Lazy-loads the WASM module and creates a `SystemMonitor` instance. Subsequent calls are no-ops.

**Example**:
```typescript
// Explicit initialization (optional)
await init();

// Or just call other functions (they auto-init)
const info = await getSystemInfo();
```

**Performance**: First call ~100ms, subsequent calls <1ms

---

#### getSystemInfo()

Get comprehensive system information.

```typescript
function getSystemInfo(): Promise<SystemInfo>
```

**Returns**: `Promise<SystemInfo>`

**Description**: Returns operating system details, CPU count, total memory, used memory, and system uptime.

**Example**:
```typescript
const info = await getSystemInfo();
console.log(`${info.os} ${info.os_version}`);
console.log(`Hostname: ${info.hostname}`);
console.log(`CPUs: ${info.cpu_count}`);
console.log(`Memory: ${(info.total_memory / 1024**3).toFixed(2)} GB`);
console.log(`Uptime: ${info.uptime}s`);
```

**Sample Output**:
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

**Platform Notes**:
- Native: Real system data
- WASM: Returns placeholder data (see [Platform Notes](#platform-notes))

---

#### getMemoryInfo()

Get current memory usage statistics.

```typescript
function getMemoryInfo(): Promise<MemoryInfo>
```

**Returns**: `Promise<MemoryInfo>`

**Description**: Returns total, used, and available memory with calculated usage percentage.

**Example**:
```typescript
const mem = await getMemoryInfo();

console.log(`Total: ${(mem.total / 1024**3).toFixed(2)} GB`);
console.log(`Used: ${(mem.used / 1024**3).toFixed(2)} GB`);
console.log(`Available: ${(mem.available / 1024**3).toFixed(2)} GB`);
console.log(`Usage: ${mem.usage_percent.toFixed(1)}%`);

if (mem.usage_percent > 90) {
  console.warn('Critical memory usage!');
}
```

**Sample Output**:
```json
{
  "total": 17179869184,
  "used": 8589934592,
  "available": 8589934592,
  "usage_percent": 50.0
}
```

**Calculation**:
```
usage_percent = (used / total) * 100
```

---

#### listDisks()

List all mounted disk volumes.

```typescript
function listDisks(): Promise<DiskInfo[]>
```

**Returns**: `Promise<DiskInfo[]>`

**Description**: Returns array of all mounted disks with space usage information.

**Example**:
```typescript
const disks = await listDisks();

for (const disk of disks) {
  const used = disk.total_space - disk.available_space;
  const usedGB = (used / 1024**3).toFixed(2);
  const totalGB = (disk.total_space / 1024**3).toFixed(2);

  console.log(`${disk.mount_point}:`);
  console.log(`  Device: ${disk.name}`);
  console.log(`  Usage: ${usedGB} GB / ${totalGB} GB (${disk.usage_percent.toFixed(1)}%)`);

  if (disk.usage_percent > 90) {
    console.warn(`  WARNING: Low space on ${disk.mount_point}`);
  }
}
```

**Sample Output**:
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

**Platform Notes**:
- Lists all mounted filesystems
- Includes network mounts (NFS, SMB, etc.)
- WASM target returns empty array

---

#### getCpuInfo()

Get per-CPU core information.

```typescript
function getCpuInfo(): Promise<CpuInfo[]>
```

**Returns**: `Promise<CpuInfo[]>`

**Description**: Returns array with one entry per logical CPU core, including usage percentage and frequency.

**Example**:
```typescript
const cpus = await getCpuInfo();

console.log(`CPU Cores: ${cpus.length}`);

cpus.forEach((cpu, i) => {
  console.log(`${cpu.name}: ${cpu.usage.toFixed(1)}% @ ${cpu.frequency} MHz`);
});

// Calculate average usage
const avgUsage = cpus.reduce((sum, cpu) => sum + cpu.usage, 0) / cpus.length;
console.log(`Average usage: ${avgUsage.toFixed(1)}%`);

// Find hottest CPU
const hottestCPU = cpus.reduce((max, cpu) =>
  cpu.usage > max.usage ? cpu : max
);
console.log(`Hottest: ${hottestCPU.name} at ${hottestCPU.usage.toFixed(1)}%`);
```

**Sample Output**:
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
  },
  {
    "name": "cpu2",
    "usage": 67.8,
    "frequency": 2600
  },
  {
    "name": "cpu3",
    "usage": 23.4,
    "frequency": 2400
  }
]
```

**Notes**:
- Usage is instantaneous (snapshot at time of call)
- For accurate usage, consider averaging multiple samples
- Frequency may vary (CPU throttling, boost modes)

---

#### refresh()

Refresh all cached system data.

```typescript
function refresh(): Promise<void>
```

**Returns**: `Promise<void>`

**Description**: Forces a refresh of all system metrics. Most `get*` methods auto-refresh, so this is rarely needed.

**Example**:
```typescript
// Ensure all data is from the same instant
await refresh();

const [sys, mem, disks, cpus] = await Promise.all([
  getSystemInfo(),
  getMemoryInfo(),
  listDisks(),
  getCpuInfo()
]);

// All data is from the same refresh cycle
```

**When to Use**:
- When you need a consistent snapshot across multiple metrics
- Before collecting metrics for monitoring

**When Not to Use**:
- Before a single `get*` call (they auto-refresh)
- In tight loops (causes excessive system calls)

### Types

#### SystemInfo

```typescript
interface SystemInfo {
  os: string;              // Operating system name
  os_version: string;      // OS version
  kernel_version: string;  // Kernel version
  hostname: string;        // System hostname
  cpu_count: number;       // Number of logical CPUs
  total_memory: number;    // Total RAM (bytes)
  used_memory: number;     // Used RAM (bytes)
  uptime: number;          // System uptime (seconds)
}
```

**Field Details**:

| Field | Type | Units | Example |
|-------|------|-------|---------|
| `os` | `string` | - | `"Linux"`, `"Windows"`, `"macOS"` |
| `os_version` | `string` | - | `"22.04"`, `"11"`, `"13.0"` |
| `kernel_version` | `string` | - | `"6.2.0-39-generic"` |
| `hostname` | `string` | - | `"dev-machine"` |
| `cpu_count` | `number` | cores | `8` |
| `total_memory` | `number` | bytes | `17179869184` (16 GB) |
| `used_memory` | `number` | bytes | `8589934592` (8 GB) |
| `uptime` | `number` | seconds | `3600` (1 hour) |

**Helper Functions**:
```typescript
function formatMemory(bytes: number): string {
  return `${(bytes / 1024**3).toFixed(2)} GB`;
}

function formatUptime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${mins}m`;
}
```

---

#### MemoryInfo

```typescript
interface MemoryInfo {
  total: number;           // Total memory (bytes)
  used: number;            // Used memory (bytes)
  available: number;       // Available memory (bytes)
  usage_percent: number;   // Usage percentage (0-100)
}
```

**Field Details**:

| Field | Type | Units | Range |
|-------|------|-------|-------|
| `total` | `number` | bytes | 0 - 2^53 |
| `used` | `number` | bytes | 0 - total |
| `available` | `number` | bytes | 0 - total |
| `usage_percent` | `number` | percent | 0 - 100 |

**Invariants**:
- `used <= total`
- `available <= total`
- `usage_percent = (used / total) * 100`

---

#### DiskInfo

```typescript
interface DiskInfo {
  name: string;            // Device name
  mount_point: string;     // Mount point path
  total_space: number;     // Total space (bytes)
  available_space: number; // Available space (bytes)
  usage_percent: number;   // Usage percentage (0-100)
}
```

**Field Details**:

| Field | Type | Units | Example |
|-------|------|-------|---------|
| `name` | `string` | - | `"/dev/sda1"`, `"C:\\"` |
| `mount_point` | `string` | - | `"/"`, `"/home"`, `"C:\\"` |
| `total_space` | `number` | bytes | `500000000000` |
| `available_space` | `number` | bytes | `250000000000` |
| `usage_percent` | `number` | percent | `50.0` |

**Calculation**:
```typescript
const used = total_space - available_space;
const usage_percent = (used / total_space) * 100;
```

---

#### CpuInfo

```typescript
interface CpuInfo {
  name: string;    // CPU name/identifier
  usage: number;   // Current usage (0-100)
  frequency: number; // Current frequency (MHz)
}
```

**Field Details**:

| Field | Type | Units | Example |
|-------|------|-------|---------|
| `name` | `string` | - | `"cpu0"`, `"Intel Core i7"` |
| `usage` | `number` | percent | `45.5` |
| `frequency` | `number` | MHz | `2400` |

**Note**: `usage` is a snapshot. For load average, sample multiple times.

## WASM API

The low-level WASM API. Most users should use the TypeScript wrappers instead.

### SystemMonitor Class

```typescript
class SystemMonitor {
  constructor();
  refresh(): void;
  get_system_info(): string;
  get_memory_info(): string;
  list_disks(): string;
  get_cpu_info(): string;
}
```

#### Constructor

```typescript
const monitor = new SystemMonitor();
```

Creates a new system monitor instance. On native platforms, initializes the `sysinfo` library.

#### refresh()

```typescript
monitor.refresh();
```

Refreshes all system metrics. Call before reading for most up-to-date data.

#### get_system_info()

```typescript
const json: string = monitor.get_system_info();
const info: SystemInfo = JSON.parse(json);
```

Returns JSON string with system information.

#### get_memory_info()

```typescript
const json: string = monitor.get_memory_info();
const info: MemoryInfo = JSON.parse(json);
```

Returns JSON string with memory information.

#### list_disks()

```typescript
const json: string = monitor.list_disks();
const disks: DiskInfo[] = JSON.parse(json);
```

Returns JSON array of disk information.

#### get_cpu_info()

```typescript
const json: string = monitor.get_cpu_info();
const cpus: CpuInfo[] = JSON.parse(json);
```

Returns JSON array of CPU information.

## Usage Examples

### Basic Monitoring

```typescript
import { getSystemInfo, getMemoryInfo } from './tools/system-monitor';

async function basicMonitoring() {
  const sys = await getSystemInfo();
  const mem = await getMemoryInfo();

  console.log(`System: ${sys.os} ${sys.os_version}`);
  console.log(`Hostname: ${sys.hostname}`);
  console.log(`Memory: ${mem.usage_percent.toFixed(1)}% used`);
}
```

### Dashboard Data Collection

```typescript
import {
  getSystemInfo,
  getMemoryInfo,
  listDisks,
  getCpuInfo
} from './tools/system-monitor';

async function collectDashboardData() {
  const [system, memory, disks, cpus] = await Promise.all([
    getSystemInfo(),
    getMemoryInfo(),
    listDisks(),
    getCpuInfo()
  ]);

  return {
    system,
    memory,
    disks,
    cpus,
    timestamp: Date.now()
  };
}
```

### Alert System

```typescript
import { getMemoryInfo, listDisks } from './tools/system-monitor';

async function checkForAlerts() {
  const alerts = [];

  // Check memory
  const mem = await getMemoryInfo();
  if (mem.usage_percent > 90) {
    alerts.push({
      type: 'memory',
      severity: 'critical',
      message: `Memory usage at ${mem.usage_percent.toFixed(1)}%`
    });
  }

  // Check disks
  const disks = await listDisks();
  for (const disk of disks) {
    if (disk.usage_percent > 90) {
      alerts.push({
        type: 'disk',
        severity: 'warning',
        message: `${disk.mount_point} at ${disk.usage_percent.toFixed(1)}%`
      });
    }
  }

  return alerts;
}
```

### Periodic Monitoring

```typescript
import { getMemoryInfo, getCpuInfo } from './tools/system-monitor';

function startMonitoring(intervalMs: number, callback: (data: any) => void) {
  const timer = setInterval(async () => {
    try {
      const [memory, cpus] = await Promise.all([
        getMemoryInfo(),
        getCpuInfo()
      ]);

      const avgCPU = cpus.reduce((sum, cpu) => sum + cpu.usage, 0) / cpus.length;

      callback({
        memory: memory.usage_percent,
        cpu: avgCPU,
        timestamp: Date.now()
      });
    } catch (err) {
      console.error('Monitoring error:', err);
    }
  }, intervalMs);

  return () => clearInterval(timer);
}

// Use it
const stop = startMonitoring(1000, data => {
  console.log(`CPU: ${data.cpu.toFixed(1)}%, Memory: ${data.memory.toFixed(1)}%`);
});

// Later: stop();
```

### AI Agent Usage

```typescript
import { getSystemInfo, getMemoryInfo } from './tools/system-monitor';

async function analyzeSystemHealth() {
  const sys = await getSystemInfo();
  const mem = await getMemoryInfo();

  // Process locally - only summary returned to context
  const issues = [];

  if (mem.usage_percent > 80) {
    issues.push('High memory usage');
  }

  if (sys.uptime < 300) {
    issues.push('Recent reboot detected');
  }

  return {
    status: issues.length === 0 ? 'healthy' : 'degraded',
    issues,
    os: sys.os,
    uptime: sys.uptime
  };
}
```

## Error Handling

### TypeScript Wrapper Errors

All functions may throw on:
- WASM initialization failure
- JSON parse errors (should not occur with valid Rust output)

```typescript
try {
  const info = await getSystemInfo();
} catch (err) {
  console.error('Failed to get system info:', err);
  // Handle error
}
```

### WASM API Errors

The WASM API returns fallback values instead of throwing:

| Condition | Return Value |
|-----------|--------------|
| Missing OS info | `"Unknown"` |
| Serialization error | `"{}"` or `"[]"` |
| Division by zero | `0.0` |
| WASM target | Placeholder JSON |

### Best Practices

```typescript
// Always check for valid data
const mem = await getMemoryInfo();
if (mem.total === 0) {
  console.warn('Invalid memory data');
  return;
}

// Validate percentages
if (mem.usage_percent < 0 || mem.usage_percent > 100) {
  console.warn('Invalid usage percentage');
  return;
}

// Handle missing disks
const disks = await listDisks();
if (disks.length === 0) {
  console.warn('No disks detected (or running in WASM)');
}
```

## Platform Notes

### Native Platforms

Full functionality on:
- Linux (all distributions)
- macOS (10.10+)
- Windows (7+)
- FreeBSD

### WASM Target

Limited functionality:
- Browser security prevents system access
- All methods return placeholder data
- Useful for:
  - Testing
  - UI development
  - Demonstrating code-first pattern

**WASM Return Values**:

```json
// getSystemInfo()
{
  "os": "WASM",
  "os_version": "N/A",
  "kernel_version": "N/A",
  "hostname": "browser",
  "cpu_count": 0,
  "total_memory": 0,
  "used_memory": 0,
  "uptime": 0
}

// getMemoryInfo()
{
  "total": 0,
  "used": 0,
  "available": 0,
  "usage_percent": 0.0
}

// listDisks()
[]

// getCpuInfo()
[]
```

## Performance Tips

### Batch Calls

```typescript
// Good - parallel
const [sys, mem] = await Promise.all([
  getSystemInfo(),
  getMemoryInfo()
]);

// Bad - sequential
const sys = await getSystemInfo();
const mem = await getMemoryInfo();
```

### Cache When Appropriate

```typescript
// Cache static data
let cachedSystemInfo: SystemInfo | null = null;

async function getOS(): Promise<string> {
  if (!cachedSystemInfo) {
    cachedSystemInfo = await getSystemInfo();
  }
  return cachedSystemInfo.os;
}
```

### Avoid Tight Loops

```typescript
// Bad - hammers system
while (true) {
  await getMemoryInfo();
}

// Good - reasonable interval
setInterval(async () => {
  await getMemoryInfo();
}, 1000); // Every second
```

## Related Documentation

- [TypeScript Integration](TypeScript-Integration) - Integration details
- [Core Components](Core-Components) - Rust implementation
- [Architecture](Architecture) - System architecture
- [Code-First Approach](Code-First-Approach) - Usage pattern

---

[Back to Home](Home)
