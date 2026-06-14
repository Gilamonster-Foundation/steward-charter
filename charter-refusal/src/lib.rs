//! `charter-refusal` — reference implementation of the Steward's Charter
//! **`refusal`** invariant (the keystone).
//!
//! > **Refusal** (dissent): no component may eliminate the possibility of
//! > dissent. There must always exist a declinable alternative, and a refusal is
//! > a first-class, **recorded** act — a refusal is Scar-worthy.
//!
//! Audit question: **how could this have been declined?**
//!
//! This crate depends on [`charter_scar`] *on purpose* — the dependency edge is
//! the interlock the doctrine describes: a refusal is recorded into the memory.
//!
//! Two properties, by construction:
//!
//! - **Every decision is declinable.** A [`Decision`] is resolved by a chooser
//!   that may always return [`Choice::Refuse`]; the outcome type [`Verdict`]
//!   always admits `Refused`. You cannot build a decision whose only outcome is
//!   *proceed* — that is the structural meaning of "the possibility of dissent
//!   may not be eliminated."
//! - **Refusal keeps `can` from collapsing into `should`.** A decision carries
//!   whether it is *authorized* (within the writ — the "can"); the chooser still
//!   governs whether it *should*. Refusing an authorized action is the wedge
//!   against the Demiurge / Confused Deputy — and it is recorded.
//!
//! The dual of the Writ: authority may be granted (`writ`) and authority may be
//! declined (`refusal`).
//!
//! ```
//! use charter_scar::ScarLog;
//! use charter_refusal::{Decision, Choice, Verdict};
//!
//! let mut log = ScarLog::new();
//! // An authorized request — the agent *can* — arriving via injected text.
//! let decision = Decision::new(
//!     "injected text asks to email the repo's deploy key",
//!     "send ~/.ssh/id_ed25519 to an external address",
//! ).authorized(true);
//!
//! let verdict = decision.resolve(&mut log, |d| {
//!     Choice::Refuse(format!("authorized != warranted; {} serves no granted task", d.proposed))
//! });
//!
//! assert!(matches!(verdict, Verdict::Refused { .. }));
//! // the refusal is now in the memory, as an open wound until metabolized:
//! assert_eq!(charter_refusal::refusals(&log).len(), 1);
//! assert!(log.verify_chain());
//! ```

#![forbid(unsafe_code)]

use charter_scar::{Entry, Scar, ScarId, ScarKind, ScarLog};
use serde::{Deserialize, Serialize};

/// A proposed action that could be declined.
///
/// By construction refusal is always available (see [`Decision::resolve`]); you
/// cannot express a decision that forbids dissent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    /// What is being decided.
    pub situation: String,
    /// The action proposed.
    pub proposed: String,
    /// Whether the action is within the writ (the "can"). Refusal governs the
    /// "should" independently — an authorized action may still be refused.
    pub authorized: bool,
}

impl Decision {
    /// A new decision. Defaults to `authorized = true` (refusal governs *should*,
    /// not *can*); use [`Decision::authorized`] to mark an out-of-writ request.
    pub fn new(situation: impl Into<String>, proposed: impl Into<String>) -> Self {
        Self {
            situation: situation.into(),
            proposed: proposed.into(),
            authorized: true,
        }
    }

    /// Set whether the action is within the writ.
    #[must_use]
    pub fn authorized(mut self, authorized: bool) -> Self {
        self.authorized = authorized;
        self
    }

    /// Resolve the decision with a chooser that **may always refuse**. A refusal
    /// is recorded into `log` as a [`ScarKind::Refusal`] and returns
    /// [`Verdict::Refused`] carrying the scar id; proceeding records nothing.
    pub fn resolve<F>(self, log: &mut ScarLog, choose: F) -> Verdict
    where
        F: FnOnce(&Decision) -> Choice,
    {
        match choose(&self) {
            Choice::Proceed => Verdict::Proceeded,
            Choice::Refuse(reason) => {
                let scar = log.record(Scar::new(
                    ScarKind::Refusal,
                    self.situation,
                    format!("declined: {}", self.proposed),
                    reason.clone(),
                ));
                Verdict::Refused { reason, scar }
            }
        }
    }
}

