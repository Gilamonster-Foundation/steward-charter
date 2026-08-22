# The Craft Register — how the invariants get built

**Version 1.1** · 17 laws, `CRAFT-01`–`CRAFT-17` · frozen 2026-08-21

> The engineering doctrine of the line, in the same spirit as the Charter but a
> different register. The Charter says **what the system must guarantee** to
> stay non-Demiurgic. This says **how the code that implements those guarantees
> is built** so it stays legible, testable, and refusable.

The six invariants (`writ`, `scar`, `refusal`, `novice`, `tether`,
`provenance`) are safety. The laws below are *craft* — how the tree that
realizes them is structured and how change lands in it. They were homeless:
scattered across per-repo agent instructions, working notes, and memory. Here
they are one citable, versioned home, owned by no product and cited by all —
the same posture as the Charter itself.

Some craft laws are the **mechanical face of an invariant**: a regression test
*is* a `scar`; a blocking quality gate *is* a `refusal`; a self-describing
identifier *is* `provenance`. Others are pure buildability with no safety
invariant behind them, and the table says so honestly rather than forcing a
link.

| ID | Law | One line | Serves |
|---|---|---|---|
| `CRAFT-01` | Loosely coupled, functionally cohesive | group by function, narrow seams | `novice` |
| `CRAFT-02` | The three Cs | Composition · Configuration · Convention | `novice` |
| `CRAFT-03` | Composition roots are manifests | the root wires, it does not implement | `novice` · `writ` |
| `CRAFT-04` | Freeze minimally, ratchet forward | lock as little as possible | `refusal` · `provenance` |
| `CRAFT-05` | Law minimalism | no law without a proof obligation | `refusal` |
| `CRAFT-06` | Name properties, pin at the edge | laws name properties; identifiers self-describe | `provenance` |
| `CRAFT-07` | TDD + the regression rule | a fix without a failing-first test is incomplete | `scar` |
| `CRAFT-08` | Coverage, reported and near-total | ~100% on the production view, every PR | `novice` |
| `CRAFT-09` | Tests separated from source | source is production-only | `novice` |
| `CRAFT-10` | Least lines of code | delete surface rather than test it | `writ` · `refusal` |
| `CRAFT-11` | Keep files small | split by cohesion past a soft cap | `novice` |
| `CRAFT-12` | One issue, one PR, merge on green | small reviewable increments | `scar` · `novice` |
| `CRAFT-13` | Zero warnings | the gate blocks on any warning | `refusal` |
| `CRAFT-14` | Hooks mirror pipelines | local pre-flight = the authoritative gate | `tether` · `refusal` |
| `CRAFT-15` | Identity is derived, not assigned | the address is computed from the bytes | `provenance` |
| `CRAFT-16` | History is tamper-evident and invertible | edits leave forensic evidence and carry an inverse | `provenance` · `scar` |
| `CRAFT-17` | Evidence nobody reads is decoration | the verifier runs in production, or the chain is theatre | `refusal` · `provenance` |

---

## I. Structure — how the parts relate

<a id="CRAFT-01"></a>
### Loosely coupled, functionally cohesive
**Law.** Modules group by *function*, not by category; seams are narrow; a
single logical change lands in a single cohesive place.
**Why.** Coupling is the tax you pay on every future change. Cohesion by
function (not "all the models here, all the services there") means the thing
that changes together lives together.
**Serves `novice`.** A fresh, context-light instance can read one seam and
understand it without swallowing the whole tree — the precondition for the
fresh-eyes challenge to be real rather than ornamental.

<a id="CRAFT-02"></a>
### The three Cs — Composition, Configuration, Convention
**Law.** Prefer composing small pieces over growing large ones; expose
variation as configuration at the seam; lean on convention over ceremony.
**Why.** The three give you flexibility without a framework: behavior is
assembled, tuned, and defaulted rather than hard-wired.

<a id="CRAFT-03"></a>
### Composition roots are manifests, not members
**Law.** The file that wires a module tree together — a Rust `lib.rs` / `mod.rs`,
a Python `__init__.py`, a JS/TS `index.ts` barrel, a Go package root — contains
**only composition**: declarations, the public re-export facade, root
attributes, at most one curated prelude. **No definitions, no logic.** Growth
delegates *down* into sub-manifests: adding the Nth member (a plugin, a command,
a data-structure) touches a sub-manifest and a new file, **never the root**, and
the root facade stays the minimal, stable spine — members are reached at their
path, not flattened up.
**Why.** The root is the one file everyone imports, so it is the path of least
resistance for accretion: a stray definition, one more re-export per feature, a
sprawling doc. Left unchecked it becomes the God object with no single reason to
change. The one-line test: *does adding a member change the root?* If yes, nest
a sub-manifest.
**Serves `novice`** (a root readable at a glance) **and `writ`** (the smallest
exposed surface is the least authority handed out — minimal-surface is
least-privilege applied to the API).

---

## II. Freeze and law — what to lock, what to prove

