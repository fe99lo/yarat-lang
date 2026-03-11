🏦 YaraT (v1.0)

The Mathematical Standard for the Next 200 Years of Finance.

YaraT is a high-level, strictly-typed Domain-Specific Language (DSL) and Enterprise Web API built from the ground up in Rust. It is engineered specifically for modern banking, payment aggregators, and financial reconciliation systems.

Traditional programming languages treat money as simple floating-point numbers, leading to catastrophic rounding errors and accidental currency mixing. YaraT treats money as a physical, mathematically protected entity.

🛡️ Core Architectural Pillars
Military-Grade Security (Semantic Auditing):
YaraT physically isolates currencies at the compiler level. The engine will instantly abort any transaction that attempts to add `KES` to `USD`, or perform math on a logical flag, preventing ledger corruption before the code even executes.Lightning Speed (< 1ms Execution):Built on Rust's zero-cost abstractions, the YaraT engine lexes, parses, audits, and executes financial smart contracts in under one millisecond.
Ultra-Low Memory Footprint: Designed to run on anything from a high-end cloud cluster to a standard office laptop. The engine strictly manages its own memory state without heavy garbage collection, preventing CPU and RAM spikes.
Enterprise Web API: YaraT natively wraps its compiler in a high-concurrency Actix-Web server, complete with CORS security, 4KB payload limits (Buffer Overflow protection), and execution telemetry.


💻 The Language Syntax

YaraT is designed to be readable by both software engineers and financial auditors.

```text
// 1. Initialize System Currencies
asset KES = Fiat(precision: 2)

// 2. Global Ledgers
treasury_vault = 1000000.00 KES
total_disbursed = 0.00 KES

// 3. Reusable Smart Contracts (Strictly Typed)
transaction ProcessLoan(loan_amount: KES, fee: KES, user_wallet: KES) {
    if treasury_vault > loan_amount {
        net_payout = loan_amount - fee
        
        // Safely mutate ledgers
        treasury_vault = treasury_vault - net_payout
        user_wallet = user_wallet + net_payout
        total_disbursed = total_disbursed + net_payout
        
        status_success = true
    } else {
        status_success = false
    }
}

```
 🚀 Getting Started

Ensure you have Rust installed, then clone this repository. YaraT operates in two distinct modes: **Local Execution** and **Enterprise Server**.

Mode 1: Local Terminal Execution

Perfect for testing scripts and running localized financial audits.

```bash
# Command format
cargo run -- run <path-to-file.yt>

# Example
cargo run -- run examples/finance_audit.yt

```

Output includes a complete Semantic Audit report and a structured printout of the Final Memory State.*

 Mode 2: Enterprise Web API

Boot the engine into a live HTTP server to connect it to web dashboards and mobile applications.

```bash
cargo run -- serve

```

Executing via API (JSON Payload):**
Send a POST request to `http://127.0.0.1:8080/api/v1/execute` with your YaraT code.

```json
// POST Payload
{
    "code": "asset USD = Fiat(precision: 2)\nvault = 500.00 USD\nfee = 10.00 USD\nnet = vault - fee"
}

```

API Response:

```json
{
    "success": true,
    "execution_time_ms": 0,
    "memory_state": {
        "vault": "500.00 USD",
        "fee": "10.00 USD",
        "net": "490.00 USD"
    },
    "error_details": null
}

```

🏗️ System Architecture

1. Forgiving Lexer: Automatically handles human accounting formats (e.g., stripping commas from `150,000.00 KES`).
2. Indestructible Parser: Maps Abstract Syntax Trees (AST) and explicitly warns developers of exact syntax failures instead of silently dropping lines.
3. Semantic Auditor: The "Gatekeeper." Enforces `(Name, Type)` tuples on all transactions before memory allocation occurs.
4. State Evaluator: Processes the validated AST and dynamically updates the hash-mapped ledger memory.


Built for scale. Built for security. Built for the future of money.*


