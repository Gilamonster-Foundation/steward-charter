//! `charter-register` — the line's doctrine registers, made resolvable.
//!
//! [`AUTHORITY.md`] and [`CRAFT.md`] are prose, and prose is what people read.
//! But a citation from another repository (`per AUTH-02`) has to *resolve*, and
//! a citation written two years ago has to resolve to the same law it named
//! then. That is a machine-checkable property, so this crate checks it rather
//! than asking the registers to be careful.
//!
//! Two things live here:
//!
//! - **Lookup.** [`authority`] and [`craft`] parse their register at compile
//!   time from the embedded markdown, so a consumer can turn `AUTH-02` into its
//!   name and one-line obligation without shelling out to a document.
//! - **The proof obligation.** `CRAFT-05` (law minimalism) says no law without
//!   a proof obligation. These registers now make a promise — *IDs are stable,
//!   permanent, and never reused* — and the tests below are that promise's
//!   proof: every summary-table row has a matching anchored section, the IDs
//!   are dense and ordered, and the declared law count in the version header
//!   matches reality. Renumbering a law, dropping a section, or adding a row
//!   without an anchor fails `cargo test`.
//!
//! The registers stay the source of truth. This crate never restates a law; it
//! reads them, which is `AUTH-05`'s own discipline applied to itself.
//!
//! [`AUTHORITY.md`]: https://github.com/Gilamonster-Foundation/steward-charter/blob/main/docs/AUTHORITY.md
//! [`CRAFT.md`]: https://github.com/Gilamonster-Foundation/steward-charter/blob/main/docs/CRAFT.md

#![forbid(unsafe_code)]

/// The raw text of the Authority Register.
pub const AUTHORITY_MD: &str = include_str!("../../docs/AUTHORITY.md");

/// The raw text of the Craft Register.
pub const CRAFT_MD: &str = include_str!("../../docs/CRAFT.md");

/// One law, as the register states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Law {
    /// The stable identifier, e.g. `AUTH-02`. Permanent and never reused.
    pub id: String,
    /// The law's name, e.g. "Attenuate, never amplify".
    pub name: String,
    /// The one-line obligation from the summary table.
    pub one_line: String,
    /// The Charter invariants this law serves, e.g. `["writ"]`. Empty when the
    /// register says the law is pure engineering with no invariant behind it.
    pub serves: Vec<String>,
}

/// A parsed register: its version, and its laws in ID order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    /// The declared version, e.g. `"1.0"`.
    pub version: String,
    /// The laws, ordered by ID.
    pub laws: Vec<Law>,
}

impl Register {
    /// The law with this ID, if the register has one. Case-insensitive.
    #[must_use]
    pub fn law(&self, id: &str) -> Option<&Law> {
        self.laws.iter().find(|l| l.id.eq_ignore_ascii_case(id))
    }
}

/// The Authority Register — how power is granted, bounded, and proven.
///
/// # Panics
///
/// Panics if the embedded markdown cannot be parsed. That is deliberate: a
/// malformed register is a build-time defect in this repository, not a runtime
/// condition a caller can handle.
#[must_use]
pub fn authority() -> Register {
    parse(AUTHORITY_MD, "AUTH")
}

/// The Craft Register — how the invariants get built.
///
/// # Panics
///
/// See [`authority`].
#[must_use]
pub fn craft() -> Register {
    parse(CRAFT_MD, "CRAFT")
}

/// Strip the markdown inline-code backticks a register uses for IDs and
/// invariant names, leaving the bare token.
fn untick(s: &str) -> String {
    s.trim().trim_matches('`').trim().to_string()
}

