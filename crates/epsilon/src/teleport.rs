//! ═══════════════════════════════════════════════════════════════════════════════
//! Epsilon Teleportation System Call
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Orchestrates the full Zero-Shot Context Teleportation pipeline:
//!
//! ```text
//!   1. Acquire SurgeryPermit from governor (β → 0)
//!   2. Inject payload into target hollow manifold
//!   3. Trigger wake-up rescan (assimilate)
//!   4. Restore governor via complete_surgery()
//!   5. Return result
//! ```
//!
//! # Proposed Capabilities (Section 5)
//!
//! - **Telepathic Swarm Architecture**: Agent A processes a 1M-token
//!   codebase, converges Seal-Loop, teleports Betti-stable manifold to
//!   Agents B, C, D. They possess the knowledge instantly at zero
//!   inference cost.
//!
//! - **O(1) RAG (Retrieval-Augmented Geometry)**: Instead of retrieving
//!   text chunks and re-embedding, the database stores pre-compiled
//!   topological shapes and injects them directly into the agent's
//!   hollow core.
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::governor::SurgeryGovernor;
use crate::manifold::{HollowCubeManifold, ManifoldPayload, SurgeryError};

// ═══════════════════════════════════════════════════════════════════════════════
// Teleport Target Specification
// ═══════════════════════════════════════════════════════════════════════════════

