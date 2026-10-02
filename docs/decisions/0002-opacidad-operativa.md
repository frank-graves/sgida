# 0002 — Operational opacity

Status: Accepted

## Context

SGIDA manages ephemeral identities and isolated environments. The orchestrator
must coordinate lifecycle, crash recovery, reconciliation, and scheduling, while
still enforcing privacy by design. If handles exposed their underlying resources,
every consumer would become a potential leakage path for sensitive material.

Handles therefore need two properties that can appear contradictory:

1. They must be opaque to consumers so that no implementation detail, secret,
   credential, or resource content can be read through the handle.
2. They must be serializable so that the orchestrator can persist them, restore
   them after a crash, and reconcile runtime state against durable state.

## Decision

Operational opacity is adopted as a design principle.

Opacity protects against data leakage, not against operational requirements.
Handles are opaque to consumers, but they remain serializable identifiers.
They can be stored in `StateSnapshot`, restored at boot, and used for
reconciliation. They never expose constructors, accessors, or conversion APIs
that reveal the underlying resource beyond the identifier required for
coordination.

The guiding rule is:

"Opaco al consumidor no significa no serializable. La opacidad protege contra
filtraciones de datos sensibles; no protege contra requisitos operativos como
recovery, auditoría o reconciliación. Cuando ambos chocan, la opacidad se
redefine — no se abandona, se reorienta."

## Consequences

- `ProfileHandle` and `EnvironmentHandle` derive `Serialize` and `Deserialize`.
- The orchestrator persists handles inside `StateSnapshot`.
- Handles expose no accessor that leaks resource content.
- Future handles must preserve this rule: serialization for recovery is
  allowed; introspection into the underlying resource is not.
