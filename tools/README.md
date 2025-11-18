# System Monitor Tools

This directory contains TypeScript wrappers for the Rust/WASM system monitor.

## Code-First Approach

Instead of loading 150,000 tokens of MCP tool definitions into context upfront,
AI agents discover these tools by exploring the filesystem structure:

```
tools/
└── system-monitor/
    └── index.ts  (0 tokens until accessed)
```

The agent can:
1. List `./tools/` to discover available tool categories
2. Read `./tools/system-monitor/index.ts` to see available functions
3. Write code that imports and uses these functions

## Token Savings

- Traditional MCP: ~150,000 tokens loaded upfront
- Code-first approach: ~2,000 tokens (only what's needed)
- **Reduction: 98.7%**

## Usage Example

AI agents write code like this:

```typescript
import { getSystemInfo, getMemoryInfo } from './tools/system-monitor';

async function monitorSystem() {
  const sysInfo = await getSystemInfo();
  const memInfo = await getMemoryInfo();

  console.log(`OS: ${sysInfo.os} ${sysInfo.os_version}`);
  console.log(`Memory: ${memInfo.usage_percent.toFixed(1)}% used`);

  if (memInfo.usage_percent > 90) {
    console.warn('High memory usage detected!');
  }
}
```

No tool definitions needed - just TypeScript code with type checking!
