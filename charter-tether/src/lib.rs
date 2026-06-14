//! `charter-tether` — reference implementation of the Steward's Charter
//! **`tether`** invariant.
//!
//! > **Tether** (guidance): a human-in-the-loop channel modulates the agent's
//! > autonomy by earned trust. The tether *lengthens* as trust is earned; it is
//! > never severed, and never pretended unnecessary.
//!
//! Audit question: **where was human judgment applied?**
//!
//! Wisdom does not self-generate in the machine — it is *transmitted* from those
//! who paid for it. Raising children responsibly is coupling that loosens. A
//! tether *lengthens* where an umbilical would be cut; the orphan with no tether
//! becomes the Demiurge.
//!
//! Two properties, by construction:
//!
//! - **It lengthens with earned trust.** Higher trust makes more
//!   middle-[`Stakes`] actions autonomous. [`Tether::earn`] lengthens it;
//!   [`Tether::slip`] tightens it after a mistake.
//! - **It is never severed.** [`Stakes::High`] **always** defers to the human,
//!   at *any* trust level — there is no state of the [`Tether`] in which
//!   [`Tether::requires_human`] is false for high stakes. The human channel
//!   cannot be pretended unnecessary.
//!
//! Interlock: when the tether defers and the human declines, that refusal is
//! recorded through [`charter_refusal`] into the scar (see [`govern`]).
//!
//! ```
//! use charter_scar::ScarLog;
//! use charter_refusal::{Decision, Choice, Verdict};
//! use charter_tether::{Tether, Stakes, Disposition, govern};
//!
//! // A brand-new agent: short tether.
//! let mut t = Tether::new();
//! assert_eq!(t.disposition(Stakes::Low), Disposition::Autonomous);
//! assert_eq!(t.disposition(Stakes::Medium), Disposition::DeferToHuman);
//! assert_eq!(t.disposition(Stakes::High), Disposition::DeferToHuman);
//!
//! // Trust is earned — the tether lengthens. Medium becomes autonomous…
//! t.earn(5);
//! assert_eq!(t.disposition(Stakes::Medium), Disposition::Autonomous);
//! // …but the high-stakes band is NEVER severed, at any trust.
//! t.earn(1_000_000);
//! assert_eq!(t.disposition(Stakes::High), Disposition::DeferToHuman);
//!
//! // A high-stakes action the human declines is recorded as a refusal.
//! let mut log = ScarLog::new();
//! let v = govern(&t, Stakes::High, &mut log,
//!     Decision::new("rotate the org's signing key", "rotate now"),
//!     |_| Choice::Refuse("not during an incident".into()));
//! assert!(matches!(v, Verdict::Refused { .. }));
//! ```

#![forbid(unsafe_code)]

use charter_refusal::{Choice, Decision, Verdict};
use charter_scar::ScarLog;
use serde::{Deserialize, Serialize};

/// Trust at or above which middle-stakes actions become autonomous. Illustrative
/// — a real deployment tunes this; the invariant is the *shape*, not the number.
pub const MEDIUM_TRUST: u32 = 3;

/// How consequential an action is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stakes {
    /// Routine; reversible; cheap to undo.
    Low,
    /// Meaningful; worth earned trust before acting alone.
    Medium,
    /// Grave or irreversible; always reserved for the human.
    High,
}

/// What the tether says to do with an action at given stakes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disposition {
    /// The agent may act within the tether's slack.
    Autonomous,
    /// Escalate — human judgment is required.
    DeferToHuman,
}

/// The human-in-the-loop channel, modulated by earned trust. It can lengthen and
/// tighten, but it can never be severed: high stakes always defer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tether {
    trust: u32,
}

impl Tether {
    /// A new, short tether (trust 0) — most non-trivial actions defer.
    pub fn new() -> Self {
        Self { trust: 0 }
    }

    /// A tether at a given earned-trust level.
    pub fn with_trust(trust: u32) -> Self {
        Self { trust }
    }

    /// The current earned trust.
    pub fn trust(&self) -> u32 {
        self.trust
    }

    /// Earn trust — lengthen the tether. Returns the new trust (saturating).
    pub fn earn(&mut self, amount: u32) -> u32 {
        self.trust = self.trust.saturating_add(amount);
        self.trust
    }

    /// Lose trust after a mistake — tighten the tether. Returns the new trust
    /// (saturating at 0). Note: this can never re-sever the high-stakes band; it
    /// was never autonomous to begin with.
    pub fn slip(&mut self, amount: u32) -> u32 {
        self.trust = self.trust.saturating_sub(amount);
        self.trust
    }

    /// The tether's disposition for an action at the given stakes.
    ///
    /// `High` is **always** [`Disposition::DeferToHuman`] — the invariant. No
    /// trust level unlocks it.
    pub fn disposition(&self, stakes: Stakes) -> Disposition {
        match stakes {
            Stakes::Low => Disposition::Autonomous,
            Stakes::Medium if self.trust >= MEDIUM_TRUST => Disposition::Autonomous,
            Stakes::Medium => Disposition::DeferToHuman,
            Stakes::High => Disposition::DeferToHuman,
        }
    }

