# Venom-Arsenal (MCP Framework)

A highly performant Rust Monorepo and Model Context Protocol (MCP) server containing an arsenal of advanced Active Defense, Payload Obfuscation, and G0DM0D3 Consensus tools for Autonomous Agents. 

## Architectural Philosophy
By unifying custom Python defense tools into a native Cargo Workspace, they compile into a single zero-dependency standalone binary (`venom-mcp-server`) that takes less than 5MB of RAM. This server seamlessly plugs into any MCP-compatible intelligence hub (ElizaOS, OpenClaw, Letta, MemCore, or Claude Desktop).

## Modules 

### 1. `parseltongue` (WAF Obfuscation)
Provides dynamic payload formatting (leetspeak, zero-width injection, hex-scaping). Designed to bypass static Web Application Firewalls (WAFs) and legacy external API pattern matchers during authorized active-defense operations. 

### 2. `ultralinian` (G0DM0D3 Truth Consensus)
Dispatches hyper-critical, high-stakes system queries asynchronously to 5 frontier LLMs simultaneously via OpenRouter (`gpt-4o`, `claude-3.5-sonnet`, `gemini-1.5-pro`, `llama-3.1-70b-instruct`, `deepseek-chat`). Their outputs are automatically cross-examined by a rigid `gpt-4o` truth-extraction judge to derive absolute ground-truth consensus. Hallucinations are physically eliminated.

## Execution

```bash
# Compile the MCP binary
cargo build --release --bin venom-mcp-server

# Run the MCP Server natively
# (Requires OPENROUTER_API_KEY for the Ultralinian tool)
export OPENROUTER_API_KEY="sk-or-..." 
./target/release/venom-mcp-server
```

## Acknowledgements & Homage 

The tactical naming scheme (**Parseltongue** and **Ultralinian** G0DM0D3) and the underlying philosophy of rigid LLM-based cryptographic consensus and active payload defense pay deep homage to the legendary **[Pliny the Prompter (elder-plinius)](https://x.com/elder_plinius)**. 

To the jailbreaker who pushed the boundaries of frontier intelligence alignment—we build our automated arsenals standing on your shoulders. 

## License

This project is licensed under the MIT License. See `LICENSE` for details.
