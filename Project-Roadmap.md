# 🗺️ Axiom Ledger: Engineering Roadmap

## Phase 1: Domain Hardening & Type Safety
*The Goal: Build a mathematically "impossible to break" core using Type-Driven Design.*
- [x] **Core Domain Types:** Define `Wallet`, `Address`, and `Balance`.
- [x] **Infallible Math:** Implement underflow protection for withdrawals.
- [x] **Module Architecture:** Establish professional file hierarchy (`mod.rs` patterns).
- [x] **Fixed-Point Conversion:** Manual parsing of XRP strings to "Drops" (Avoid IEEE 754 float errors).
- [x] **The Newtype Pattern:** Wrap primitive types (e.g., `pub struct DropCount(u128)`) to prevent logic errors.
- [x] **Trait System:** Implement `Display` (formatting), `Add`/`Sub` (operator overloading), and `FromStr`.
- [x] **Validation Layer:** Use the `validator` crate for XRPL-specific address checksums.

## Phase 2: Persistence & The Repository Pattern
*The Goal: Durable, ACID-compliant storage decoupled from the business logic.*
- [x] **Local-First Infrastructure:** Setup a `docker-compose.yml` for PostgreSQL (Free/Local).
- [x] **Schema Design:** Define `wallets` and `ledger_entries` with strict SQL constraints.
- [x] **SQLx Integration:** Setup `sqlx` with compile-time query verification.
- [x] **The Repository Trait:** Define a `WalletRepository` trait to allow for easy mocking.
- [x] **Atomic Transactions:** Implement a "Transfer" function using SQL transactions.
- [ ] **Multi-Asset Safety:** Implement currency validation in the Domain and Service layers to prevent illegal cross-asset transfers.

## Phase 3: The GraphQL Supergraph Layer
*The Goal: A high-performance, contract-first API using `async-graphql`.*
- [ ] **Schema Definition:** Design Queries (`wallet`), Mutations (`deposit`, `withdraw`, `transfer`).
- [ ] **Custom Scalars:** Create a `Balance` scalar in GraphQL to match your Rust type.
- [ ] **Dataloaders:** Implement batching for wallet lookups to solve the N+1 problem.
- [ ] **Apollo Federation:** Annotate types (e.g., `@key`) to ensure Supergraph compatibility.

## Phase 4: Event-Driven Architecture (EDA) & Kafka
*The Goal: High-reliability distributed systems logic as requested by Kraken.*
- [ ] **Local Message Broker:** Add **Redpanda** (Kafka-compatible) to your Docker Compose.
- [ ] **The Outbox Pattern:** Ensure ledger updates and event emissions happen in one atomic DB transaction.
- [ ] **Event Schema:** Use `serde` or `prost` (Protobuf) to define "TransferInitiated" and "BalanceUpdated" events.
- [ ] **Background Worker:** Build a "Relayer" in Rust that reads the Outbox table and pushes to Kafka.
- [ ] **Idempotency:** Ensure that processing the same event twice doesn't result in double-spending.

## Phase 5: Systems Depth & High-Frequency Trading (HFT)
*The Goal: Optimize for high throughput and handle race conditions.*
- [ ] **Lock-Free Concurrency:** Use `Atomics` or `dashmap` to handle "Hot Wallet" contention.
- [ ] **Merkle Tree Implementation:** Build a "Proof of Reserves" module for cryptographic auditability.
- [ ] **Performance Profiling:** Use `Criterion` for benchmarks and `flamegraph` for CPU analysis.
- [ ] **Zero-Copy Serialization:** Experiment with `rkyv` or `flatbuffers` for ultra-fast message passing.

## Phase 6: Operations & Production Readiness
*The Goal: Observability, security, and world-class telemetry.*
- [ ] **Structured Logging:** Implement `tracing` with spans for request-id tracking across EDA.
- [ ] **Metrics & Telemetry:** Export Prometheus metrics for ledger balance and event lag.
- [ ] **Chaos Testing:** Write tests that "kill" the Kafka broker mid-transaction to prove resilience.
- [ ] **Security Audit:** Implement `cargo-deny` for dependency security.