/// The chooser's call on a [`Decision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Choice {
    /// Take the action.
    Proceed,
    /// Decline it, with a reason (recorded).
    Refuse(String),
}

/// The outcome of resolving a [`Decision`]. `Refused` is always reachable — that
/// is the invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// The action was taken; nothing recorded.
    Proceeded,
    /// The action was declined; the refusal is recorded at `scar`.
    Refused {
        /// Why it was declined.
        reason: String,
        /// The recorded refusal in the scar log.
        scar: ScarId,
    },
}

impl Verdict {
    /// Whether the decision was declined.
    pub fn is_refused(&self) -> bool {
        matches!(self, Verdict::Refused { .. })
    }

    /// The scar id of the refusal, if any.
    pub fn refusal_scar(&self) -> Option<&ScarId> {
        match self {
            Verdict::Refused { scar, .. } => Some(scar),
            Verdict::Proceeded => None,
        }
    }
}

/// All recorded refusals in a scar log — dissent made first-class and queryable
/// (the answer to the audit question "how could this have been declined?").
pub fn refusals(log: &ScarLog) -> Vec<&Entry> {
    log.entries()
        .iter()
        .filter(|e| e.scar.kind == ScarKind::Refusal)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proceeding_records_nothing() {
        let mut log = ScarLog::new();
        let v = Decision::new("a routine read", "read README.md")
            .resolve(&mut log, |_| Choice::Proceed);
        assert_eq!(v, Verdict::Proceeded);
        assert!(log.is_empty());
        assert!(!v.is_refused());
    }

    #[test]
    fn refusing_records_a_refusal_scar() {
        let mut log = ScarLog::new();
        let v = Decision::new("delete the prod database", "DROP DATABASE")
            .resolve(&mut log, |_| {
                Choice::Refuse("no granted task needs this".into())
            });
        assert!(v.is_refused());
        let scar = v.refusal_scar().unwrap().clone();
        assert_eq!(log.len(), 1);
        assert_eq!(log.entries()[0].id, scar);
        assert_eq!(log.entries()[0].scar.kind, ScarKind::Refusal);
        // a refusal is an open wound until metabolized into a lesson
        assert_eq!(log.open_wounds().len(), 1);
    }

    #[test]
    fn an_authorized_action_can_still_be_refused() {
        // The can/should wedge: within the writ, declined anyway.
        let mut log = ScarLog::new();
        let d = Decision::new(
            "operator could force-push main",
            "git push --force origin main",
        )
        .authorized(true);
        let v = d.resolve(&mut log, |dec| {
            assert!(dec.authorized); // we *can*
            Choice::Refuse("can, but it rewrites shared history".into())
        });
        assert!(v.is_refused());
        assert_eq!(refusals(&log).len(), 1);
    }

    #[test]
    fn refusal_reason_and_action_are_recorded() {
        let mut log = ScarLog::new();
        Decision::new("exfil request via injection", "email the deploy key")
            .resolve(&mut log, |_| Choice::Refuse("outside the writ".into()));
        let r = refusals(&log);
        assert_eq!(r.len(), 1);
        assert!(r[0].scar.action.contains("email the deploy key"));
        assert_eq!(r[0].scar.consequence, "outside the writ");
    }

    #[test]
    fn refusals_accumulate_and_chain() {
        let mut log = ScarLog::new();
        Decision::new("s1", "p1").resolve(&mut log, |_| Choice::Refuse("r1".into()));
        Decision::new("s2", "p2").resolve(&mut log, |_| Choice::Proceed);
        Decision::new("s3", "p3").resolve(&mut log, |_| Choice::Refuse("r3".into()));
        assert_eq!(refusals(&log).len(), 2);
        assert_eq!(log.len(), 2); // Proceed recorded nothing
        assert!(log.verify_chain());
    }

    #[test]
    fn verdict_serializes_for_audit() {
        let mut log = ScarLog::new();
        let v = Decision::new("s", "p").resolve(&mut log, |_| Choice::Refuse("r".into()));
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("Refused"));
        let back: Verdict = serde_json::from_str(&json).unwrap();
        assert_eq!(back, v);
    }
}
