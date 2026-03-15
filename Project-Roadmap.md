# 🗺️ Axiom Ledger: Engineering Roadmap

## Phase 1: Domain Hardening & Type Safety
*The Goal: Build a mathematically "impossible to break" core using Type-Driven Design.*
- [x] **Core Domain Types:** Define `Wallet`, `Address`, and `Balance`.
- [x] **Infallible Math:** Implement underflow protection for withdrawals.
- [x] **Module Architecture:** Establish professional file hierarchy (`mod.rs` patterns).
- [ ] **Fixed-Point Conversion:** Manual parsing of XRP strings to "Drops" (Avoid IEEE 754 float errors).
- [ ] **The Newtype Pattern:** Wrap primitive types to prevent "Primitive Obsession" bugs.
- [ ] **Trait System:** Implement `Display` (formatting), `Add`/`Sub` (operator overloading), and `FromStr`.
- [ ] **Validation Layer:** Use the `validator` crate or custom logic for XRPL-specific address checksums.

## Phase 2: Persistence & The Repository Pattern
*The Goal: Durable, ACID-compliant storage that is decoupled from the business logic.*
- [ ] **Schema Design:** Define `wallets` and `ledger_entries` with strict constraints (Check constraints for non-negative balances).
- [ ] **SQLx Integration:** Setup `sqlx` with PostgreSQL and compile-time query verification.
- [ ] **The Repository Trait:** Define a `WalletRepository` trait to allow for easy mocking in tests.
- [ ] **Atomic Transactions:** Implement a "Transfer" function that spans multiple SQL operations safely.
- [ ] **Database Migrations:** Manage schema versions using `sqlx-cli`.

## Phase 3: The GraphQL Supergraph Layer
*The Goal: A high-performance, contract-first API using `async-graphql`.*
- [ ] **Schema Definition:** Design Queries (`wallet`), Mutations (`deposit`, `withdraw`, `transfer`).
- [ ] **Custom Scalars:** Create a `Balance` scalar in GraphQL to match your Rust type.
- [ ] **Dataloaders:** Implement batching for wallet lookups to solve the $N+1$ problem.
- [ ] **Complex Validation:** Integrate domain errors into GraphQL's error format.
- [ ] **Apollo Federation:** Annotate types (e.g., `@key`) to ensure the ledger can work in a Supergraph.

## Phase 4: Systems Depth & Concurrency
*The Goal: Optimize for high throughput and handle race conditions.*
- [ ] **Tokio Runtime:** Tuning task spawning and understanding the multi-threaded scheduler.
- [ ] **Concurrency Control:** Implement `Arc<RwLock<T>>` or `Atomics` for in-memory caching.
- [ ] **Race Condition Testing:** Use `loom` or heavy concurrent tests to verify no "double-spend" is possible.
- [ ] **Performance Profiling:** Use `Criterion` for micro-benchmarks and `flamegraph` to find CPU bottlenecks.
- [ ] **Memory Layout:** Analyze struct packing and alignment to minimize memory footprint.

## Phase 5: Operations & Production Readiness
*The Goal: Observability, security, and world-class CI/CD.*
- [ ] **Structured Logging:** Implement `tracing` with spans for request lifecycle tracking.
- [ ] **Metrics & Telemetry:** Export Prometheus metrics (e.g., `transaction_count`, `ledger_latency`).
- [ ] **Security Audit:** Implement `cargo-deny` for dependency security and `clippy` (strict) for code quality.
- [ ] **Containerization:** Multistage Docker builds optimized for size and security (Distroless/Alpine).
- [ ] **Integration Testing:** Write a "Black Box" test suite that runs against a real Postgres container.
