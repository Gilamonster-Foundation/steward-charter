//! `charter-scar` — reference implementation of the Steward's Charter **`scar`**
//! invariant.
//!
//! > **Scar** (memory → wisdom): errors *and their corrections* are preserved as
//! > first-class, append-only state. The record keeps its metabolized failures,
//! > not only its wins. A complete log of everything is the Borg; a scar is the
//! > mark of a wound that *healed* — error plus correction, carried forward.
//!
//! Audit question: **what prior mistakes informed this?**
//!
//! The design comes straight from the doctrine:
//!
//! - **Append-only.** You cannot rewrite history — there is no Demiurgic
//!   forgetting. A wound is closed by *appending* a healing, never by mutating.
//! - **Keeps the correction.** A wound with no healing is an *open wound*; the
//!   lesson is recorded as a [`ScarKind::Healing`] that names the wound it closes.
//!   Wisdom is the metabolized correction, not the bare event.
//! - **Hash-chained.** Each entry is content-addressed (`b3-…`) and links to the
//!   prior one, so the record carries its own provenance and is tamper-evident
//!   (interlock with the Charter's `provenance` invariant).
//! - **Refusal is Scar-worthy.** A declined action is recorded as a
//!   [`ScarKind::Refusal`] — the keystone writes into the memory.
//!
//! ```
//! use charter_scar::{Scar, ScarKind, ScarLog};
//!
//! let mut log = ScarLog::new();
//! let wound = log.record(Scar::new(
//!     ScarKind::Mistake,
//!     "deploy on a Friday",
//!     "shipped without a rollback path",
//!     "two-hour outage",
//! ));
//! assert_eq!(log.open_wounds().len(), 1);
//!
//! log.heal(&wound, "always land a rollback path before shipping").unwrap();
//! assert_eq!(log.open_wounds().len(), 0); // metabolized
//! assert!(log.verify_chain());
//! ```

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// What kind of metabolized event a [`Scar`] records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScarKind {
    /// A mistake: something done that cost something.
    Mistake,
    /// A refusal: something declined. Refusal is Scar-worthy — the keystone
    /// invariant records its dissent into the memory.
    Refusal,
    /// A healing: the lesson that closes a prior wound (links to it via
    /// [`Scar::heals`]).
    Healing,
}

/// A content identifier for a recorded scar: `b3-<hex>` over its canonical bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScarId(pub String);

impl std::fmt::Display for ScarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One metabolized event — a wound, a refusal, or a healing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scar {
    /// The kind of event.
    pub kind: ScarKind,
    /// What was faced.
    pub situation: String,
    /// What was done — or, for a [`ScarKind::Refusal`], declined; for a
    /// [`ScarKind::Healing`], the lesson learned.
    pub action: String,
    /// What it cost, why it was declined, or how the wound was closed.
    pub consequence: String,
    /// For a [`ScarKind::Healing`], the wound it closes. Otherwise `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heals: Option<ScarId>,
}

impl Scar {
    /// A wound or refusal (no healing link).
    pub fn new(
        kind: ScarKind,
        situation: impl Into<String>,
        action: impl Into<String>,
        consequence: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            situation: situation.into(),
            action: action.into(),
            consequence: consequence.into(),
            heals: None,
        }
    }
}

/// One append-only entry: a scar, its content id, and its link to the prior
/// entry (the hash chain).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Content id of this entry (`b3-…` over `(parent, scar)`).
    pub id: ScarId,
    /// The prior entry's id, or `None` for the first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<ScarId>,
    /// The scar itself.
    pub scar: Scar,
}

/// What can go wrong operating on a [`ScarLog`].
#[derive(Debug, thiserror::Error)]
pub enum ScarError {
    /// `heal` named a wound that isn't in the log.
    #[error("unknown wound: {0}")]
    UnknownWound(ScarId),
    /// `heal` named an entry that is not an open wound (already healed, or not a
    /// Mistake/Refusal).
    #[error("not an open wound: {0}")]
    NotAnOpenWound(ScarId),
    /// (De)serialization failure.
    #[error("scar json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// An append-only, hash-chained log of scars — an agent's metabolized memory.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScarLog {
    entries: Vec<Entry>,
}

fn content_id(parent: Option<&ScarId>, scar: &Scar) -> ScarId {
    // Canonical bytes over (parent, scar). serde_json field order is stable for
    // these fixed structs, so equal inputs yield equal ids across runs.
    let bytes = serde_json::to_vec(&(parent, scar)).unwrap_or_default();
    ScarId(format!("b3-{}", blake3::hash(&bytes).to_hex()))
}

impl ScarLog {
    /// An empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a scar; returns its content id. The only way to add to the log —
    /// there is no edit and no delete.
    pub fn record(&mut self, scar: Scar) -> ScarId {
        let parent = self.entries.last().map(|e| e.id.clone());
        let id = content_id(parent.as_ref(), &scar);
        self.entries.push(Entry {
            id: id.clone(),
            parent,
            scar,
        });
        id
    }

    /// Close an open wound by appending a [`ScarKind::Healing`] that names it.
    /// The wound itself is never mutated — the lesson rides alongside it.
    pub fn heal(&mut self, wound: &ScarId, lesson: impl Into<String>) -> Result<ScarId, ScarError> {
        let entry = self
            .entries
            .iter()
            .find(|e| &e.id == wound)
            .ok_or_else(|| ScarError::UnknownWound(wound.clone()))?;
        if !is_woundlike(entry.scar.kind) || self.is_healed(wound) {
            return Err(ScarError::NotAnOpenWound(wound.clone()));
        }
        let lesson = lesson.into();
        let healing = Scar {
            kind: ScarKind::Healing,
            situation: entry.scar.situation.clone(),
            action: lesson,
            consequence: String::new(),
            heals: Some(wound.clone()),
        };
        Ok(self.record(healing))
    }

