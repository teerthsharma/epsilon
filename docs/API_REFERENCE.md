# Epsilon API Reference

---

## Core Types

### `EpsilonPoint<const D: usize>` — manifold.rs

Point in D-dimensional space. Public `coords: [f64; D]`.

```rust
let p = EpsilonPoint::<3>::new([1.0, 2.0, 3.0]);
p.distance(&other)         // Euclidean
p.is_neighbor(&other, 0.5) // ε-test
```

### `SparseGraph<const D: usize>` — manifold.rs

Sparse attention graph. Public `points`, `point_count`.

```rust
let mut g = SparseGraph::<3>::new(1.0);
g.add_point(EpsilonPoint::new([0.0, 0.0, 0.0]));
g.shape() // (β₀, β₁)
```

### `ManifoldPayload<const D: usize>` — manifold.rs

Geometric payload with Betti signature and liveness anchor.

```rust
let payload = ManifoldPayload::from_graph(&graph, 5.0);
// payload.signature_b0, payload.liveness_anchor
```

### `HollowCubeManifold<const D: usize>` — manifold.rs

S² shell with β₂=1 void.

```rust
let mut m = HollowCubeManifold::<3>::new(1.0);
m.add_shell_point(point);
m.inject_into_void(payload)?; // topological surgery
m.assimilate();               // merge into shell
```

### `SurgeryError` — manifold.rs

```rust
VoidOccupied | TopologyMismatch { expected_b0, actual_b0 }
EmptyPayload | DegenerateShell { shell_b0 }
```

---

## Governor

### `SurgeryGovernor` — governor.rs

PD controller: `ε(t+1) = ε(t) + α·e(t) + β·de/dt`

```rust
let mut gov = SurgeryGovernor::new();
gov.adapt(deviation, dt);
let permit = gov.prepare_for_surgery(); // β → 0
gov.complete_surgery(permit);           // β → saved
```

### `SurgeryPermit` — governor.rs

One-shot. Non-Clone. Non-Copy. Consumed on restore.

---

## Memory

### `LivenessAnchor` — memory.rs

```rust
let anchor = LivenessAnchor::from_samples(&data, 2.0);
// anchor.anchor_liveness = μ + σ
```

### `ChebyshevGuard` — memory.rs

```rust
let guard = ChebyshevGuard::with_inherited_anchor(&anchor);
guard.is_safe(liveness) // true if above μ - k·σ
```

---

## Orchestration

### `sys_teleport_context()` — teleport.rs

```rust
let result = sys_teleport_context(&mut manifold, payload, &mut gov);
// TeleportResult::Success { points_assimilated }
```

Full pipeline: clutch → inject → assimilate → restore.
