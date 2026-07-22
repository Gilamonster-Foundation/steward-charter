# The Craft Register — how the invariants get built

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

| Law | One line | Serves |
|---|---|---|
| Loosely coupled, functionally cohesive | group by function, narrow seams | `novice` |
| The three Cs | Composition · Configuration · Convention | `novice` |
| Composition roots are manifests | the root wires, it does not implement | `novice` · `writ` |
| Freeze minimally, ratchet forward | lock as little as possible | `refusal` · `provenance` |
| Law minimalism | no law without a proof obligation | `refusal` |
| Name properties, pin at the edge | laws name properties; identifiers self-describe | `provenance` |
| TDD + the regression rule | a fix without a failing-first test is incomplete | `scar` |
| Coverage, reported and near-total | ~100% on the production view, every PR | `novice` |
| Tests separated from source | source is production-only | `novice` |
| Least lines of code | delete surface rather than test it | `writ` · `refusal` |
| Keep files small | split by cohesion past a soft cap | `novice` |
| One issue, one PR, merge on green | small reviewable increments | `scar` · `novice` |
| Zero warnings | the gate blocks on any warning | `refusal` |
| Hooks mirror pipelines | local pre-flight = the authoritative gate | `tether` · `refusal` |

---

## I. Structure — how the parts relate

### Loosely coupled, functionally cohesive
**Law.** Modules group by *function*, not by category; seams are narrow; a
single logical change lands in a single cohesive place.
**Why.** Coupling is the tax you pay on every future change. Cohesion by
function (not "all the models here, all the services there") means the thing
that changes together lives together.
**Serves `novice`.** A fresh, context-light instance can read one seam and
understand it without swallowing the whole tree — the precondition for the
fresh-eyes challenge to be real rather than ornamental.

### The three Cs — Composition, Configuration, Convention
**Law.** Prefer composing small pieces over growing large ones; expose
variation as configuration at the seam; lean on convention over ceremony.
**Why.** The three give you flexibility without a framework: behavior is
assembled, tuned, and defaulted rather than hard-wired.

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

### Freeze minimally; ratchet forward
**Law.** Lock down as little as possible per step. Even "current" pins are
provisional and rotate via a forward ratchet, not a permanent freeze.
**Why.** Every freeze forecloses futures. A smaller freeze is cheaper to carry
and cheaper to correct.
**Serves `refusal`** (fewer frozen commitments keep more choices declinable
later) **and `provenance`** (a self-describing artifact needs no frozen
algorithm — the identifier already names its own scheme).

### Law minimalism
**Law.** Admit only the laws the system actually needs. Nothing enters the law
layer without a **proof obligation**; the algebra tells you what to cut.
**Why.** A law you cannot discharge is decoration, and decoration in the law
layer is where certainty ossifies.
**Serves `refusal`.** A law that carries no proof obligation is dogma that
cannot be challenged — exactly the thing the Novice must be able to question.

### Name properties, pin implementations at the edge
**Law.** Laws name *properties* (collision-resistance, determinism, ordering);
*profiles* pin the algorithm; *identifiers self-describe* (a multihash, not a
bare "sha256" assumption).
**Why.** Binding a law to one algorithm dies the day the algorithm does. Naming
the property survives the rotation.
**Serves `provenance`.** An identifier that carries its own scheme resolves to
its origin without an out-of-band assumption.

---

## III. Craft — how code earns trust

### TDD, and every fix carries a regression test
**Law.** Red before green. A bug fix is incomplete without a test that *would
have failed before the fix* and passes after — verified against the old code
path, not assumed.
**Why.** The test is the durable record that the failure happened and was
metabolized, not merely patched.
**Serves `scar`.** A regression test *is* a scar in code: error plus correction,
kept as first-class, permanent state so the same wound cannot silently reopen.

### Coverage, reported and near-total
**Law.** Every change reports its coverage; new modules target ~100% line and
function coverage, measured on the **production view** (test files excluded so
the number is honest). Residue that is pure unused-monomorphization accounting
may be waived — but named, not hidden.
**Serves `novice`.** Coverage is the fresh-eyes defect-catch metric made
mechanical: an uncovered line is a claim no one has challenged.

### Tests separated from source
**Law.** Source files carry production code only; test suites live beside them,
not inside.
**Why.** It keeps the coverage measurement honest (no test scaffolding inflating
the source numbers) and the source legible.
**Serves `novice`.**

### Least lines of code
**Law.** Delete unused surface rather than test it. No speculative API, no
defensive assertion a law test already pins. Re-admit additively (a
`#[non_exhaustive]` enum, a later export) the day a real consumer appears.
**Serves `writ` and `refusal`.** Less surface is less to audit and less
authority to leak; the smallest thing that does the job is the most declinable.

### Keep files small
**Law.** New source files stay under a soft cap (~2,500 lines); past it, split
by cohesion.
**Why.** A file no one can hold in their head is a file no fresh instance can
challenge.
**Serves `novice`.**

---

## IV. Flow — how change lands

### One issue, one PR; merge on green
**Law.** One logical change per branch; small, reviewable increments; each merge
a checkpoint; no long-lived branches. Draft status is a hard stop — never merged
past.
**Serves `scar`** (small increments localize failure so it can be metabolized)
**and `novice`** (a diff a fresh reviewer can actually hold).

### Zero warnings
**Law.** The gate blocks on any warning — lints denied, formatting checked.
**Why.** Warnings accumulate into noise, and noise is where real defects hide.
**Serves `refusal`.** A quality gate that can decline is a refusal made
mechanical.

### Hooks mirror pipelines
**Law.** The local pre-flight (push hook, `just check`) runs the *same* checks
as the authoritative CI gate, and the two are kept in parity by rule — editing
one triggers an audit of the others.
**Serves `tether`** (a fast local pre-flight before the authoritative gate)
**and `refusal`** (the gate itself).

---

*This register is itself under the Charter. **Provenance:** it descends from the
line's scattered working doctrine — carried here faithfully, claiming to author
none of it. **Refusal:** it submits to the same falsification discipline as
[`VALIDATION.md`](VALIDATION.md) — any craft law whose removal costs nothing
should be cut. A craft law that cannot be wrestled with has become the Demiurge
in miniature.*