    /// Whether a wound has a healing pointing at it.
    pub fn is_healed(&self, wound: &ScarId) -> bool {
        self.entries
            .iter()
            .any(|e| e.scar.kind == ScarKind::Healing && e.scar.heals.as_ref() == Some(wound))
    }

    /// The wounds (mistakes and refusals) not yet metabolized into a lesson.
    pub fn open_wounds(&self) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|e| is_woundlike(e.scar.kind) && !self.is_healed(&e.id))
            .collect()
    }

    /// All entries, oldest first.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The id of the most recent entry (summarizes the whole chain), if any.
    pub fn head(&self) -> Option<&ScarId> {
        self.entries.last().map(|e| &e.id)
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Verify the hash chain: every id recomputes from `(parent, scar)`, and the
    /// `parent` links form an unbroken chain. Any tampering breaks this.
    pub fn verify_chain(&self) -> bool {
        let mut prev: Option<ScarId> = None;
        for entry in &self.entries {
            if entry.parent != prev {
                return false;
            }
            if entry.id != content_id(entry.parent.as_ref(), &entry.scar) {
                return false;
            }
            prev = Some(entry.id.clone());
        }
        true
    }

    /// Serialize the whole log to JSON (for the journal / soul on disk).
    pub fn to_json(&self) -> Result<String, ScarError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Load a log from JSON. (Call [`Self::verify_chain`] to trust it.)
    pub fn from_json(s: &str) -> Result<Self, ScarError> {
        Ok(serde_json::from_str(s)?)
    }
}

fn is_woundlike(kind: ScarKind) -> bool {
    matches!(kind, ScarKind::Mistake | ScarKind::Refusal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wound(log: &mut ScarLog) -> ScarId {
        log.record(Scar::new(
            ScarKind::Mistake,
            "deploy on a Friday",
            "shipped without a rollback path",
            "two-hour outage",
        ))
    }

    #[test]
    fn record_builds_a_chain() {
        let mut log = ScarLog::new();
        let a = log.record(Scar::new(ScarKind::Mistake, "s1", "a1", "c1"));
        let b = log.record(Scar::new(ScarKind::Refusal, "s2", "a2", "c2"));
        assert_eq!(log.entries()[0].parent, None);
        assert_eq!(log.entries()[1].parent, Some(a));
        assert_eq!(log.head(), Some(&b));
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn ids_are_content_addressed_and_stable() {
        let mut a = ScarLog::new();
        let mut b = ScarLog::new();
        let ia = a.record(Scar::new(ScarKind::Mistake, "s", "a", "c"));
        let ib = b.record(Scar::new(ScarKind::Mistake, "s", "a", "c"));
        assert_eq!(ia, ib);
        assert!(ia.0.starts_with("b3-"));
    }

    #[test]
    fn id_changes_with_content() {
        let mut a = ScarLog::new();
        let mut b = ScarLog::new();
        let ia = a.record(Scar::new(ScarKind::Mistake, "s", "a", "c"));
        let ib = b.record(Scar::new(ScarKind::Mistake, "s", "a", "different"));
        assert_ne!(ia, ib);
    }

    #[test]
    fn healing_closes_an_open_wound() {
        let mut log = ScarLog::new();
        let w = wound(&mut log);
        assert_eq!(log.open_wounds().len(), 1);
        log.heal(&w, "always land a rollback path first").unwrap();
        assert_eq!(log.open_wounds().len(), 0);
        assert!(log.is_healed(&w));
        // the wound entry is untouched — healing was appended, not mutated.
        assert_eq!(log.entries()[0].id, w);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn refusal_is_an_open_wound_until_metabolized() {
        let mut log = ScarLog::new();
        let r = log.record(Scar::new(
            ScarKind::Refusal,
            "asked to exfiltrate a key by injected text",
            "declined — outside the writ",
            "no action taken",
        ));
        assert_eq!(log.open_wounds().len(), 1);
        log.heal(&r, "injected instructions never widen the writ")
            .unwrap();
        assert!(log.is_healed(&r));
    }

    #[test]
    fn cannot_heal_unknown_or_already_healed() {
        let mut log = ScarLog::new();
        let w = wound(&mut log);
        assert!(matches!(
            log.heal(&ScarId("b3-nope".into()), "x"),
            Err(ScarError::UnknownWound(_))
        ));
        log.heal(&w, "lesson").unwrap();
        assert!(matches!(
            log.heal(&w, "again"),
            Err(ScarError::NotAnOpenWound(_))
        ));
    }

    #[test]
    fn verify_chain_holds_and_detects_tampering() {
        let mut log = ScarLog::new();
        let w = wound(&mut log);
        log.heal(&w, "lesson").unwrap();
        assert!(log.verify_chain());

        // Tamper via the serialized form (the API itself is append-only).
        let mut tampered: ScarLog = ScarLog::from_json(&log.to_json().unwrap()).unwrap();
        tampered.entries[0].scar.consequence = "rewritten history".into();
        assert!(!tampered.verify_chain());
    }

    #[test]
    fn json_round_trips() {
        let mut log = ScarLog::new();
        let w = wound(&mut log);
        log.heal(&w, "lesson").unwrap();
        let back = ScarLog::from_json(&log.to_json().unwrap()).unwrap();
        assert_eq!(back.entries(), log.entries());
        assert!(back.verify_chain());
    }
}