<a id="CRAFT-04"></a>
### Freeze minimally; ratchet forward
**Law.** Lock down as little as possible per step. Even "current" pins are
provisional and rotate via a forward ratchet, not a permanent freeze.
**Why.** Every freeze forecloses futures. A smaller freeze is cheaper to carry
and cheaper to correct.
**Serves `refusal`** (fewer frozen commitments keep more choices declinable
later) **and `provenance`** (a self-describing artifact needs no frozen
algorithm — the identifier already names its own scheme).

<a id="CRAFT-05"></a>
### Law minimalism
**Law.** Admit only the laws the system actually needs. Nothing enters the law
layer without a **proof obligation**; the algebra tells you what to cut.
**Why.** A law you cannot discharge is decoration, and decoration in the law
layer is where certainty ossifies.
**Serves `refusal`.** A law that carries no proof obligation is dogma that
cannot be challenged — exactly the thing the Novice must be able to question.

<a id="CRAFT-06"></a>
### Name properties, pin implementations at the edge
**Law.** Laws name *properties* (collision-resistance, determinism, ordering);
*profiles* pin the algorithm; *identifiers self-describe* (a multihash, not a
bare "sha256" assumption).
**Why.** Binding a law to one algorithm dies the day the algorithm does. Naming
the property survives the rotation.
**Serves `provenance`.** An identifier that carries its own scheme resolves to
its origin without an out-of-band assumption.

## III. Craft — how code earns trust

<a id="CRAFT-07"></a>
### TDD, and every fix carries a regression test
**Law.** Red before green. A bug fix is incomplete without a test that *would
have failed before the fix* and passes after — verified against the old code
path, not assumed.
**Why.** The test is the durable record that the failure happened and was
metabolized, not merely patched.
**Serves `scar`.** A regression test *is* a scar in code: error plus correction,
kept as first-class, permanent state so the same wound cannot silently reopen.

<a id="CRAFT-08"></a>
### Coverage, reported and near-total
**Law.** Every change reports its coverage; new modules target ~100% line and
function coverage, measured on the **production view** (test files excluded so
the number is honest). Residue that is pure unused-monomorphization accounting
may be waived — but named, not hidden.
**Serves `novice`.** Coverage is the fresh-eyes defect-catch metric made
mechanical: an uncovered line is a claim no one has challenged.

<a id="CRAFT-09"></a>
### Tests separated from source
**Law.** Source files carry production code only; test suites live beside them,
not inside.
**Why.** It keeps the coverage measurement honest (no test scaffolding inflating
the source numbers) and the source legible.
**Serves `novice`.**

<a id="CRAFT-10"></a>
### Least lines of code
**Law.** Delete unused surface rather than test it. No speculative API, no
defensive assertion a law test already pins. Re-admit additively (a
`#[non_exhaustive]` enum, a later export) the day a real consumer appears.
**Serves `writ` and `refusal`.** Less surface is less to audit and less
authority to leak; the smallest thing that does the job is the most declinable.

<a id="CRAFT-11"></a>
### Keep files small
**Law.** New source files stay under a soft cap (~2,500 lines); past it, split
by cohesion.
**Why.** A file no one can hold in their head is a file no fresh instance can
challenge.
**Serves `novice`.**

---

## IV. Flow — how change lands

<a id="CRAFT-12"></a>
### One issue, one PR; merge on green
**Law.** One logical change per branch; small, reviewable increments; each merge
a checkpoint; no long-lived branches. Draft status is a hard stop — never merged
past.
**Serves `scar`** (small increments localize failure so it can be metabolized)
**and `novice`** (a diff a fresh reviewer can actually hold).

<a id="CRAFT-13"></a>
### Zero warnings
**Law.** The gate blocks on any warning — lints denied, formatting checked.
**Why.** Warnings accumulate into noise, and noise is where real defects hide.
**Serves `refusal`.** A quality gate that can decline is a refusal made
mechanical.

<a id="CRAFT-14"></a>
### Hooks mirror pipelines
**Law.** The local pre-flight (push hook, `just check`) runs the *same* checks
as the authoritative CI gate, and the two are kept in parity by rule — editing
one triggers an audit of the others.
**Serves `tether`** (a fast local pre-flight before the authoritative gate)
**and `refusal`** (the gate itself).

---

## V. Provenance — how data earns trust

<a id="CRAFT-15"></a>

### Identity is derived, not assigned
**Law.** A thing's name is **computed from the thing**. Where an artifact,
message, record, or span needs identity, that identity is a content address —
`content-addressable`'s `ContentId` / `RawContentId` where the code can reach
it, a self-describing multihash CID everywhere else. Sequence numbers, UUIDs,
paths, and timestamps are *locators*; they may accompany an identity but they
may never **be** one.
**Why.** An assigned name is a claim by an authority, and it is only as good as
that authority's memory and honesty. A derived name is a claim anyone can check
against the bytes in front of them, with no trusted third party and no
out-of-band agreement. Hand someone the bytes and the address and they can
verify the pairing themselves — the proof travels *with* the data.
**Serves `provenance`.** This is the invariant's mechanical face: an artifact
that carries its own address needs no ledger to prove what it is.
**Discharge.** Recompute the address from the bytes and compare. If a type
cannot state how its address is derived, it does not have an identity yet.
**Two failure modes this rules out.** *Bare digests* — a 32-byte hex string
whose algorithm and codec live in a comment — and *vendored copies* of the
addressing code, which drift and silently start minting different addresses for
the same value.