fn parse(md: &str, prefix: &str) -> Register {
    let version = md
        .lines()
        .find_map(|l| l.strip_prefix("**Version "))
        .and_then(|rest| rest.split("**").next())
        .unwrap_or_else(|| panic!("{prefix}: no `**Version N.N**` header"))
        .trim()
        .to_string();

    let laws: Vec<Law> = md
        .lines()
        .filter(|l| l.starts_with(&format!("| `{prefix}-")))
        .map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split('|').collect();
            assert!(
                cells.len() >= 4,
                "{prefix}: table row has {} cells, expected 4: {line}",
                cells.len()
            );
            Law {
                id: untick(cells[0]),
                name: cells[1].trim().to_string(),
                one_line: cells[2].trim().to_string(),
                serves: cells[3]
                    .split('·')
                    .map(untick)
                    .filter(|s| !s.is_empty())
                    .collect(),
            }
        })
        .collect();

    assert!(!laws.is_empty(), "{prefix}: no laws found in the table");
    Register { version, laws }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every ID in the summary table has an anchored section in the prose.
    ///
    /// This is the load-bearing one. A citation `per AUTH-02` is a link to
    /// `AUTHORITY.md#AUTH-02`; if the anchor is missing the citation resolves
    /// to the top of the document and quietly means nothing.
    fn every_id_is_anchored(md: &str, reg: &Register) {
        for law in &reg.laws {
            let anchor = format!("<a id=\"{}\"></a>", law.id);
            assert!(
                md.contains(&anchor),
                "{} is in the table but has no anchor — a citation to it cannot resolve",
                law.id
            );
        }
    }

    /// IDs are dense and ordered: `PREFIX-01`, `-02`, … with no gaps.
    ///
    /// A gap means a law was deleted rather than retired in place, which breaks
    /// every citation that named it. The registers say a retired law keeps its
    /// ID and its section; this is that rule, enforced.
    fn ids_are_dense_and_ordered(reg: &Register, prefix: &str) {
        for (i, law) in reg.laws.iter().enumerate() {
            let expected = format!("{prefix}-{:02}", i + 1);
            assert_eq!(
                law.id, expected,
                "IDs must be dense and ordered; found {} where {expected} was expected. \
                 A retired law keeps its ID and section rather than being deleted.",
                law.id
            );
        }
    }

    /// The version header's declared law count matches the table.
    fn header_count_matches(md: &str, reg: &Register) {
        let declared = md
            .lines()
            .find(|l| l.starts_with("**Version "))
            .expect("version header");
        let n = reg.laws.len();
        assert!(
            declared.contains(&format!("{n} laws")),
            "header says {declared:?} but the table has {n} laws"
        );
    }

    /// No two laws share an ID, and no two share a name.
    fn ids_and_names_are_unique(reg: &Register) {
        for (i, a) in reg.laws.iter().enumerate() {
            for b in &reg.laws[i + 1..] {
                assert_ne!(a.id, b.id, "duplicate ID {}", a.id);
                assert_ne!(
                    a.name, b.name,
                    "two laws named {:?} — a name-based citation is ambiguous",
                    a.name
                );
            }
        }
    }

    #[test]
    fn authority_register_is_well_formed() {
        let reg = authority();
        every_id_is_anchored(AUTHORITY_MD, &reg);
        ids_are_dense_and_ordered(&reg, "AUTH");
        header_count_matches(AUTHORITY_MD, &reg);
        ids_and_names_are_unique(&reg);
    }

    #[test]
    fn craft_register_is_well_formed() {
        let reg = craft();
        every_id_is_anchored(CRAFT_MD, &reg);
        ids_are_dense_and_ordered(&reg, "CRAFT");
        header_count_matches(CRAFT_MD, &reg);
        ids_and_names_are_unique(&reg);
    }

    /// The specific laws this repository's own tooling and sibling repos cite
    /// by ID today. Pinned so a renumbering cannot silently redirect them.
    #[test]
    fn cited_ids_still_name_what_they_named() {
        let auth = authority();
        assert_eq!(auth.law("AUTH-01").unwrap().name, "Fail closed");
        assert_eq!(
            auth.law("AUTH-02").unwrap().name,
            "Attenuate, never amplify"
        );
        assert_eq!(
            auth.law("AUTH-03").unwrap().name,
            "Amplification needs the human root"
        );

        let craft = craft();
        assert_eq!(craft.law("CRAFT-05").unwrap().name, "Law minimalism");
    }

    #[test]
    fn lookup_is_case_insensitive_and_misses_cleanly() {
        let auth = authority();
        assert!(auth.law("auth-02").is_some());
        assert!(auth.law("AUTH-99").is_none());
    }

    #[test]
    fn serves_parses_the_invariant_list() {
        let auth = authority();
        // `AUTH-03` serves two invariants, dot-separated in the table.
        assert_eq!(
            auth.law("AUTH-03").unwrap().serves,
            vec!["tether".to_string(), "writ".to_string()]
        );
    }

    #[test]
    fn versions_are_declared() {
        assert_eq!(authority().version, "1.0");
        assert_eq!(craft().version, "1.1");
    }
}
