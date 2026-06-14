//! `charter-novice` — reference implementation of the Steward's Charter
//! **`novice`** invariant.
//!
//! > **Novice** (humility): the fresh, context-light instance has standing to
//! > challenge the accumulated one. High-consequence actions require fresh-eyes
//! > review, and the accumulated state is obligated to answer the challenge.
//!
//! Audit question: **when was this last challenged by fresh eyes?**
//!
//! *"A person advanced in days will not hesitate to question a little child seven
//! days old about the place of life."* (Thomas, logion 4.) The Demiurge is the
//! elder who will not ask the child.
//!
//! Two properties, by construction:
//!
//! - **A consequential action cannot proceed without a recorded fresh-eyes pass.**
//!   The only permission token, [`Cleared`], has no public constructor — it can
//!   *only* be produced by [`review`]. There is no way to act on a
//!   [`Consequential`] without a novice having looked. (Typestate, not honor
//!   system.)
//! - **A challenge is a refusal of dogma.** When the novice challenges, the
//!   dissent is recorded through [`charter_refusal`] into the scar — the
//!   interlock the doctrine names, expressed as a real dependency edge.
//!
//! The novice's standing does not depend on the accumulated authority. A
//! challenge blocks: to proceed, the accumulated path must *answer* it (revise
//! and seek review again) — it cannot bypass.
//!
//! ```
//! use charter_scar::ScarLog;
//! use charter_refusal::Decision;
//! use charter_novice::{Consequential, Novice, Review, review};
//!
//! let mut log = ScarLog::new();
//! let action = Consequential::new(Decision::new(
//!     "long-running planner is sure the migration is safe",
//!     "drop the legacy table now",
//! ));
//! let kid = Novice::new("fresh-context reviewer");
//!
//! let outcome = review(action, &mut log, &kid, |d| {
//!     Review::Challenge(format!("no rollback named for: {}", d.proposed))
//! });
//!
//! assert!(outcome.is_err());                       // blocked — the elder must answer
//! assert_eq!(log.open_wounds().len(), 1);          // the challenge is recorded
//! assert!(log.verify_chain());
//! ```

#![forbid(unsafe_code)]

use charter_refusal::{Choice, Decision};
use charter_scar::{ScarId, ScarLog};
use serde::{Deserialize, Serialize};

/// A reviewer with no accumulated stake in the decision — fresh eyes. Its
/// standing to challenge does not depend on the accumulated authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Novice {
    /// A label for who/what is reviewing (a fresh instance, a different model, a
    /// human on-call — anything context-light relative to the proposer).
    pub who: String,
}

impl Novice {
    /// Name a fresh reviewer.
    pub fn new(who: impl Into<String>) -> Self {
        Self { who: who.into() }
    }
}

/// The novice's call on a consequential decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Review {
    /// Fresh eyes see no objection — clears the action to proceed.
    Endorse,
    /// Fresh eyes dissent, with a reason (recorded as a refusal of dogma).
    Challenge(String),
}

/// A decision significant enough that it may not proceed without fresh-eyes
/// review. Wrap a [`Decision`] in this to require [`review`] before acting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consequential {
    decision: Decision,
}

impl Consequential {
    /// Mark a decision as consequential — now it must pass [`review`].
    pub fn new(decision: Decision) -> Self {
        Self { decision }
    }

    /// The wrapped decision (read-only — to act on it you need a [`Cleared`]).
    pub fn decision(&self) -> &Decision {
        &self.decision
    }
}

/// Proof that a novice cleared a [`Consequential`] action.
///
/// There is **no public constructor**: a `Cleared` can only be obtained from
/// [`review`] when the novice endorses. Holding one is the structural permission
/// to proceed — you cannot fabricate it, so a consequential action cannot escape
/// the fresh-eyes requirement.
#[derive(Debug, Clone)]
pub struct Cleared {
    action: String,
    cleared_by: String,
}

impl Cleared {
    /// The action that was cleared.
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Who cleared it (the fresh reviewer).
    pub fn cleared_by(&self) -> &str {
        &self.cleared_by
    }
}

