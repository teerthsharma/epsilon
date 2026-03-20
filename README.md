# Epsilon

**Zero-Shot Context Transfer via Topological Surgery on Hollow Manifolds**

*Pre-Print Reference Implementation v0.1.0*

> **Author:** Teerth Sharma

---

## Overview

Epsilon is a research framework for instantaneous geometric state injection between autonomous agents. It structures the agent's cognitive state space as a **hollow S² manifold** (Betti number β₂ = 1), creating an interior void that accepts pre-computed, topologically stable payloads in **O(1)** time — independent of sequence length.

This eliminates the O(N²) attention bottleneck during context loading by operating directly on the manifold geometry rather than the token stream.

## Architecture

```
crates/
├── aether-core/          # Mathematical foundation
│   ├── manifold.rs       # ManifoldPoint, SparseAttentionGraph
│   ├── topology.rs       # Betti numbers, persistent homology
│   ├── governor.rs       # GeometricGovernor (PD control)
│   ├── memory.rs         # ManifoldHeap, Chebyshev evictor
│   └── ...               
│
└── epsilon/              # Geometric state transfer (this research)
    ├── manifold.rs       # HollowCubeManifold, ManifoldPayload
    ├── governor.rs       # SurgeryGovernor, SurgeryPermit
    ├── memory.rs         # LivenessAnchor, ChebyshevGuard
    └── teleport.rs       # sys_teleport_context (orchestration)
```

## Key Ideas

### Hollow Manifold (β₂ = 1)

| | Solid | Hollow |
|---|---|---|
| β₀ (Components) | 1 | 1 |
| β₁ (Cycles) | 0 | 0 |
| β₂ (Voids) | 0 | **1** |

The void defined by β₂ = 1 is a receptacle. A converged sub-manifold from one agent maps directly into this void — the geometry carries the meaning.

### Topological Surgery

```
f: D ⊂ M_source → Void(M_target)
```

Injection is guarded by Betti signature verification. The payload must be topologically consistent (β₀ = 1, connected) and the receiver shell must be non-degenerate.

### Safety

1. **Surgery Permit** — Derivative gain β is zeroed for one tick to absorb the instantaneous state discontinuity without oscillation panic.
2. **Chebyshev Liveness Inheritance** — Injected data inherits statistical bounds (μ, σ, k) from the source, preventing premature eviction by the receiver's garbage collector.
3. **Assimilation Rescan** — Betti boundaries are verified before merging payload points into the active shell.

## Building

```bash
cargo check -p epsilon
cargo test -p epsilon
cargo test --workspace
```

## Complexity

| Operation | Cost |
|-----------|------|
| Payload construction | O(P) |
| Betti verification | O(V+E) |
| Governor clutch | O(1) |
| Injection + assimilation | O(P) |
| **Total pipeline** | **O(P)** |

Where P ≤ 64 payload points. Compare to O(N²) for N-token attention.

## License

MIT — Teerth Sharma
