/**
 * System Monitor - TypeScript Integration
 *
 * Code-first approach: Instead of loading 150,000 tokens of MCP tool definitions,
 * agents discover these functions via filesystem and call them directly.
 *
 * This achieves 98.7% token reduction as described in Anthropic's MCP paper.
 */

import { SystemMonitor } from '../../pkg/rust_wasm_monitor';

let monitor: SystemMonitor | null = null;

/**
 * Initialize the system monitor
 * Lazy-loaded only when needed - 0 tokens until this function is called
 */
export async function init(): Promise<void> {
  if (!monitor) {
    monitor = new SystemMonitor();
  }
}

/**
 * Get basic system information
 *
 * @returns System info object with OS, memory, uptime, etc.
 */
export async function getSystemInfo(): Promise<SystemInfo> {
  await init();
  const json = monitor!.get_system_info();
  return JSON.parse(json);
}

/**
 * Get memory usage information
 *
 * @returns Memory info with total, used, available, and usage percentage
 */
export async function getMemoryInfo(): Promise<MemoryInfo> {
  await init();
  const json = monitor!.get_memory_info();
  return JSON.parse(json);
}

/**
 * List all disk mounts
 *
 * @returns Array of disk information objects
 */
export async function listDisks(): Promise<DiskInfo[]> {
  await init();
  const json = monitor!.list_disks();
  return JSON.parse(json);
}

/**
 * Get CPU information
 *
 * @returns Array of CPU info objects with usage and frequency
 */
export async function getCpuInfo(): Promise<CpuInfo[]> {
  await init();
  const json = monitor!.get_cpu_info();
  return JSON.parse(json);
}

/**
 * Refresh all system data
 */
export async function refresh(): Promise<void> {
  await init();
  monitor!.refresh();
}

// Type definitions
export interface SystemInfo {
  os: string;
  os_version: string;
  kernel_version: string;
  hostname: string;
  cpu_count: number;
  total_memory: number;
  used_memory: number;
  uptime: number;
}

export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  usage_percent: number;
}

export interface DiskInfo {
  name: string;
  mount_point: string;
  total_space: number;
  available_space: number;
  usage_percent: number;
}

export interface CpuInfo {
  name: string;
  usage: number;
  frequency: number;
}