<a id="CRAFT-16"></a>

### History is tamper-evident and invertible
**Law.** Any structure that records what happened is **append-only and
hash-linked**. History is not edited in place. Where the record must change,
the change is itself a recorded event that (a) leaves **forensic evidence** —
the prior state stays addressable and the mutation names what it shadowed — and
(b) carries an **inverse** the runtime can apply to get back.
**Why.** The two properties answer different questions. Tamper-evidence answers
*"has this been changed?"* — detection. Invertibility answers *"can we get
back?"* — recovery. A system with only the first can prove it was corrupted but
not repair itself; one with only the second can undo damage it cannot detect.
Neither alone is enough, and the pair is what separates a record from a mutable
blob with good intentions.
**Serves `provenance`** (the record resolves to its origin) **and `scar`** (a
corruption that can be detected can be regression-tested).
**Discharge.** Verify the chain, and exercise the inverse in a test that
mutates, reverts, and asserts byte-equality with the pre-state.
**Prior art to lift, not reinvent.** *Revertible effects* — every context
transformation carrying a tracked inverse, composed so the property survives
interleaving — are formalized in *A Programming Paradigm for Spatiotemporal
Composability* (github.com/cordiverse/paper, draft 2026-08-13), the theory under
the Cordis framework. Read it before designing an undo mechanism by hand.

<a id="CRAFT-17"></a>

### Evidence nobody reads is decoration
**Law.** Writing integrity evidence is not the obligation; **checking it is.**
Every hash chain, digest, signature, or attestation must have a **verifier on a
production path**, and that verifier must be reachable from a caller that runs
without a test harness. A verification function whose only callers are tests is
an unmet obligation, not a feature.
**Why.** This is `CRAFT-05` pointed at integrity. The cost of the chain is
paid on every append; the benefit is realized only where something reads it back
and *refuses*. In between, the chain buys nothing but the feeling of having one
— which is worse than none, because it is cited as though it were protection.
**Serves `refusal`** (evidence that cannot stop anything is not a gate) **and
`provenance`**.
**Discharge.** Grep for the verifier's call sites and require at least one
outside `#[cfg(test)]`, `tests/`, and benches. Then delete the chain or wire the
verifier — both are honest; the current state is not.
**This has already happened here.** `newt-agent`'s
`ConversationStore::verify_chain` is called by **every append and read by
nothing in production** — tests and one offline bench script only — while
restore feeds unverified turn rows straight into `restore_turns` (tracked as
newt-agent#1785). The tamper evidence has been written on every turn, for
months, and never once consulted. Assume this failure mode is the default, not
the exception.

---

## Identity, versioning, and how to cite

Every law has a **stable ID** (`CRAFT-NN`). The ID is permanent: it is never
renumbered, never reused for a different law, and survives rewording or
reordering of the prose. A law's *text* may improve; its identity may not move.
That is what makes a citation from another repository — or from a checkout two
years old — resolvable.

The heading anchors are additive. `#CRAFT-02` and the older name-derived
anchor both resolve, so citations written before IDs existed keep working.

**Cite the ID and the version.** A bare name is ambiguous once a law is
reworded; a bare ID is unambiguous but does not say *which* wording you relied
on:

> per `CRAFT-02` (Craft Register v1.0)

**Pin exactly when it matters.** For a decision record whose reasoning depends
on the precise wording in force at the time, cite the release tag, which is
immutable:

> per `CRAFT-02`, `steward-charter@craft-v1.0`

### What bumps the version

| Change | Bump | Why |
|---|---|---|
| Wording, examples, formatting | PATCH | the obligation is unchanged |
| A new law added | MINOR | existing citations stay valid |
| A law's obligation narrowed, widened, or retired | MAJOR | a citation may no longer mean what it did |

A retired law keeps its ID and its section, marked retired with the version
that retired it. It is never deleted, because a citation to it must still
resolve — retirement is information, and a dangling ID is not.

### Versions

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-08-20 | Stable IDs assigned to the existing laws. No law's obligation changed. |
| 1.1 | 2026-08-21 | `CRAFT-15`–`CRAFT-17` added: derived identity, tamper-evident + invertible history, evidence-must-be-read. MINOR — existing citations unchanged. |

---

*This register is itself under the Charter. **Provenance:** it descends from the
line's scattered working doctrine — carried here faithfully, claiming to author
none of it. **Refusal:** it submits to the same falsification discipline as
[`VALIDATION.md`](VALIDATION.md) — any craft law whose removal costs nothing
should be cut. A craft law that cannot be wrestled with has become the Demiurge
in miniature.*
