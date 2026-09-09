# The Craft Register — how the invariants get built

**Version 2.1** · 21 laws, `CRAFT-01`–`CRAFT-21` · frozen 2026-09-09

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
| `CRAFT-18` | Evidence proportional to surprise | a claim names the decision it changes | `provenance` · `refusal` |
| `CRAFT-19` | The ladder — reach for the least first | reuse before build; the rung you stop at is a decision | `refusal` · `novice` |
| `CRAFT-20` | Output is another program's input | a fact reachable only through a UI is not reported | `provenance` · `novice` |
| `CRAFT-21` | Silence unless surprising | noise is where real defects hide | `scar` · `refusal` |

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
**What this law does not reach** is enumerated in
[`CRAFT-19`](#CRAFT-19) — unchanged obligation, stated where the procedure
lives, because "unused surface" was never meant to include the code that makes
the surface safe.
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

<a id="CRAFT-19"></a>
### The ladder — reach for the least first
**Law.** Before writing code, stop at the first rung that holds: (1) does this
need to exist at all; (2) does this repository already do it; (3) does the
standard library; (4) does the platform; (5) does a dependency already present;
(6) is it one line; (7) only then, the minimum that works. **The rung you stop
at is a decision and is stated** — "skipped, YAGNI" or "reused `X`" is an
answer, and an unstated rung is an unexamined one.
**Why.** [`CRAFT-10`](#CRAFT-10) names the outcome — least surface — but an
outcome is not a procedure, and "write less" is advice nobody can follow under
pressure. The ladder is the procedure, and its order is not arbitrary: each
rung is cheaper to *carry* than the one below it, because the cost of code is
paid at every future read, not at the moment it is typed.
**The ladder never reaches these**, at any rung: validation at a trust
boundary, error handling that prevents data loss, a security control, an
accessibility affordance, or anything the requester asked for explicitly. They
are not surface — they are the reason surface is safe to expose, and a line
count cannot tell them apart from padding. The ladder removes what nothing
depends on; it never removes what *failure* depends on. This states, where the
procedure lives, a boundary [`CRAFT-10`](#CRAFT-10) always had: "unused
surface" never meant these.

**The ladder shortens the solution, never the reading.** It runs *after* the
problem is understood — every seam the change touches, traced. The smallest
diff in the wrong place is not economy, it is a second defect, and it is more
expensive than the code it saved because it also spends the reviewer's trust.
**Serves `refusal`** (the least thing is the most declinable) **and `novice`**
(the fresh reader inherits rungs 1–6 as absence, which costs nothing to read).

<a id="CRAFT-20"></a>
### Output is another program's input
**Law.** Anything worth reporting is emitted in a form a *second* program can
consume — a structured event, a stream, a file — before it is rendered for a
human. A dashboard, a TUI or a chat message is **a** consumer of that stream,
never the only route to the fact.
**Why.** A fact reachable only by looking at a rendering cannot be tested,
diffed, archived, or replayed, so it cannot participate in
[`CRAFT-16`](#CRAFT-16)'s tamper-evident history or
[`CRAFT-17`](#CRAFT-17)'s verifier. It is not evidence; it is a picture of
evidence. This is also what makes composition possible at all: a component
whose output only a human can read has no downstream, and every later need
becomes a modification of it rather than a new thing beside it.
**Serves `provenance`** (a rendering has no chain) **and `novice`** (a stream
can be read without running the UI that renders it).

<a id="CRAFT-21"></a>
### Silence unless surprising
**Law.** When there is nothing surprising to report, report nothing. Success is
quiet; a log line, a notification or an alert must name something the reader
would act on.
**Why.** The same argument as [`CRAFT-13`](#CRAFT-13): noise is where real
defects hide. But the failure here is worse than in a build log, because a
channel that cries routinely is not merely ignored — it is *muted*, and a muted
channel keeps consuming the belief that something is watching. Coverage that
exists and is not read is [`CRAFT-17`](#CRAFT-17)'s decoration wearing an
operational costume.
**The obligation runs both ways.** Silence must mean "nothing happened", never
"the reporter died" — so absence of an expected signal is itself surprising,
and must be detectable. A monitor that cannot distinguish a healthy system from
a dead collector reports nothing in both cases and is honest in neither.
**Serves `scar`** (a signal that survives being read) **and `refusal`** (a
channel that stays worth interrupting for).

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

<a id="CRAFT-18"></a>
### Evidence proportional to surprise
**Law.** A claim in a review artifact — a PR body, an ADR, a finding, a commit
message — earns its place by **naming a decision that changes if it is false**.
Evidence is carried in proportion to how *surprising* the claim is: an expected
claim gets an assertion, a surprising one gets the receipt. Every figure is
graded **measured** (it was run, and its output **is shown**), **derived** (it
follows from something measured, and the step is shown), or **believed**
(unverified). A claim that can name neither a decision nor a grade is cut.
**Why.** Length is a proxy, and proxies are gamed. The failure this addresses
is not a long document but a **flat** one: uniform evidence density across
claims of wildly different value, which buries the one finding worth
challenging under a hedge of findings nobody doubted. Flat density is the
signature of performed rigor, and it is *cheaper* to produce than the real
thing, so a review culture that rewards visible thoroughness will select for
it. Grading makes the padding expensive: the writer must either mislabel —
visible, and an offence against the gate — or write `believed` beside their own
filler, which nobody does twice.
**The proof obligation** (per [`CRAFT-05`](#CRAFT-05)): for each claim, the
decision it changes and the grade of its warrant. Both are answerable at review
time, by a human or an arbiter, without running anything.
**Not a word budget.** Compression along no seam is worse than length — the
same rule [`CRAFT-11`](#CRAFT-11) states for files. The fix direction is
*removing unearned claims*, never *shortening earned ones*; a load-bearing
justification deleted to hit a budget is a defect wearing the costume of
concision, and therefore worse than the padding it replaced.
**Discharge.** For a `measured` figure the command and its output appear in the
artifact, not a description of them. A figure whose output is merely
*producible* is `believed` until it is produced. That gap is not pedantry: it is
where a plausible number gets written before it is run. **This has already
happened here.** The PR that introduced this law to `newt-agent` (#2208)
published a grep, a result, and the phrase "verified before writing" in a single
pass, none of it executed; the command as printed did not even run, and the real
output contradicted it. The conclusion survived; the evidence did not. Nothing
caught it but a reread, because no gate reads a PR body. Assume this failure
mode is the default, not the exception.
**Kin to [`CRAFT-17`](#CRAFT-17), and not a duplicate of it.** That law governs
*machine-checkable* evidence — a chain nobody verifies in production is
decoration. This governs *prose* evidence — a claim no decision depends on is
the same decoration in a register no verifier can reach. `CRAFT-17`'s discharge
is a grep for call sites; this one's is a reader, which is why it is a rubric
and not a gate.
**Serves `provenance`** (a claim that self-describes its warrant is a claim you
can audit) **and `refusal`** (a reader can only challenge what is legible as a
claim; flat evidence hides the challengeable one).

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
| 1.2 | 2026-09-07 | `CRAFT-18` added: evidence proportional to surprise. MINOR — existing citations unchanged. |
| 2.0 | 2026-09-07 | `CRAFT-18` narrowed: a `measured` figure's output **is shown**, not merely showable. MAJOR — a citation to v1.2 asked for less. |
| 2.1 | 2026-09-09 | `CRAFT-19`–`CRAFT-21` added: the ladder, output-is-input, silence-unless-surprising. MINOR — existing citations unchanged. `CRAFT-10` gained a pointer to `CRAFT-19`'s boundary; its obligation is untouched, which is why this is not MAJOR. |

---

## Lineage — where this thinking comes from

Stated because a reader who wants the reasoning behind a law should be able to
find its roots, and because getting the direction of descent right matters.

**Doug McIlroy is the deep source.** *"Make each program do one thing well.
Expect the output of every program to become the input to another."* These laws
were worked out with that taken in early and applied for years — including in
object-oriented design well before any of this was Rust. That transfer is the
point: McIlroy's principles are not about pipes and text streams, they are
about **decomposition and composition**, and "program" scales to whatever the
unit of assembly happens to be — an object, a module, a crate, a service, an
agent. [`CRAFT-01`](#CRAFT-01), [`CRAFT-03`](#CRAFT-03) and
[`CRAFT-20`](#CRAFT-20) are that idea at the units this line assembles.

**Eric Raymond's rules are a sibling, not a source.** He read McIlroy deeply
too and restated it at length; where a law here rhymes with one of his, both are
downstream of the same ancestor rather than one citing the other. That
independent arrival is worth more than a citation would be: two readings of the
same source, applied in different decades to different materials, landing in the
same place is evidence the source was right.

**[ponytail](https://github.com/DietrichGebert/ponytail) is a separate lineage
that converges.** It is agent-era — a discipline for how much code a *model*
should write — and it reaches [`CRAFT-10`](#CRAFT-10)'s conclusion by a
different road, along with the carve-out now stated in
[`CRAFT-19`](#CRAFT-19). Convergence from an unrelated starting point is
corroboration; it is not provenance, and this register does not descend from it.

**None of the above authored these laws.** They are this line's own formulation,
tested against this line's own scars. The genealogy is offered so the reasoning
can be traced, not to lend borrowed authority — and where a law here departs
from any of these sources, the law wins, because it is the one that has been
falsified against real work.

---

*This register is itself under the Charter. **Provenance:** it descends from the
line's scattered working doctrine — carried here faithfully, claiming to author
none of it. **Refusal:** it submits to the same falsification discipline as
[`VALIDATION.md`](VALIDATION.md) — any craft law whose removal costs nothing
should be cut. A craft law that cannot be wrestled with has become the Demiurge
in miniature.*