/// Target specification for context teleportation.
#[derive(Debug, Clone)]
pub enum TeleportTarget {
    /// Inject into the local agent's own hollow manifold
    LocalVoid,
    /// Inject into a remote agent (identified by agent_id)
    RemoteVoid { agent_id: u64 },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Teleport Result
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of a teleportation operation.
#[derive(Debug, Clone, PartialEq)]
pub enum TeleportResult {
    /// Teleportation succeeded
    Success {
        /// Number of manifold points assimilated into the target
        points_assimilated: usize,
    },
    /// Target void is already occupied
    VoidOccupied,
    /// Payload topology doesn't match target constraints
    TopologyMismatch { expected_b0: u32, actual_b0: u32 },
    /// Surgery permit was denied (governor in unstable state)
    PermitDenied,
    /// Target shell is degenerate (β₀ ≠ 1)
    DegenerateTarget { shell_b0: u32 },
    /// Payload is empty
    EmptyPayload,
}

impl From<SurgeryError> for TeleportResult {
    fn from(err: SurgeryError) -> Self {
        match err {
            SurgeryError::VoidOccupied => TeleportResult::VoidOccupied,
            SurgeryError::TopologyMismatch { expected_b0, actual_b0 } =>
                TeleportResult::TopologyMismatch { expected_b0, actual_b0 },
            SurgeryError::EmptyPayload => TeleportResult::EmptyPayload,
            SurgeryError::DegenerateShell { shell_b0 } =>
                TeleportResult::DegenerateTarget { shell_b0 },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// sys_teleport_context — The System Call
// ═══════════════════════════════════════════════════════════════════════════════

/// System call: teleport a manifold payload into a hollow cube manifold.
///
/// Orchestrates the full teleportation pipeline with formal safety
/// guarantees:
///
/// 1. **Governor Clutch** (Section 3.1): Acquires `SurgeryPermit`,
///    zeroing the derivative gain β to prevent oscillation panic.
///
/// 2. **Void Injection** (Section 2.2): Injects the payload into the
///    target manifold's interior void, verifying topological constraints.
///
/// 3. **Wake-Up Rescan** (Section 3.3): Triggers assimilation, merging
///    payload points into the shell's active graph.
///
/// 4. **Governor Restore**: Returns the permit, re-enabling the PD
///    controller's derivative gain.
///
/// # Arguments
/// * `target_manifold` - The hollow cube receiving the teleported context
/// * `payload` - The pre-computed manifold payload from the source agent
/// * `governor` - The surgery governor managing threshold adaptation
///
/// # Returns
/// [`TeleportResult`] indicating success (with assimilation count) or
/// the specific failure mode.
///
/// # Example
/// ```text
/// // Agent A: Process 1M tokens → converge Seal-Loop → build payload
/// let payload = ManifoldPayload::from_graph(&converged_graph, 5.0);
///
/// // Agent B: Receive context at O(1) cost
/// let result = sys_teleport_context(&mut agent_b_manifold, payload, &mut gov);
/// assert!(matches!(result, TeleportResult::Success { .. }));
/// ```
pub fn sys_teleport_context<const D: usize>(
    target_manifold: &mut HollowCubeManifold<D>,
    payload: ManifoldPayload<D>,
    governor: &mut SurgeryGovernor,
) -> TeleportResult {
    // ─── Step 1: Acquire Surgery Permit ───────────────────────────────
    // Zero the derivative gain β to absorb the instantaneous state jump
    let permit = governor.prepare_for_surgery();

    // ─── Step 2: Inject Payload into Void ─────────────────────────────
    // Verify topological constraints and write into the β₂ interior
    if let Err(surgery_err) = target_manifold.inject_into_void(payload) {
        // Injection failed — restore governor before returning
        governor.complete_surgery(permit);
        return TeleportResult::from(surgery_err);
    }

    // ─── Step 3: Wake-Up Rescan ───────────────────────────────────────
    // Verify Betti boundaries and merge into active processing shell
    let points_assimilated = target_manifold.assimilate();

    // ─── Step 4: Restore Governor ─────────────────────────────────────
    // Re-enable derivative gain for normal operation
    governor.complete_surgery(permit);

    TeleportResult::Success { points_assimilated }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unit Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifold::{EpsilonPoint, SparseGraph};

    /// Helper: build a connected shell with 3 points
    fn build_shell<const D: usize>(epsilon: f64, pts: &[[f64; D]]) -> HollowCubeManifold<D> {
        let mut hollow = HollowCubeManifold::new(epsilon);
        for p in pts {
            hollow.add_shell_point(EpsilonPoint::new(*p));
        }
        hollow
    }

    /// Helper: build a connected payload from points
    fn build_payload<const D: usize>(
        epsilon: f64,
        pts: &[[f64; D]],
        liveness: f64,
    ) -> ManifoldPayload<D> {
        let mut graph = SparseGraph::new(epsilon);
        for p in pts {
            graph.add_point(EpsilonPoint::new(*p));
        }
        ManifoldPayload::from_graph(&graph, liveness)
    }

    #[test]
    fn test_full_teleportation_pipeline() {
        let mut manifold = build_shell(1.0, &[
            [0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0],
            [0.5, 0.5, 0.0],
        ]);
        let payload = build_payload(1.0, &[
            [1.0, 1.0, 1.0],
            [1.5, 1.0, 1.0],
        ], 5.0);
        let mut gov = SurgeryGovernor::new();

        let result = sys_teleport_context(&mut manifold, payload, &mut gov);

        assert_eq!(result, TeleportResult::Success { points_assimilated: 2 });
        assert!(manifold.void_is_empty());
        // Governor should be fully restored
        assert!((gov.beta() - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_teleportation_rejects_disconnected() {
        let mut manifold = build_shell(1.0, &[
            [0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0],
        ]);

        // Disconnected payload (β₀ = 2)
        let payload = build_payload(0.1, &[
            [0.0, 0.0, 0.0],
            [100.0, 100.0, 100.0],
        ], 3.0);
        let mut gov = SurgeryGovernor::new();

        let result = sys_teleport_context(&mut manifold, payload, &mut gov);

        assert_eq!(result, TeleportResult::TopologyMismatch {
            expected_b0: 1, actual_b0: 2
        });
        // Governor MUST still be restored even on failure
        assert!((gov.beta() - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_teleportation_void_occupied() {
        let mut manifold = build_shell(2.0, &[
            [0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0],
            [0.5, 0.5, 0.0],
        ]);
        let mut gov = SurgeryGovernor::new();

        // First teleportation — but inject without assimilating
        let payload1 = build_payload(2.0, &[
            [1.0, 1.0, 1.0],
            [1.5, 1.0, 1.0],
        ], 5.0);
        manifold.inject_into_void(payload1).unwrap();

        // Second attempt — should fail (void occupied)
        let payload2 = build_payload(2.0, &[
            [2.0, 2.0, 2.0],
        ], 3.0);
        let result = sys_teleport_context(&mut manifold, payload2, &mut gov);

        assert_eq!(result, TeleportResult::VoidOccupied);
    }
}
