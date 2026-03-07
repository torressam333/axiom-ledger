# Axiom Ledger

Axiom Ledger is a high-performance, crypto-native financial engine built in Rust. It serves as a distributed "Source of Truth" for value exchange, designed with the rigor required for modern digital asset exchanges.

## Development Philosophy: Deep Systems Engineering

This project is not an exercise in shallow prompt engineering. It is a hands-on, rigorous deep-dive into the mechanics of memory safety, asynchronous runtimes, and distributed state.

* **Heavily Iterative Learning:** The codebase is built through a cycle of intentional breaking changes to understand the underlying constraints of the Rust compiler and the borrow checker.
* **Architect-Led AI Collaboration:** AI is utilized as a high-level pedagogical consultant for exploring obscure systems-level concepts (such as vtable layouts, memory alignment, and cache-line contention). However, all implementation, debugging, and architectural decisions are executed manually to ensure a ground-truth understanding of the stack.
* **TDD and Correctness:** Every feature begins with a failing test. The goal is not just "working code," but code that is mathematically sound and resilient to edge cases common in fintech environments.

## Architectural Foundations

1. **Mathematical Integrity:** Utilizing fixed-point arithmetic (modeling "Drops" for XRPL) to eliminate floating-point rounding errors in financial calculations.
2. **Mechanical Sympathy:** Optimized for low-latency by prioritizing stack allocation over heap indirection where possible and leveraging Rust's ownership model to ensure thread safety without over-locking.
3. **Event-Driven Resiliency:** Architecture designed to integrate with message brokers like Kafka to decouple core ledger updates from auxiliary services like fraud detection and analytics.



## Tech Stack & Tooling

* **Language:** Rust (Stable)
* **Runtime:** Tokio (Multi-threaded Asynchronous I/O)
* **Persistence:** SQLx (Compile-time verified SQL for PostgreSQL)
* **Messaging:** Kafka / gRPC (Planned)
* **Observability:** Tracing-subscriber for structured, hierarchical logging.

## Core Domain Features

* **Multi-Currency Support:** Extensible Currency domain model currently supporting XRP (XRPL native).
* **Validation Logic:** Domain-driven design (DDD) ensures that invalid states, such as negative balances, are unrepresentable in the type system.
* **Zero-Copy Mentality:** Strategic use of stack-allocated byte arrays for cryptographic addresses to minimize heap pressure and garbage collection overhead.



## Project Structure

Following the library/binary split for maximum testability:

* `src/lib.rs`: The Brain – Core domain logic, validation rules, and traits.
* `src/main.rs`: The Muscle – Entry point, configuration management, and server initialization.
* `src/domain/`: Pure business logic, isolated from infrastructure concerns.
* `tests/`: Black-box integration tests ensuring the API meets functional requirements.

## Roadmap

- [ ] Implementation of a high-throughput transaction matching engine.
- [ ] Integration with XRPL Testnet for real-time deposit/withdrawal monitoring.
- [ ] Kafka producer integration for outbound financial events.
- [ ] Prometheus/Grafana dashboard for real-time performance metrics.

---

**Author:** [Sam Torres/Axiom Logic LLC]
