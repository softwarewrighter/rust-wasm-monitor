# Architecture

This document provides a comprehensive overview of the Rust/WASM System Monitor architecture, including component diagrams, data flows, and interaction patterns.

## Table of Contents

- [Overview](#overview)
- [System Architecture](#system-architecture)
- [Component Architecture](#component-architecture)
- [Data Flow](#data-flow)
- [Sequence Diagrams](#sequence-diagrams)
- [Build Process](#build-process)
- [Technology Stack](#technology-stack)

## Overview

The Rust/WASM System Monitor is built on a layered architecture that separates concerns between system access, WASM interoperability, and TypeScript integration. This design enables both direct WASM usage and a code-first discovery pattern for AI agents.

### Design Principles

1. **Separation of Concerns**: Clear boundaries between Rust core, WASM bindings, and TypeScript wrappers
2. **Progressive Discovery**: Zero tokens until tools are accessed
3. **Type Safety**: End-to-end type safety from Rust to TypeScript
4. **Performance First**: Optimized compilation and minimal overhead
5. **Zero Dependencies**: No external services or API keys required

## System Architecture

### High-Level Architecture

```mermaid
graph TB
    subgraph "Host Operating System"
        OS[Operating System APIs]
        FS[Filesystem]
        MEM[Memory]
        CPU[CPU]
        DISK[Disk]
    end

    subgraph "Rust Core Layer"
        SYSINFO[sysinfo crate]
        STRUCTS[Data Structures]
        IMPL[SystemMonitor impl]
    end

    subgraph "WASM Interop Layer"
        BINDGEN[wasm-bindgen]
        WASM[WASM Binary<br/>17KB]
        JSON[JSON Serialization]
    end

    subgraph "TypeScript Layer"
        WRAPPERS[Type-Safe Wrappers]
        TYPES[TypeScript Types]
        INIT[Lazy Initialization]
    end

    subgraph "Consumption Layer"
        BROWSER[Browser]
        NODE[Node.js]
        AGENT[AI Agents]
        DEMO[Demo UI]
    end

    OS --> SYSINFO
    FS --> SYSINFO
    MEM --> SYSINFO
    CPU --> SYSINFO
    DISK --> SYSINFO

    SYSINFO --> STRUCTS
    STRUCTS --> IMPL
    IMPL --> BINDGEN
    BINDGEN --> WASM
    IMPL --> JSON
    JSON --> WASM

    WASM --> WRAPPERS
    WRAPPERS --> TYPES
    TYPES --> INIT

    INIT --> BROWSER
    INIT --> NODE
    INIT --> AGENT
    INIT --> DEMO
```

### Layer Responsibilities

#### 1. Rust Core Layer
- System information collection via `sysinfo` crate
- Data structure definitions with Serialize/Deserialize
- Business logic implementation
- Cross-platform compatibility handling

#### 2. WASM Interop Layer
- `wasm-bindgen` FFI bindings
- JSON serialization for JavaScript compatibility
- Platform-specific compilation (`#[cfg]` attributes)
- Optimized binary generation

#### 3. TypeScript Layer
- Type-safe function wrappers
- Lazy WASM initialization
- JSON parsing and type conversion
- Ergonomic API for consumers

#### 4. Consumption Layer
- Browser integration
- Node.js integration
- AI agent code-first discovery
- Interactive demo application

## Component Architecture

### Component Diagram

```mermaid
graph LR
    subgraph "src/lib.rs"
        A[SystemInfo struct]
        B[MemoryInfo struct]
        C[DiskInfo struct]
        D[CpuInfo struct]
        E[SystemMonitor struct]
        F[impl SystemMonitor]
    end

    subgraph "tools/system-monitor/index.ts"
        G[getSystemInfo]
        H[getMemoryInfo]
        I[listDisks]
        J[getCpuInfo]
        K[refresh]
        L[init]
    end

    subgraph "pkg/"
        M[rust_wasm_monitor.js]
        N[rust_wasm_monitor_bg.wasm]
        O[rust_wasm_monitor.d.ts]
    end

    A --> F
    B --> F
    C --> F
    D --> F
    E --> F
    F --> M
    F --> N
    F --> O

    M --> L
    N --> L
    O --> L

    L --> G
    L --> H
    L --> I
    L --> J
    L --> K
```

### Core Components

#### SystemMonitor (Rust)
```rust
pub struct SystemMonitor {
    #[cfg(not(target_arch = "wasm32"))]
    sys: System,
}
```
- Main entry point for system monitoring
- Wraps `sysinfo::System` on native platforms
- Empty struct on WASM target (data fetched on-demand)

#### Data Structures (Rust)
```rust
pub struct SystemInfo { ... }
pub struct MemoryInfo { ... }
pub struct DiskInfo { ... }
pub struct CpuInfo { ... }
```
- Serializable structures for cross-language communication
- JSON-compatible field types
- Comprehensive system metrics

#### TypeScript Wrappers
```typescript
export async function getSystemInfo(): Promise<SystemInfo>
export async function getMemoryInfo(): Promise<MemoryInfo>
export async function listDisks(): Promise<DiskInfo[]>
export async function getCpuInfo(): Promise<CpuInfo[]>
```
- Type-safe async functions
- Automatic JSON parsing
- Lazy WASM initialization

## Data Flow

### Initialization Flow

```mermaid
sequenceDiagram
    participant User
    participant TS as TypeScript Wrapper
    participant WASM as WASM Binary
    participant Rust as Rust Core
    participant OS as Operating System

    User->>TS: import { getSystemInfo }
    Note over User,TS: Zero tokens until first call

    User->>TS: await getSystemInfo()
    TS->>TS: Check if monitor initialized
    alt Not initialized
        TS->>WASM: Load WASM module
        WASM->>Rust: SystemMonitor::new()
        Rust->>OS: Initialize sysinfo
        OS-->>Rust: System handle
        Rust-->>WASM: SystemMonitor instance
        WASM-->>TS: Monitor ready
    end

    TS->>WASM: monitor.get_system_info()
    WASM->>Rust: Call get_system_info()
    Rust->>OS: Query system info
    OS-->>Rust: Raw system data
    Rust->>Rust: Create SystemInfo struct
    Rust->>Rust: Serialize to JSON
    Rust-->>WASM: JSON string
    WASM-->>TS: JSON string
    TS->>TS: Parse JSON
    TS-->>User: Typed SystemInfo object
```

### Data Collection Flow

```mermaid
flowchart TD
    START([Function Called]) --> CHECK{Monitor<br/>Initialized?}
    CHECK -->|No| LOAD[Load WASM Module]
    CHECK -->|Yes| REFRESH
    LOAD --> CREATE[Create SystemMonitor]
    CREATE --> REFRESH[Refresh System Data]

    REFRESH --> COLLECT[Collect Metrics]

    COLLECT --> SYS[System Info]
    COLLECT --> MEM[Memory Info]
    COLLECT --> DISK[Disk Info]
    COLLECT --> CPU[CPU Info]

    SYS --> STRUCT1[SystemInfo struct]
    MEM --> STRUCT2[MemoryInfo struct]
    DISK --> STRUCT3[DiskInfo struct]
    CPU --> STRUCT4[CpuInfo struct]

    STRUCT1 --> SERIALIZE1[Serialize to JSON]
    STRUCT2 --> SERIALIZE2[Serialize to JSON]
    STRUCT3 --> SERIALIZE3[Serialize to JSON]
    STRUCT4 --> SERIALIZE4[Serialize to JSON]

    SERIALIZE1 --> RETURN[Return JSON String]
    SERIALIZE2 --> RETURN
    SERIALIZE3 --> RETURN
    SERIALIZE4 --> RETURN

    RETURN --> PARSE[Parse JSON in TypeScript]
    PARSE --> TYPED[Return Typed Object]
    TYPED --> END([Complete])
```

## Sequence Diagrams

### AI Agent Discovery Pattern

```mermaid
sequenceDiagram
    participant Agent as AI Agent
    participant FS as Filesystem
    participant Tools as tools/system-monitor/index.ts
    participant WASM as WASM Runtime
    participant OS as Operating System

    Note over Agent: Agent explores codebase
    Agent->>FS: ls ./tools/
    FS-->>Agent: system-monitor/

    Agent->>FS: ls ./tools/system-monitor/
    FS-->>Agent: index.ts

    Agent->>FS: cat ./tools/system-monitor/index.ts
    FS-->>Agent: Function signatures & types

    Note over Agent: Agent writes code
    Agent->>Agent: import { getSystemInfo, getMemoryInfo }

    Note over Agent: Agent executes code
    Agent->>Tools: await getSystemInfo()
    Tools->>WASM: Initialize & call
    WASM->>OS: Query system
    OS-->>WASM: System data
    WASM-->>Tools: JSON data
    Tools-->>Agent: Typed SystemInfo

    Agent->>Tools: await getMemoryInfo()
    Tools->>WASM: Call get_memory_info
    WASM->>OS: Query memory
    OS-->>WASM: Memory data
    WASM-->>Tools: JSON data
    Tools-->>Agent: Typed MemoryInfo

    Note over Agent: Agent processes data locally
    Agent->>Agent: if (mem.usage_percent > 90) { alert() }
    Note over Agent: Only summary returned to context
```

### Browser Usage Flow

```mermaid
sequenceDiagram
    participant Browser
    participant HTML as demo.html
    participant JS as JavaScript
    participant WASM as WASM Module
    participant Rust as Rust Core

    Browser->>HTML: Load page
    HTML->>JS: Load script
    JS->>WASM: import init, { SystemMonitor }
    JS->>WASM: await init()
    WASM->>Rust: Initialize WASM runtime
    Rust-->>WASM: Ready

    JS->>WASM: monitor = SystemMonitor.new()
    WASM->>Rust: SystemMonitor::new()
    Rust-->>WASM: Instance created

    loop Every 2 seconds
        JS->>WASM: monitor.get_system_info()
        WASM->>Rust: Collect system info
        Rust-->>WASM: JSON string
        WASM-->>JS: JSON string
        JS->>JS: Parse & update UI

        JS->>WASM: monitor.get_memory_info()
        WASM->>Rust: Collect memory info
        Rust-->>WASM: JSON string
        WASM-->>JS: JSON string
        JS->>JS: Parse & update UI

        JS->>WASM: monitor.list_disks()
        WASM->>Rust: Collect disk info
        Rust-->>WASM: JSON array
        WASM-->>JS: JSON array
        JS->>JS: Parse & update UI

        JS->>WASM: monitor.get_cpu_info()
        WASM->>Rust: Collect CPU info
        Rust-->>WASM: JSON array
        WASM-->>JS: JSON array
        JS->>JS: Parse & update UI
    end
```

## Build Process

### Build Pipeline

```mermaid
flowchart LR
    START([./build.sh]) --> CARGO[Cargo Build]

    CARGO --> RUSTC[rustc Compiler]
    RUSTC --> OPT[Optimization Pass]

    OPT --> LTO[Link-Time<br/>Optimization]
    OPT --> SIZE[Size Optimization]
    OPT --> STRIP[Strip Debug Info]

    LTO --> WASM32[wasm32-unknown-unknown]
    SIZE --> WASM32
    STRIP --> WASM32

    WASM32 --> BINDGEN[wasm-bindgen]
    BINDGEN --> JS[.js bindings]
    BINDGEN --> WASMBIN[.wasm binary]
    BINDGEN --> DTS[.d.ts types]

    JS --> PKG[pkg/ directory]
    WASMBIN --> PKG
    DTS --> PKG

    PKG --> END([Build Complete<br/>~5 seconds])
```

### Build Optimization Flow

```mermaid
graph TB
    subgraph "Source Code"
        A[src/lib.rs<br/>273 lines]
        B[Cargo.toml<br/>Config]
    end

    subgraph "Compilation Flags"
        C[opt-level = 'z']
        D[lto = true]
        E[codegen-units = 1]
        F[strip = true]
    end

    subgraph "Compilation"
        G[rustc]
        H[LLVM Optimization]
    end

    subgraph "Output"
        I[WASM Binary<br/>17KB]
        J[JS Bindings<br/>~30KB]
        K[TypeScript Defs<br/>~5KB]
    end

    A --> G
    B --> C
    B --> D
    B --> E
    B --> F
    C --> H
    D --> H
    E --> H
    F --> H
    G --> H

    H --> I
    H --> J
    H --> K
```

## Technology Stack

### Language & Runtime
- **Rust**: 2024 edition for core implementation
- **WebAssembly**: wasm32-unknown-unknown target
- **TypeScript**: Type-safe wrapper layer
- **JavaScript**: ES6+ modules

### Key Dependencies

#### Rust
- `sysinfo` (0.33.0): Cross-platform system information
- `wasm-bindgen` (0.2): Rust/WASM/JavaScript interop
- `serde` (1.0): Serialization framework
- `serde_json` (1.0): JSON serialization

#### Build Tools
- `cargo`: Rust build system
- `wasm-pack`: WASM build tool
- `rustc`: Rust compiler
- `LLVM`: Optimization backend

### Platform Support

#### Native Platforms (via sysinfo)
- Linux (all distributions)
- macOS (10.10+)
- Windows (7+)
- FreeBSD
- Other Unix-like systems

#### WASM Platforms
- Modern browsers (Chrome, Firefox, Safari, Edge)
- Node.js (with WASM support)
- Deno
- Browser extensions

## Performance Characteristics

### Build Performance
- **Compilation Time**: ~5 seconds (release build)
- **Binary Size**: 17KB (WASM, optimized)
- **JS Bindings Size**: ~30KB
- **TypeScript Defs**: ~5KB

### Runtime Performance
- **Initialization**: <100ms first call
- **Subsequent Calls**: <10ms
- **Memory Overhead**: <1MB
- **CPU Overhead**: Negligible

### Token Efficiency
- **Traditional MCP**: ~150,000 tokens
- **Code-First**: ~2,000 tokens
- **Reduction**: 98.7%

## Security Considerations

### Sandboxing
- WASM runs in browser security sandbox
- No filesystem access beyond what OS provides
- No network access in core library
- Read-only system information queries

### Data Privacy
- All data processing happens locally
- No external API calls
- No telemetry or tracking
- Data never leaves execution environment

### Type Safety
- Rust's type system prevents memory unsafety
- TypeScript provides compile-time type checking
- JSON schema validation via serde
- No runtime type coercion errors

## Extensibility

### Adding New Metrics

```mermaid
flowchart TD
    ADD[Add New Metric] --> STRUCT[Define Rust Struct]
    STRUCT --> IMPL[Implement Collection]
    IMPL --> WASM[Add wasm_bindgen Method]
    WASM --> TS[Add TypeScript Wrapper]
    TS --> TYPES[Export TypeScript Types]
    TYPES --> BUILD[Rebuild WASM]
    BUILD --> USE[Use in Agent Code]
```

### Extension Points

1. **Rust Core**: Add new data structures and collection methods
2. **WASM Bindings**: Expose new methods via `#[wasm_bindgen]`
3. **TypeScript Layer**: Create type-safe wrappers
4. **Discovery**: New tools automatically discoverable by agents

## Comparison with Traditional Architectures

### Traditional MCP Architecture

```mermaid
graph TB
    A[AI Model] --> B[MCP Client]
    B --> C[Load All Tool Definitions<br/>150,000 tokens]
    C --> D[MCP Server]
    D --> E[Tool Implementation]
    E --> F[System APIs]
    F --> E
    E --> D
    D --> C
    C --> B
    B --> A
```

### Code-First Architecture

```mermaid
graph TB
    A[AI Agent] --> B[Explore Filesystem]
    B --> C[Read Tool Files<br/>~2,000 tokens]
    C --> D[Write Code]
    D --> E[TypeScript Execution]
    E --> F[WASM Call]
    F --> G[Rust Implementation]
    G --> H[System APIs]
    H --> G
    G --> F
    F --> E
    E --> D
    D --> I[Process Locally]
    I --> J[Return Summary]
    J --> A
```

## Related Documentation

- [Core Components](Core-Components.md) - Detailed Rust implementation
- [TypeScript Integration](TypeScript-Integration.md) - TypeScript wrapper details
- [Build System](Build-System.md) - Build process and optimization
- [Code-First Approach](Code-First-Approach.md) - Discovery pattern details

---

[Back to Home](Home.md)
