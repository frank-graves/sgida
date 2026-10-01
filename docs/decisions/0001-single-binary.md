# 0001 Single Binary Architecture

## Status
Accepted

## Context
The system must orchestrate isolated browsing environments for privacy research.
We need to manage identities, namespaces, and state without external daemon overhead.

## Decision
The entire system is compiled into a single binary (`sgidad`).
All modules communicate via in-process traits defined in `sgida-ports`.
No RPC, no IPC, no subprocesses for the modules themselves.

## Consequences
- No Node.js, Go, or C FFI allowed.
- Playwright is rejected in favor of `chromiumoxide` (Rust-native CDP).
- Frida is rejected in favor of `rquickjs` (embedded JS).
- `utls` is rejected in favor of `boring` (Apache-2.0 TLS).
- Simplifies deployment and eliminates network overhead between components.