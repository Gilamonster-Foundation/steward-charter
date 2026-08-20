# The Authority Register — how power is granted, bounded, and proven

**Version 1.0** · 10 laws, `AUTH-01`–`AUTH-10` · frozen 2026-08-20

> The authority doctrine of the line, a sibling to the Craft Register. The
> Charter says **what the system must guarantee**. The Craft Register says
> **how the code is built**. This says **how an agent's power is granted,
> attenuated, and later proven**, which is neither of those and was homeless
> between them.

Craft laws answer "is this buildable and legible". Authority laws answer a
different question: when this agent acted, what was it permitted to do, who
permitted it, and can anyone check afterwards. That question has been answered
the same way in dozens of per-repo decision records, each restating the
reasoning from scratch. Here it is stated once and cited.

Several of these are the mechanical face of an invariant. A default-deny gate
*is* a `refusal`. A signed grant *is* a `writ`. A content-addressed record *is*
`provenance`. Where a law is pure engineering with no invariant behind it, the
table says so.

| ID | Law | One line | Serves |
|---|---|---|---|
| `AUTH-01` | Fail closed | absent, malformed, or unverifiable input denies | `refusal` · `writ` |
| `AUTH-02` | Attenuate, never amplify | authority may narrow as it travels, never widen | `writ` |
| `AUTH-03` | Amplification needs the human root | a standing grant is a live human act | `tether` · `writ` |
| `AUTH-04` | Permissive is a posture, not a bypass | maximum authority still flows through the gate | `writ` · `provenance` |
| `AUTH-05` | One authority vocabulary | a permission has one spelling across the line | `novice` · `writ` |
| `AUTH-06` | Make the unsafe state unrepresentable | prefer types over per-site checks | `refusal` |
| `AUTH-07` | Identity is content-addressed | a name is a function of the bytes it names | `provenance` |
| `AUTH-08` | History is a chain; mutability is one ref | append immutable records, move a single head | `provenance` · `scar` |
| `AUTH-09` | Confinement is environmental and in-process | the sandbox and the leash are both required | `writ` · `refusal` |
| `AUTH-10` | An authority decision is observable | what was permitted is recoverable afterwards | `provenance` · `novice` |

---

## I. Grant — how authority arrives

<a id="AUTH-01"></a>
### Fail closed

Absent, malformed, expired, or unverifiable authority denies. This is the
default in every direction: an unparsed policy is not an empty policy, a
missing signature is not a valid one, and a side call that errors does not
proceed on assumption.

The failure mode this prevents is not a breach, it is a *silent* breach. A
system that opens on error will do so at the moment its inputs are least
trustworthy, and will look identical to one that was never tested.

<a id="AUTH-02"></a>
### Attenuate, never amplify

Authority may narrow as it travels and may never widen. A delegated agent, a
spawned tool, a child process, or a forwarded request receives at most what its
parent held, expressed as a meet against the parent's grant.

This is what makes a grant possible to reason about at a distance. If any hop can
widen, then reading the grant at the root tells you nothing about what happens
at the leaf, and the whole chain has to be re-derived to answer a question the
first line should have answered.

<a id="AUTH-03"></a>
### Amplification needs the human root

Narrowing is free and needs no ceremony. Widening is not, and a standing grant
is the widest form there is. Enlarging authority requires a live human act,
not a configuration file and not an environment variable, because a file can be
written by anything that can write files.

The mechanical form of that act is an attestation: a signed human decision
recorded where it can be checked later.

## II. Posture — how much authority is in force

<a id="AUTH-04"></a>
### Permissive is a posture, not a bypass

A layer may legitimately choose maximum authority. A local development agent
with full host access is a reasonable configuration, not a defect. What is not
legitimate is expressing that by *skipping the mediation*.

A bypass and a top grant look the same from the outside and differ completely
when something goes wrong. After a bypass there is no record of what authority
was in force, because nothing evaluated any. After a top grant there is: the
gate ran, it was handed everything, and it says so.