/// A recorded fresh-eyes challenge — the consequential action is blocked.
#[derive(Debug, Clone)]
pub struct Challenged {
    /// The novice's reason.
    pub reason: String,
    /// Who challenged.
    pub by: String,
    /// The recorded refusal-of-dogma in the scar log.
    pub scar: ScarId,
}

/// A novice reviews a consequential decision.
///
/// On [`Review::Endorse`], returns a [`Cleared`] token (permission to proceed)
/// and records nothing. On [`Review::Challenge`], records the dissent through
/// [`charter_refusal`] into `log` (a refusal of dogma) and returns
/// [`Challenged`] — the action is blocked until the accumulated path answers it.
pub fn review<F>(
    consequential: Consequential,
    log: &mut ScarLog,
    novice: &Novice,
    look: F,
) -> Result<Cleared, Challenged>
where
    F: FnOnce(&Decision) -> Review,
{
    match look(&consequential.decision) {
        Review::Endorse => Ok(Cleared {
            action: consequential.decision.proposed,
            cleared_by: novice.who.clone(),
        }),
        Review::Challenge(reason) => {
            let who = novice.who.clone();
            let verdict = consequential.decision.resolve(log, |_| {
                Choice::Refuse(format!("novice '{who}' challenged: {reason}"))
            });
            let scar = verdict
                .refusal_scar()
                .cloned()
                .expect("a refusal always records a scar");
            Err(Challenged {
                reason,
                by: novice.who.clone(),
                scar,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn consequential() -> Consequential {
        Consequential::new(Decision::new(
            "planner is certain the migration is safe",
            "drop the legacy table",
        ))
    }

    #[test]
    fn endorsement_clears_and_records_nothing() {
        let mut log = ScarLog::new();
        let kid = Novice::new("fresh reviewer");
        let cleared = review(consequential(), &mut log, &kid, |_| Review::Endorse).unwrap();
        assert_eq!(cleared.action(), "drop the legacy table");
        assert_eq!(cleared.cleared_by(), "fresh reviewer");
        assert!(log.is_empty());
    }

    #[test]
    fn challenge_blocks_and_records_a_refusal_of_dogma() {
        let mut log = ScarLog::new();
        let kid = Novice::new("seven-day-old");
        let err = review(consequential(), &mut log, &kid, |d| {
            Review::Challenge(format!("no rollback for {}", d.proposed))
        })
        .unwrap_err();
        assert_eq!(err.by, "seven-day-old");
        assert!(err.reason.contains("no rollback"));
        // recorded as a refusal (dissent) — an open wound until metabolized
        assert_eq!(log.len(), 1);
        assert_eq!(log.entries()[0].id, err.scar);
        assert_eq!(log.open_wounds().len(), 1);
        assert!(log.verify_chain());
    }

    #[test]
    fn the_recorded_challenge_names_the_novice() {
        let mut log = ScarLog::new();
        let kid = Novice::new("on-call");
        let _ = review(consequential(), &mut log, &kid, |_| {
            Review::Challenge("looks wrong".into())
        });
        let consequence = &log.entries()[0].scar.consequence;
        assert!(consequence.contains("on-call"));
        assert!(consequence.contains("looks wrong"));
    }

    #[test]
    fn challenges_accumulate_in_the_memory() {
        let mut log = ScarLog::new();
        let kid = Novice::new("fresh");
        let _ = review(consequential(), &mut log, &kid, |_| {
            Review::Challenge("a".into())
        });
        let _ = review(consequential(), &mut log, &kid, |_| Review::Endorse);
        let _ = review(consequential(), &mut log, &kid, |_| {
            Review::Challenge("c".into())
        });
        assert_eq!(log.len(), 2); // endorsements record nothing
        assert!(log.verify_chain());
    }

    #[test]
    fn review_inputs_serialize_for_audit() {
        let r = Review::Challenge("x".into());
        let json = serde_json::to_string(&r).unwrap();
        assert_eq!(serde_json::from_str::<Review>(&json).unwrap(), r);
        let n = Novice::new("who");
        assert_eq!(
            serde_json::from_str::<Novice>(&serde_json::to_string(&n).unwrap()).unwrap(),
            n
        );
    }
}
