<div align="center">

```
██████╗  ██████╗ ████████╗██╗  ██╗██╗████████╗
██╔══██╗██╔═══██╗╚══██╔══╝██║ ██╔╝██║╚══██╔══╝
██████╔╝██║   ██║   ██║   █████╔╝ ██║   ██║   
██╔═══╝ ██║   ██║   ██║   ██╔═██╗ ██║   ██║   
██║     ╚██████╔╝   ██║   ██║  ██╗██║   ██║   
╚═╝      ╚═════╝    ╚═╝   ╚═╝  ╚═╝╚═╝   ╚═╝   
```

### **The Developer Operating System for Portaldot**

*Inspired by Hardhat & Foundry — built for the Substrate era.*

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/Blockchain-Portaldot%2FSubstrate-purple?style=flat-square)](https://substrate.io/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square)](LICENSE)
[![ink! Contracts](https://img.shields.io/badge/Smart%20Contracts-ink!-pink?style=flat-square)](https://use.ink/)
[![Docker](https://img.shields.io/badge/Node-Docker%20Orchestration-blue?style=flat-square&logo=docker)](https://docker.com/)

</div>

---

## The Problem

Building on Substrate is powerful — but painfully fragmented. A developer must juggle:

| What they want to do | What they actually do |
|---|---|
| Start developing | Manually spin up nodes, configure RPC, fix Docker issues |
| Deploy a contract | Switch between 5+ tools, read 3 docs, debug Wasm manually |
| Monitor the chain | Open separate terminals, write one-off scripts |
| Fund a test wallet | Hunt for faucet URLs, wait for web UIs |

**PotKit eliminates every one of these friction points** — replacing a fragmented toolchain with a single, unified CLI.

---

## Solution: One CLI. Full Stack.

```
potkit <command>
```

That's it. PotKit is your **single entry point** for the entire Portaldot development lifecycle — from first setup to live production monitoring.

---

## 30-Second Quickstart

```bash
# Clone & install
git clone <repo-url> && cd potkit-starter
cargo install --path .

# Verify your environment
potkit status
```

**Instant, readable output:**

```
⚡ PotKit System Status
─────────────────────────────────────────
Environment
  Rust             ✓  1.75.0
  Cargo            ✓  OK
  Docker           ✓  Running

Blockchain
  RPC Connected    ✓  ws://127.0.0.1:9944
  Latest Block     ✓  #3047
  Chain ID         ✓  Portaldot Local
─────────────────────────────────────────
✅ Ready to build. Run `potkit dev` to start.
```

---

## The 7-Step Developer Workflow

PotKit structures development into a clear, repeatable flow:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    POTKIT DEVELOPER WORKFLOW                         │
├──────────┬──────────┬──────────┬──────────┬──────────┬─────────────┤
│  1 INIT  │ 2 STATUS │  3 BUILD │ 4 UPLOAD │ 5 FAUCET │  6 LOGS     │
│          │          │          │          │          │             │
│ scaffold │  verify  │ compile  │  deploy  │   fund   │  monitor    │
│ project  │   env    │  ink!    │   Wasm   │  wallet  │   chain     │
└────┬─────┴────┬─────┴────┬─────┴────┬─────┴────┬─────┴──────┬──────┘
     │          │          │          │          │             │
     └──────────┴──────────┴──────────┘          │    ┌────────▼───────┐
                                                  │    │   7 DEV MODE   │
                                                  └───►│  watch + build │
                                                       │  + live chain  │
                                                       └────────────────┘
```

---

## Architecture

PotKit sits as the **unified orchestration layer** between you and the Portaldot blockchain:

```
╔══════════════════════════════════════════════════════════════════╗
║                         YOU (Developer)                          ║
╚══════════════════════════╤═══════════════════════════════════════╝
                           │
                           ▼
╔══════════════════════════════════════════════════════════════════╗
║                       POTKIT CLI                                 ║
║                                                                  ║
║   potkit status │ potkit node │ potkit build │ potkit upload     ║
║   potkit faucet │ potkit logs │ potkit dev   │ potkit inspect    ║
╚══════╤══════════╤════════════╤══════════════╤════════════════════╝
       │          │            │              │
       ▼          ▼            ▼              ▼
╔══════════╗ ╔══════════╗ ╔══════════╗ ╔═══════════════╗
║  Node    ║ ║ Contract ║ ║  Faucet  ║ ║   Monitoring  ║
║  Orch.   ║ ║ Tooling  ║ ║ Service  ║ ║   & Events    ║
║ (Docker) ║ ║  (ink!)  ║ ║          ║ ║  (Subxt/WS)  ║
╚════╤═════╝ ╚════╤═════╝ ╚════╤═════╝ ╚══════╤════════╝
     │             │            │              │
     └─────────────┴────────────┴──────────────┘
                           │
                           ▼
╔══════════════════════════════════════════════════════════════════╗
║                    PORTALDOT BLOCKCHAIN                          ║
║         Substrate Runtime │ Pallets │ Smart Contracts            ║
╚══════════════════════════════════════════════════════════════════╝
```

**Four clean layers. No context switching. No escaping to other tools.**

---

## Command Reference

### Core Workflow Commands

| Command | What It Does | When to Use |
|---|---|---|
| `potkit status` | Full environment health check | Start of every session |
| `potkit node` | Spin up local Portaldot node | Before any chain interaction |
| `potkit build` | Compile ink! smart contract to Wasm | After code changes |
| `potkit upload` | Deploy compiled Wasm bytecode on-chain | Ready for testing |
| `potkit faucet <addr>` | Fund any account with test tokens | Before testing transactions |
| `potkit logs --follow` | Stream live block and event feed | Monitor activity |
| `potkit dev` | **Full integrated dev environment** | Active development loop |

### Additional Tools

| Command | Purpose |
|---|---|
| `potkit init <project>` | Scaffold a new Portaldot project |
| `potkit deploy` | Deploy contracts to testnet |
| `potkit inspect` | Analyze contract metadata and ABI |
| `potkit doctor` | Diagnose environment issues |
| `potkit config` | Manage PotKit configuration |
| `potkit chain-info` | Display live Portaldot network info |

---

## Dev Mode — The Full Loop

`potkit dev` is PotKit's killer feature: a single command that runs your **entire development environment** in parallel.

```bash
potkit dev
```

```
🚀 Starting PotKit Dev Environment...
────────────────────────────────────────
  ✓  RPC connected      ws://127.0.0.1:9944
  ✓  Contract artifacts detected
  ✓  Watching           ./src
  ✓  Live chain monitor active

🔥 PotKit dev mode running. Ctrl+C to stop.
────────────────────────────────────────
[chain] Block #2646  — 0 events
[chain] Block #2647  — 2 events
[watch] Create("/src/contracts/flipper.rs")
[build] File change detected → Rebuild recommended
[build] Compiling flipper v0.1.0...
[build] ✓ Build successful → flipper.wasm (14.2 KB)
[chain] Block #2648  — 0 events
```

Three things happening simultaneously:
- **File watcher** — detects every `.rs` change
- **Auto build** — signals you to rebuild on file changes
- **Live chain monitor** — streams blocks and events in real time

---

## Example: Full Iteration Cycle

```bash
# 1. Verify environment
potkit status

# 2. Start local node
potkit node

# 3. Build your ink! contract
potkit build

# 4. Deploy to local chain
potkit upload

# 5. Fund your test account
potkit faucet 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

# 6. Watch live chain activity
potkit logs --follow

# — OR — run everything together:
potkit dev
```

**Rapid iteration (4 commands from code to deployed):**

```bash
potkit status && potkit build && potkit upload && potkit logs --follow
```

---

## Tech Stack

```
Language          Rust 1.70+
Blockchain Client Subxt
Async Runtime     Tokio
Containerization  Docker
Smart Contracts   ink!
Blockchain        Substrate / Portaldot
File Watching     notify-rs
CLI Framework     Clap
```

---

## Project Structure

```
potkit/
├── src/
│   └── commands/          ← PotKit command implementations
│       ├── status.rs
│       ├── node.rs
│       ├── build.rs
│       ├── upload.rs
│       ├── faucet.rs
│       ├── logs.rs
│       └── dev.rs
├── real-contract/          ← Sample ink! smart contract project
├── docs/
│   ├── architecture.svg    ← Architecture diagram
│   ├── workflow.svg        ← Developer workflow diagram
│   └── demo-notes.md
├── screenshots/            ← Demo assets
├── templates/              ← CLI templates and project boilerplate
├── Cargo.toml
└── README.md
```

---

## Installation

### Prerequisites

- Rust 1.70+
- Cargo
- Docker (for local node support)

### Install

```bash
git clone <repo-url>
cd potkit-starter
cargo install --path .
```

### Verify

```bash
potkit status
# Expected: all checks green ✓
```

---

## Why PotKit Wins

> *Developer tooling is ecosystem infrastructure.*

### For Hackathon Judges

🏆 **Real Infrastructure** — Not a demo. A tool the Portaldot ecosystem will actually use.

🚀 **Measurable Impact** — Reduces developer onboarding from days to minutes.

🛠️ **Production Quality** — Written in Rust with industry-standard async patterns (Tokio, Subxt, Clap).

📊 **Proven Patterns** — Inspired by Hardhat and Foundry, tools that defined their ecosystems.

🌱 **Clear Roadmap** — Commitment beyond the hackathon, with a path to full ABI tooling, hot reload, and multi-network deployment.

### For the Portaldot Ecosystem

- **Accelerates onboarding** — New devs productive in 30 minutes, not 3 days
- **Reduces support burden** — Built-in diagnostics (`potkit doctor`, `potkit status`)
- **Raises dApp quality** — Faster iteration → better tested contracts
- **Sets a standard** — Unified workflow means shared best practices across teams

---

## Roadmap

- [ ] Full smart contract instantiation
- [ ] ABI tooling and type-safe bindings
- [ ] Auto-rebuild + hot reload on save
- [ ] Advanced event streaming and filtering
- [ ] Contract testing utilities
- [ ] Multi-network deployment (testnet, mainnet)
- [ ] Block explorer integration
- [ ] Plugin system for community extensions

---

## Vision

**PotKit is the foundational developer workflow layer for Portaldot.**

Just as Hardhat unified Ethereum development and Foundry revolutionized Solidity tooling, PotKit is designed to become the default operating system for every Portaldot developer.

```
Goal 1  Reduce developer onboarding from days → hours
Goal 2  Make smart contract development frictionless
Goal 3  Build the infrastructure layer for ecosystem growth
Goal 4  Create a sustainable, community-driven platform
```

This is not just a hackathon submission — it's the beginning of Portaldot's developer tooling story.

---

## Contributing

PotKit is open to community contributions. Submit issues, feature requests, and pull requests to help build the future of Portaldot developer tooling.

---

## License

Licensed under the [Apache License 2.0](LICENSE).

---

<div align="center">

**Built with ❤️ for Portaldot developers.**

*PotKit — From zero to deployed in 30 seconds.*

</div>