So maximum permission is spelled as a capability that flows through the same
gate as every other capability. A permissive layer keeps its convenience and
loses nothing but the blind spot.

<a id="AUTH-05"></a>
### One authority vocabulary

A permission has one spelling across the line. Not one enforcement mechanism,
which may reasonably differ by platform, but one *name*: the same type, the
same field, the same word in configuration.

A layer that invents a second vocabulary has not extended the system; it has
forked it, and every later question about what an agent could do has two
answers that must be reconciled by hand.

## III. Structure — how authority is made checkable

<a id="AUTH-06"></a>
### Make the unsafe state unrepresentable

Where the same defect can occur at many call sites, a per-site fix inherits the
sprawl and will be incomplete on the day someone adds site N+1. Prefer types,
required parameters, and lifetimes that make the wrong call fail to compile.

This is the authority-side statement of the Craft Register's structural laws.
The test of a good seam is not that the unsafe call is caught, it is that the
unsafe call cannot be written.

<a id="AUTH-07"></a>
### Identity is content-addressed

Where a thing is identified, prefer a name derived from its bytes over a name
issued by an allocator. Allocated identity carries a family of failure modes
that content addressing does not have: double issue, reuse after free,
concurrent claim, and rebinding a name to different content.

Those are not hypothetical. They are the ordinary failures of any registry
under concurrency, and each is guarded separately until the identity scheme
changes and they all stop existing.

<a id="AUTH-08"></a>
### History is a chain; mutability is one ref

Some things do change: who owns a context, which model is selected, what
the current policy is. That does not make content addressing inapplicable. It
means the *history* is immutable and content-addressed, and the mutable part is
reduced to a single reference that names a point in it.

This is the shape of a version-control system, and it is the right shape for
authority: every grant, transfer, and revocation is an appended record, and the
question "what is true now" is answered by folding the chain rather than by
trusting a field someone may have overwritten.

## IV. Proof — how it is checked afterwards

<a id="AUTH-09"></a>
### Confinement is environmental and in-process

A sandbox bounds what the process can reach. A leash bounds what the agent asks
for. Neither substitutes for the other: a hermetic environment with no leash
cannot express intent, and a leash inside an unbounded environment is advice.

Where one is deliberately relaxed, the other carries the weight, and that
trade should be written down as an invariant rather than assumed. "Unleashed
only when hermetic" is a legitimate position. "Unleashed" is not.

<a id="AUTH-10"></a>
### An authority decision is observable

What was permitted must be recoverable after the fact, by someone who was not
present. That means the decision is recorded, not merely made: which grant was
in force, what was asked, what the gate answered.

An unobservable decision cannot be audited, cannot be regression-tested, and
cannot be reasoned about in an incident. It also cannot be distinguished from
the case where no decision happened at all, which is the specific confusion
that makes a bypass expensive later.

---

## Citing this register

Repositories in the line should cite these laws rather than restate them. A
decision record that needs fail-closed behaviour says so and links here; it
does not re-argue the case. Where a repository deviates, the deviation and its
reason belong in that repository, stated against the law it departs from.

When a law here and a repository's own instructions disagree, this register
wins, and the repository's instructions are the thing to fix.

## Identity, versioning, and how to cite

Every law has a **stable ID** (`AUTH-NN`). The ID is permanent: it is never
renumbered, never reused for a different law, and survives rewording or
reordering of the prose. A law's *text* may improve; its identity may not move.
That is what makes a citation from another repository — or from a checkout two
years old — resolvable.

The heading anchors are additive. `#AUTH-02` and the older name-derived
anchor both resolve, so citations written before IDs existed keep working.

**Cite the ID and the version.** A bare name is ambiguous once a law is
reworded; a bare ID is unambiguous but does not say *which* wording you relied
on:

> per `AUTH-02` (Authority Register v1.0)

**Pin exactly when it matters.** For a decision record whose reasoning depends
on the precise wording in force at the time, cite the release tag, which is
immutable:

> per `AUTH-02`, `steward-charter@auth-v1.0`

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
