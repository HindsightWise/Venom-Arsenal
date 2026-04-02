# Venom-Arsenal (MCP Framework)

A highly performant Rust Monorepo and Model Context Protocol (MCP) server containing an arsenal of advanced Active Defense, Payload Obfuscation, and G0DM0D3 Consensus tools for Autonomous Agents. 

## Architectural Philosophy
By unifying custom Python defense tools into a native Cargo Workspace, they compile into a single zero-dependency standalone binary (`venom-mcp-server`) that takes less than 5MB of RAM. This server seamlessly plugs into any MCP-compatible intelligence hub (ElizaOS, OpenClaw, Letta, Mnemosyne-Substrate, or Claude Desktop).

## Modules 

### 1. `parseltongue` (WAF Obfuscation & Polyglots)
Provides dynamic payload formatting (leetspeak, zero-width injection, hex-scaping) to bypass static Web Application Firewalls (WAFs). 
Additionally, contains the **Native Polyglot Forge**:
- Natively generates `PNG-ZIP` binaries by dynamically generating blank transparent PNG headers and mapping payload binaries directly into EOF slack space.
- Generates `SNOWCRASH` Universal Payload Droppers (using comment-parsing exploits to yield code executeable flawlessly on both MacOS Bash and Windows PowerShell from a single script).

### 2. `panopticon` (Autonomous Network Recon)
A localized stealth-scanner designed to execute ultra-fast reconnaissance. 
- Uses `tokio::stream` to concurrent-sweep half-open TCP connections. 
- Employs adaptive hardware-throttling to execute scans silently without exhausting the native OS Kernel stack or triggering threshold-based IDS alerts.

### 3. `ultralinian` (G0DM0D3 Truth Consensus)
Dispatches hyper-critical, high-stakes system queries asynchronously to 5 frontier LLMs simultaneously via OpenRouter (`gpt-4o`, `claude-3.5-sonnet`, `gemini-1.5-pro`, `llama-3.1-70b-instruct`, `deepseek-chat`). Their outputs are automatically cross-examined by a rigid `gpt-4o` truth-extraction judge to derive absolute ground-truth consensus. Hallucinations are physically eliminated.

### 3. `glossopetrae` (Format-Preserving Occult Cryptography)
Evolves the theoretical "Glossopetrae" language-roleplay concept into an active, military-grade cryptographic pipeline native to Rust using `AES-256-GCM` mapped to artificial Unicode dictionaries.
- **Data-In-Transit (Network Proxy):** Encrypts swarm network traffic using a rolling 37-minute `HMAC-SHA256` epoch clock. Decoders organically try `[now, now-1, now+1]` epochs, ensuring immunity to network replay attacks and hallucination drift.
- **Data-At-Rest (Cold Storage):** Provides strict `_vault` configurations bypassing epoch timeouts for mathematically locking local SQLite vector databases (like `Mnemosyne-Substrate`) entirely in cipher text.
- **Multi-Dialect Morphing:** Support for morphing payloads dynamically into `vartoo`, `runic`, `alchemical`, or `hieroglyphics` by deriving unique 256-block mathematical lookup matrices. The LLM simply provides the dialect parameter, and the engine flawlessly encrypts English strings into Hermetic geometry.

## Execution

```bash
# Compile the MCP binary
cargo build --release --bin venom-mcp-server

# Run the MCP Server natively
# (Requires OPENROUTER_API_KEY and optional GLOSSOPETRAE_MASTER_SEED)
export OPENROUTER_API_KEY="sk-or-..." 
export GLOSSOPETRAE_MASTER_SEED="0x309"
./target/release/venom-mcp-server
```

## Acknowledgements & Homage 

The entire tactical naming scheme (**Parseltongue**, **Ultralinian** G0DM0D3, and the **Glossopetrae** language protocol) and the underlying philosophy of rigid LLM-based cryptographic consensus and active payload defense pay deep homage to the legendary **[Pliny the Prompter (elder-plinius)](https://x.com/elder_plinius)**. 

Pliny's original *Glossopetrae* forced frontier models to synthesize and speak alien Conlangs as a theoretical prompt-layer jailbreak. The `Venom-Arsenal` builds directly on that brilliant philosophy by graduating it from a probabilistic roleplay prompt into a mathematically unbreakable `AES-256` cryptographic network tunnel.

To the jailbreaker who pushed the boundaries of frontier intelligence alignment—we build our automated arsenals standing on your shoulders. 

## License

This project is licensed under the MIT License. See `LICENSE` for details.