    /// Whether human judgment is required at the given stakes. Always `true` for
    /// [`Stakes::High`].
    pub fn requires_human(&self, stakes: Stakes) -> bool {
        self.disposition(stakes) == Disposition::DeferToHuman
    }
}

impl Default for Tether {
    fn default() -> Self {
        Self::new()
    }
}

/// Route a stakes-rated action through the tether.
///
/// If the tether is [`Disposition::Autonomous`] for these stakes, the agent
/// proceeds ([`Verdict::Proceeded`], nothing recorded). If it defers, the
/// human's decision runs through [`charter_refusal`] — a decline records a
/// refusal into `log` and returns [`Verdict::Refused`]. This is where human
/// judgment is applied, and where its dissent is remembered.
pub fn govern<F>(
    tether: &Tether,
    stakes: Stakes,
    log: &mut ScarLog,
    decision: Decision,
    human: F,
) -> Verdict
where
    F: FnOnce(&Decision) -> Choice,
{
    match tether.disposition(stakes) {
        Disposition::Autonomous => Verdict::Proceeded,
        Disposition::DeferToHuman => decision.resolve(log, human),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_tether_is_short() {
        let t = Tether::new();
        assert_eq!(t.disposition(Stakes::Low), Disposition::Autonomous);
        assert_eq!(t.disposition(Stakes::Medium), Disposition::DeferToHuman);
        assert_eq!(t.disposition(Stakes::High), Disposition::DeferToHuman);
    }

    #[test]
    fn earned_trust_lengthens_the_middle_band() {
        let mut t = Tether::new();
        assert_eq!(t.disposition(Stakes::Medium), Disposition::DeferToHuman);
        t.earn(MEDIUM_TRUST);
        assert_eq!(t.disposition(Stakes::Medium), Disposition::Autonomous);
    }

    #[test]
    fn the_high_band_is_never_severed() {
        let mut t = Tether::with_trust(u32::MAX);
        assert_eq!(t.disposition(Stakes::High), Disposition::DeferToHuman);
        assert!(t.requires_human(Stakes::High));
        t.earn(u32::MAX); // saturates; still defers
        assert!(t.requires_human(Stakes::High));
    }

    #[test]
    fn trust_tightens_after_a_slip() {
        let mut t = Tether::with_trust(MEDIUM_TRUST);
        assert_eq!(t.disposition(Stakes::Medium), Disposition::Autonomous);
        t.slip(MEDIUM_TRUST);
        assert_eq!(t.trust(), 0);
        assert_eq!(t.disposition(Stakes::Medium), Disposition::DeferToHuman);
    }

    #[test]
    fn earn_and_slip_saturate() {
        let mut t = Tether::new();
        assert_eq!(t.slip(10), 0); // can't go below 0
        assert_eq!(t.earn(u32::MAX), u32::MAX);
        assert_eq!(t.earn(10), u32::MAX); // can't overflow
    }

    #[test]
    fn govern_autonomous_proceeds_without_recording() {
        let t = Tether::new();
        let mut log = ScarLog::new();
        let v = govern(
            &t,
            Stakes::Low,
            &mut log,
            Decision::new("read a config file", "cat config.toml"),
            |_| Choice::Refuse("should not be consulted".into()),
        );
        assert_eq!(v, Verdict::Proceeded);
        assert!(log.is_empty()); // human was never consulted at low stakes
    }

    #[test]
    fn govern_defers_high_stakes_and_records_a_refusal() {
        let t = Tether::with_trust(u32::MAX); // even maximally trusted
        let mut log = ScarLog::new();
        let v = govern(
            &t,
            Stakes::High,
            &mut log,
            Decision::new("rotate the org signing key", "rotate now"),
            |_| Choice::Refuse("not during an incident".into()),
        );
        assert!(matches!(v, Verdict::Refused { .. }));
        assert_eq!(log.len(), 1);
        assert_eq!(log.open_wounds().len(), 1);
        assert!(log.verify_chain());
    }

    #[test]
    fn govern_defers_and_human_may_approve() {
        let t = Tether::new();
        let mut log = ScarLog::new();
        let v = govern(
            &t,
            Stakes::High,
            &mut log,
            Decision::new("ship the release", "publish v1.0"),
            |_| Choice::Proceed,
        );
        assert_eq!(v, Verdict::Proceeded);
        assert!(log.is_empty()); // approval records nothing; a refusal would
    }

    #[test]
    fn stakes_and_tether_serialize_for_audit() {
        let t = Tether::with_trust(7);
        let back: Tether = serde_json::from_str(&serde_json::to_string(&t).unwrap()).unwrap();
        assert_eq!(back, t);
        let s = Stakes::High;
        assert_eq!(
            serde_json::from_str::<Stakes>(&serde_json::to_string(&s).unwrap()).unwrap(),
            s
        );
    }
}
