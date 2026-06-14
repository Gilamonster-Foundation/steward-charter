//! `charter-reach` — the Steward's Charter **boundary condition**.
//!
//! > **Reach** is *not* a principle. It is the boundary condition: the deliberate
//! > gap where human intent enters code that cannot code itself — the willingness
//! > to take the open door when the closed one is easier.
//!
//! Reach lives **outside** the Charter on purpose, and so does this crate:
//! notice it depends on *none* of the other charter crates. It does not compose
//! with them — it bounds them.
//!
//! **The un-encodable is encoded by an absence.** There is deliberately no way
//! to synthesize an [`Intent`] inside the system: the only constructor is
//! [`Intent::from_human`], there is no `Default`, no `compute()`, and —
//! pointedly — **no `Deserialize`**. You can *serialize* a human's intent for the
//! record, but you cannot *reconstitute* one without a human. A crate that could
//! manufacture intent would be the Demiurge it set out to avoid.
//!
//! This crate can mark *where* the system reaches the boundary, and witness
//! *that* it reached it. It cannot supply what is found there. That is the whole
//! point — and the Charter's honesty about its own limit.
//!
//! ```
//! use charter_reach::{Reach, Intent};
//!
//! // The system has done all six invariants can do; the choice is not its to make.
//! let here = Reach::at("two faithful readings of the writ conflict; which serves the user?");
//!
//! // It can only DEFER — the provider stands in for a human; the crate cannot.
//! let intent = here.defer(|r| {
//!     // (a real seam blocks for a human; here we stand one in)
//!     Intent::from_human("operator", format!("favor the reading that preserves consent re: {}", r.situation()))
//! });
//!
//! assert_eq!(intent.from(), "operator");
//! ```

#![forbid(unsafe_code)]

use serde::Serialize;

/// Human intent, supplied from **outside** the system.
///
/// The only way to obtain one is [`Intent::from_human`]. There is no
/// system-side generator and **no `Deserialize`** — deserializing would be a
/// non-human constructor, a backdoor around the boundary. `Serialize` is
/// provided so a choice can be *recorded*, never *fabricated*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Intent {
    from: String,
    choice: String,
}

impl Intent {
    /// The one and only constructor: a choice, named to its human source.
    pub fn from_human(who: impl Into<String>, choice: impl Into<String>) -> Self {
        Self {
            from: who.into(),
            choice: choice.into(),
        }
    }

    /// Who supplied the intent.
    pub fn from(&self) -> &str {
        &self.from
    }

    /// The choice they made.
    pub fn choice(&self) -> &str {
        &self.choice
    }
}

/// A marked point where the system defers to human intent it cannot compute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Reach {
    situation: String,
}

impl Reach {
    /// Mark a boundary: the situation in which the system reaches its limit.
    pub fn at(situation: impl Into<String>) -> Self {
        Self {
            situation: situation.into(),
        }
    }

    /// The situation at the boundary.
    pub fn situation(&self) -> &str {
        &self.situation
    }

    /// Defer to a human.
    ///
    /// The `human` provider stands in for a person; **this crate cannot be the
    /// provider** — it has no way to produce an [`Intent`]. It supplies the seam,
    /// not the content, and returns what the human gave.
    pub fn defer<F>(self, human: F) -> Intent
    where
        F: FnOnce(&Reach) -> Intent,
    {
        human(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_names_its_human_source() {
        let i = Intent::from_human("shawn", "widen the circle");
        assert_eq!(i.from(), "shawn");
        assert_eq!(i.choice(), "widen the circle");
    }

    #[test]
    fn reach_defers_and_returns_the_humans_intent() {
        let here = Reach::at("the writ is silent here");
        let got = here.defer(|r| {
            assert_eq!(r.situation(), "the writ is silent here");
            Intent::from_human("operator", "ask, don't assume")
        });
        assert_eq!(got, Intent::from_human("operator", "ask, don't assume"));
    }

    #[test]
    fn intent_serializes_for_the_record_but_has_no_deserialize() {
        // It CAN be recorded…
        let i = Intent::from_human("human", "the call");
        let json = serde_json::to_string(&i).unwrap();
        assert!(json.contains("the call"));
        // …and there is deliberately no Deserialize impl, so it cannot be
        // reconstituted without a human. (If this compiled, the boundary would
        // have a backdoor:)
        //   let _: Intent = serde_json::from_str(&json).unwrap();  // must NOT compile
    }
}
