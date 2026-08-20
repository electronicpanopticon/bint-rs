# EPIC-01: Total Bint — No Panicking Arithmetic (TB)

> **Reconciled 2026-08-20 at commit `5477aad`.** This EPIC was drafted against
> `4c90d10` (2023-07-02), when `src/lib.rs` held one type in ~60 lines. It now
> holds three types in 927 lines. The arithmetic goal has largely landed; the
> encapsulation goal has not. Original line citations were stale and have been
> repointed. See `## Implementation corrigendum` for the design-vs-actual deltas.

## Context

`bint` exposes three bounded-integer types, all built on the same modular
arithmetic:

- `Bint` (`src/lib.rs:27`) — immutable, `Copy`, two public fields
  (`pub value: u8` at `:28`, `pub boundary: u8` at `:29`).
- `BintCell` (`src/lib.rs:303`) — interior mutability via `Cell<u8>` (`:304`),
  with `pub boundary: u8` still exposed (`:305`).
- `DrainableBintCell` (`src/lib.rs:603`) — a `BintCell` (`:604`) plus a
  `Cell<usize>` capacity (`:605`) that turns exhaustion into `None`.

**Arithmetic totality is achieved.** All four `Bint` arithmetic methods are
total for every `(value, boundary)` pair the type permits:

- `up()` (`src/lib.rs:86`) delegates to `up_x(1)`.
- `up_x()` (`src/lib.rs:102`) widens to `u16` before adding, so no overflow.
- `down()` (`src/lib.rs:133`) guards `boundary == 0` and `value == 0` before
  subtracting.
- `down_x()` (`src/lib.rs:167`) uses `i16::rem_euclid`, so no underflow.

`BintCell` and `DrainableBintCell` inherit that totality because every one of
their arithmetic methods routes through `Bint` (`src/lib.rs:358`, `:376`,
`:397`, `:414`). `DrainableBintCell::drain` (`src/lib.rs:677`) uses
`checked_sub(1)?`, so capacity exhaustion returns `None` rather than
underflowing.

Probe re-run at `5477aad` (debug profile), against the original Context table:

| Call | At `4c90d10` | At `5477aad` |
|---|---|---|
| `Bint{value:255, boundary:6}.up()` | PANIC | `Bint { value: 4, boundary: 6 }` |
| `Bint::new(0).up()` | PANIC | `Bint { value: 0, boundary: 0 }` |
| `Bint{value:0, boundary:0}.down()` | PANIC | `Bint { value: 0, boundary: 0 }` |
| `Bint{value:5, boundary:6}.up()` | `ok(0)` | `Bint { value: 0, boundary: 6 }` |

**Encapsulation is not achieved.** The public fields at `src/lib.rs:28-29` and
`:305` still admit states no constructor would produce, and no method rejects
them:

| Illegal state | Constructed by | Current behavior |
|---|---|---|
| `boundary == 0` | `Bint::new(0)`, `Bint { boundary: 0, .. }` | absorbing — `up`/`down`/`up_x`/`down_x` return self, forever `0` |
| `value >= boundary` | `Bint { value: 10, boundary: 6 }` | silently snaps: `.up()` yields `5`, `.down()` yields `3` |
| `value >= boundary` (cell) | assigning `cell.boundary` directly | value `199` under a forced boundary of `6` yields `up() == 2` |

Test state at `5477aad`: `cargo test` → **17 unit tests + 38 doctests, 0
failed**. `cargo clippy -- -D warnings` on the library → clean.
`cargo clippy --all-targets -- -D warnings` fails with 24 `clippy::unwrap_used`
findings, all inside `mod tests` and the doctests — test-only noise, not a
library defect, but it means the all-targets gate is not usable as written.

This EPIC makes the arithmetic total and the illegal states unrepresentable. It
does **not** widen `u8`, does not add `Add`/`Sub` operator impls, does not touch
the `Display` impls (`src/lib.rs:209`, `:526`), and does not change
`DrainableBintCell`'s capacity semantics.

---

## Status

*As of commit `5477aad`, 2026-08-20.*

| Component | Status |
|---|---|
| `Bint::down` total | **Complete** — guards at `src/lib.rs:136-144` |
| `Bint::up` total | **Complete** — `src/lib.rs:86` delegates to `up_x` |
| `Bint::up_x` / `down_x` total | **Complete** — `u16`/`i16` widening at `:111`, `:174` |
| `BintCell` total | **Complete** — routes through `Bint` at `:358`, `:376`, `:397`, `:414` |
| `DrainableBintCell` total | **Complete** — `checked_sub` at `:677` |
| `Boundary` newtype (non-zero, checked) | **Deferred** — superseded, see corrigendum §1 |
| Field encapsulation / constructors | Planned |
| Construction-time normalization | Planned |
| `clippy::arithmetic_side_effects` gate | Planned |
| **Demo on demand** (`cargo run --example bint_tour`) | Planned |

---

## Goals

- Make every public method on **Bint**, **BintCell**, and **DrainableBintCell**
  **total**: no input reaches a panic. *(achieved)*
- Make the **illegal state unrepresentable**: neither a `boundary` of 0 nor a
  `value >= boundary` can exist in a live instance. *(outstanding)*
- Make the **totality regression-proof**: a lint, not vigilance, catches the
  next raw `+`/`-` on a `u8` field. *(outstanding)*
- Preserve the published wrapping behavior for all currently-valid inputs.

## Scope

- `up()` on `value == boundary - 1` yields `0`. `down()` on `value == 0` yields
  `boundary - 1`. Unchanged from today.
- No method panics for any `(value, boundary)` pair the type system permits.
- A `value >= boundary` is normalized **once, at construction** — never
  re-derived at call time, and never silently absorbed mid-sequence.
- `boundary` is always >= 1. `Bint::new(0)` must not produce a usable `Bint`.
- The three types agree: the same `(value, boundary)` pair steps identically
  through `Bint`, `BintCell`, and a non-exhausted `DrainableBintCell`.

---

## Domain map

| Domain concept | Code construct | Status |
|---|---|---|
| Bounded integer, immutable | `struct Bint` (`src/lib.rs:27`) | ✅ total arithmetic |
| Bounded integer, mutable | `struct BintCell` (`src/lib.rs:303`) | ✅ total arithmetic |
| Bounded integer with a budget | `struct DrainableBintCell` (`src/lib.rs:603`) | ✅ total arithmetic |
| The boundary (a modulus) | `pub boundary: u8` (`:29`, `:305`) | ❌ no non-zero invariant, publicly writable |
| The value's in-range invariant | none | ❌ enforced only in `new_with_value` (`:63`), bypassed by literals |
| Wrap forward / backward | `up` / `down` / `up_x` / `down_x` | ✅ done |
| Exhaustible budget | `capacity: Cell<usize>` (`:605`), `drain` (`:677`) | ✅ done |
| Rendering | `impl Display` (`:209`, `:526`) | ✅ done |

---

## Design

### `Boundary` — a non-zero modulus *(deferred)*

The original sketch introduced `src/boundary.rs` wrapping `NonZeroU8`, so that
`% self.boundary` could not divide by zero. **The implementation solved that
problem a different way** — a runtime `if self.boundary == 0` guard at
`src/lib.rs:136` and `:168` — so the newtype is no longer needed *for
totality*.

It is still the cheapest way to get the *second* goal (unrepresentable illegal
state), so the sketch is retained for whenever encapsulation is picked up:

```rust
use std::num::NonZeroU8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boundary(NonZeroU8);

impl Boundary {
    pub fn new(n: u8) -> Option<Boundary> { NonZeroU8::new(n).map(Boundary) }
    #[must_use] pub fn get(self) -> u8 { self.0.get() }
}
```

Rationale, restated for today: the guard makes a zero boundary *safe*; it does
not make it *meaningful*. `Bint::new(0)` currently returns a value that silently
absorbs every subsequent operation — a bounded integer with no bound. That is a
failure the caller never sees. A `Boundary` returns `None` at the door instead.

### Total wrapping — as built

```rust
pub fn up(&self) -> Bint { self.up_x(1) }                       // src/lib.rs:86

pub fn up_x(self, x: u8) -> Bint {                              // src/lib.rs:102
    if self.boundary == 0 { return Bint { value: 0, boundary: 0 }; }
    let value = ((u16::from(self.value) + u16::from(x)) % u16::from(self.boundary)) as u8;
    Bint { value, boundary: self.boundary }
}
```

Rationale: widening to `u16` before the add is what makes `up` total, and it
holds for **any** `x`, not just `1`. Having `up` delegate to `up_x` rather than
carry its own `(value + 1) % boundary` removes the duplicate that drifted — at
`36265db`, `up()` panicked at `value == 255` while `up_x(1)` returned `4` for the
same input. One implementation cannot disagree with itself.

`down_x` (`src/lib.rs:167`) uses `i16::rem_euclid` rather than a widened `u16`,
because a backward step can go negative before it wraps; `rem_euclid` returns a
non-negative remainder where `%` would not.

### Encapsulated `Bint` *(outstanding)*

```rust
pub struct Bint { value: u8, boundary: Boundary }

impl Bint {
    pub fn new(boundary: u8) -> Option<Bint>;
    pub fn at(value: u8, boundary: u8) -> Option<Bint>;  // value % boundary
    #[must_use] pub fn value(&self) -> u8;
    #[must_use] pub fn boundary(&self) -> u8;
}
```

Rationale: with `value` normalized once at construction, `value <= boundary - 1`
is a standing invariant, and the `u16` widening in `up_x` becomes belt-and-braces
rather than the load-bearing safety mechanism. This is a **breaking** change —
struct-literal construction is the documented usage in `README.md:16` and in the
crate-level doctest at `src/lib.rs:18` — and warrants `0.2.0`, not a patch.

---

## Work Items

Checked boxes are landed at the stated commit. Unchecked boxes are outstanding.

### Phase 0 — Prerequisites & lint gating

- [ ] **0a.** Add `#![forbid(unsafe_code)]` and
      `#![warn(clippy::arithmetic_side_effects)]` at `src/lib.rs:1`, alongside the
      existing `clippy::pedantic` / `unwrap_used` / `expect_used` warns.
- [ ] **0b.** Triage the resulting findings. The three original 2023 sites are
      gone; the remaining raw arithmetic is inside the widened `u16`/`i16`
      expressions (`:111`, `:174`) and needs `#[allow]` with a one-line proof of
      why it cannot wrap.
- [ ] **0c.** Make `cargo clippy --all-targets -- -D warnings` usable: it fails
      today with 24 `unwrap_used` findings from `mod tests` and the doctests. Add
      `#![cfg_attr(test, allow(clippy::unwrap_used))]` or switch the doctests to
      pattern matching, then wire the all-targets form into the Makefile's
      `clippy` target.

### Phase 1 — `Boundary` *(deferred — unblock only with Phase 2)*

- [ ] **1.** Add `src/boundary.rs` with `Boundary::new` / `get`.
- [ ] **2.** Unit tests: `boundary_rejects_zero`, `boundary_accepts_one`.

### Phase 2 — Encapsulation

- [ ] **3.** Privatize `value` / `boundary` at `src/lib.rs:28-29` and `boundary`
      at `:305`; add accessors. `BintCell::value()` (`:494`) and
      `DrainableBintCell::value()` (`:730`) already exist as the pattern to copy.
- [ ] **4.** Add `Bint::at(value, boundary) -> Option<Bint>` normalizing
      `value % boundary`, and decide the fate of `new_with_value`
      (`src/lib.rs:63`), which currently *resets to 0* on out-of-range input
      rather than taking the remainder. Same call for `BintCell::set` (`:448`),
      which resets identically.
- [ ] **5.** Tests: `at_normalizes_out_of_range_value`,
      `struct_literal_no_longer_compiles` (as a `compile_fail` doctest),
      `cell_boundary_cannot_be_reassigned`.

### Phase 3 — Total arithmetic ✅ **Complete at `5477aad`**

- [x] **6.** Guard `boundary == 0` in `down()` / `down_x()` — `src/lib.rs:136`,
      `:168`.
- [x] **7.** Widen `up_x` to `u16` and `down_x` to `i16::rem_euclid` —
      `src/lib.rs:111`, `:174`.
- [x] **8.** Collapse `up()` onto `up_x(1)` — `src/lib.rs:86`. Fixes the
      `value == u8::MAX` overflow panic.
- [x] **9.** Test `up_at_u8_max_does_not_panic` (`src/lib.rs:780`). Verified to
      FAIL at `36265db` with `attempt to add with overflow`, and pass at
      `5477aad`.
- [x] **10.** Tests `up_default_defect` (`src/lib.rs:771`) and
      `down_default_defect` (`:801`) pin the zero-boundary guards.

### Phase 4 — Cell parity ✅ **Complete at `5477aad`** (retro-added)

- [x] **11.** `BintCell` routes all four arithmetic methods through `Bint`
      (`src/lib.rs:358`, `:376`, `:397`, `:414`), so totality is inherited, not
      re-implemented.
- [x] **12.** `DrainableBintCell::drain` uses `checked_sub(1)?`
      (`src/lib.rs:677`); exhaustion surfaces as `None`, not a panic.
- [x] **13.** `has_capacity` (`src/lib.rs:691`) lets a caller check before
      spending.
- [ ] **14.** Property test that `Bint`, `BintCell`, and a non-exhausted
      `DrainableBintCell` agree step-for-step over the same `(value, boundary, x)`
      inputs. No such cross-type test exists today.

### Phase 5 — Demo & docs

- [ ] **15a.** Build `examples/bint_tour.rs`, the artifact named in
      `## Demo on Demand`. `examples/` now exists (`examples/double.rs`), so this
      is an addition, not a new directory.
- [ ] **15b.** Run the demo runbook on a clean checkout; paste the real output
      into `docs/demos/bint_tour.txt`. That directory does not exist yet.
- [ ] **15c.** Update `README.md:13-23` and the crate doctest at `src/lib.rs:15-25`
      — both use struct-literal construction, which Phase 2 breaks — and point
      them at `cargo run --example bint_tour`. Note that `README.md` IS
      already doctested: `#![cfg_attr(doc, doc = include_str!("../README.md"))]`
      at `src/lib.rs:3` pulls it in, and its three code blocks run as
      `src/lib.rs - (line 15)`, `(line 29)`, `(line 43)`. Phase 2 will break the
      build, not just the docs — which is the desired signal.

---

## Test Plan

**Landed:**

- `up_at_u8_max_does_not_panic` (`src/lib.rs:780`) — pins the `u8` overflow.
  Failed at `36265db`, passes at `5477aad`.
- `up_default_defect` (`src/lib.rs:771`), `down_default_defect` (`:801`) — pin
  the zero-boundary guards.
- 38 doctests across all three types — pin the published wrap behavior.

**Outstanding:**

- `boundary_rejects_zero` — pins "a zero modulus is unrepresentable".
- `at_normalizes_out_of_range_value` — pins that normalization happens once, at
  construction.
- `struct_literal_no_longer_compiles` — a `compile_fail` doctest proving the
  encapsulation actually closed the hole.
- Cross-type agreement property test (Work Item 14).

**Coverage note.** `make mutants` (cargo-mutants, added at `36265db`) is the tool
that answers "would a real behavioral change make a passing test fail?" It has
not been run against the arithmetic paths since the `up` collapse. Run it before
declaring Phase 3 audited rather than merely green.

## Key Files

| File | Role |
|---|---|
| `src/lib.rs` | all three types, the arithmetic, `Display`, and `mod tests` |
| `src/boundary.rs` | planned — the non-zero modulus (Phase 1, deferred) |
| `examples/double.rs` | existing example; the `Bint` odometer in miniature |
| `examples/bint_tour.rs` | planned — the demo artifact (Phase 5) |
| `README.md` | usage examples that Phase 2 breaks |
| `Makefile` | `ayce`, `mutants`, `miri`, `deny` — the verification surface |

## Reuse (do NOT recreate)

- `std::num::NonZeroU8` — the non-zero invariant already exists in std; do not
  hand-roll a checked wrapper.
- `Bint::up_x` (`src/lib.rs:102`) and `down_x` (`:167`) — the widened arithmetic
  is the single source of truth. Every other method delegates to them. Do not add
  a fifth arithmetic path.
- `BintCell::value()` (`src/lib.rs:494`) — the accessor pattern Phase 2 copies.
- `src/lib.rs:209`, `:526` — the `Display` impls are correct as written; the demo
  prints through them rather than formatting fields directly.

## Compatibility

- **Preserves** the wrapping semantics for every `(value, boundary)` pair that
  did not panic at `4c90d10`.
- **Changed, silently:** `Bint{value:255, boundary:N}.up()` aborted the process
  before `5477aad`; it now returns `(256 % N)`. No caller could have depended on
  the panic, so this ships as a patch.
- **Breaks (Phase 2, not yet shipped):** direct field access —
  `Bint { value: 5, boundary: 6 }` and `cell.boundary` — which is the documented
  usage in `README.md:16` and the crate doctest at `src/lib.rs:18`. That warrants
  `0.2.0`.

## Dependencies

- **Blocks:** none currently in this repo.
- **Built on:** the 17 unit tests and 38 doctests green at `5477aad`.
- **Related:** none. `BintCell::static_up_x` / `static_down_x`
  (`src/lib.rs:468`, `:489`) exist to serve a downstream poker library, per their
  own doc comments — check that consumer before breaking field access in Phase 2.

## Verification

```bash
cargo build
cargo test
cargo clippy -- -D warnings
make mutants
cargo run --example bint_tour
```

Exit criteria:

1. ✅ The three probe inputs (`{255,6}.up()`, `new(0)`, `{0,0}.down()`) return a
   value — none panic. *Met at `5477aad`.*
2. ✅ All pre-existing unit tests and doctests still pass. *Met — 17 + 38, 0
   failed.*
3. ⬜ `cargo clippy --all-targets -- -D warnings` is clean. *Not met — 24
   `unwrap_used` findings in tests.*
4. ⬜ `README.md` compiles as a doctest against the encapsulated API.
5. ⬜ The `## Demo on Demand` runbook runs clean on a fresh clone, at HEAD, with
   no manual setup beyond the documented commands.

---

## Implementation corrigendum

*Written 2026-08-20, reconciling the 2023 design against `5477aad`.*

1. **`Boundary` was never built; a runtime guard was used instead.** The design
   called for `NonZeroU8` to make division-by-zero impossible. The code shipped
   `if self.boundary == 0 { return ... }` at `src/lib.rs:136` and `:168`. Both
   reach totality. Only the newtype reaches *unrepresentability* — so Phase 1 is
   marked **Deferred**, not Complete, and the second goal of this EPIC remains
   open. The cost of the substitution: `Bint::new(0)` returns a bounded integer
   with no bound, which absorbs every later operation without ever telling the
   caller.

2. **The overflow outlived the div-by-zero by three years.** The 2023 Context
   listed three panic sites. Two were closed by the zero-boundary guards. The
   third — `(self.value + 1) % self.boundary` overflowing at `value == u8::MAX` —
   survived because the fix was applied where the *reported* crash was, not
   across the whole method family. `up_x` was written correctly with `u16`
   widening at the same time `up` was left with raw `u8` arithmetic. Closed at
   `5477aad` by making `up` delegate to `up_x(1)`.

3. **Duplication, not arithmetic, was the root cause.** Two implementations of
   "step forward by n" drifted until they disagreed on the same input: `up()`
   panicked at `{255,6}` while `up_x(1)` returned `4`. The fix deleted code
   rather than adding a guard.

4. **`down_x` diverges from `up_x` deliberately.** `up_x` widens to `u16`;
   `down_x` uses `i16::rem_euclid` (`src/lib.rs:174`). A backward step can go
   negative before wrapping, and `%` on a negative left operand returns a negative
   remainder in Rust. This is correct, not an oversight, and should survive any
   future "make these symmetric" refactor.

5. **Normalization went the other way.** The design said a `value >= boundary` is
   *normalized* (`value % boundary`). `Bint::new_with_value` (`src/lib.rs:63`)
   and `BintCell::set` (`:448`) instead **reset to 0**. Meanwhile a struct literal
   normalizes at *call* time — `Bint{10,6}.up()` yields `5` — which is exactly the
   "never at call time" the Scope forbids. Three behaviors for one rule. Work
   Item 4 must pick one.

6. **The crate tripled while the EPIC stood still.** `BintCell` and
   `DrainableBintCell` did not exist at `4c90d10`. They inherit totality by
   routing through `Bint`, which is the right structure and is why Phase 4 could
   be marked Complete retroactively — but no test asserts that the three types
   *agree*, so the inheritance is a reading of the code, not a pinned invariant.
   That gap is Work Item 14.

### Phase status summary

| Phase | Scope | Status |
|---|---|---|
| 0 — Lint gating | `arithmetic_side_effects`, all-targets clippy | Planned |
| 1 — `Boundary` | non-zero newtype | **Deferred** (superseded by runtime guards) |
| 2 — Encapsulation | private fields, `at()`, one normalization rule | Planned |
| 3 — Total arithmetic | `up`, `down`, `up_x`, `down_x` | **Complete** at `5477aad` |
| 4 — Cell parity | `BintCell`, `DrainableBintCell` | **Complete** at `5477aad` (item 14 open) |
| 5 — Demo & docs | `bint_tour`, README | Planned |

### Inherited debt

- Public fields on two types (`src/lib.rs:28-29`, `:305`) keep every illegal
  state constructible. Totality means those states no longer crash — it does not
  mean they are correct. A silently-absorbing `Bint` is arguably worse than the
  panic it replaced, because nothing reports it.
- Three conflicting answers to "what happens when `value >= boundary`"
  (reset-to-0, remainder-at-call-time, and untouched-in-a-literal) are live in
  the published API simultaneously.
- `cargo clippy --all-targets -- -D warnings` does not pass, so the strictest
  gate in the Makefile cannot be turned on.
- `make mutants` has not been run against the arithmetic since the `up` collapse;
  Phase 3's green is untested-by-mutation.

---

## Demo on Demand

**Demo artifact:** `examples/bint_tour.rs` — committed alongside the code. A
library crate has no binary, so `cargo run --example` is the cheapest tactile
surface here; it needs no new dependency and no `[[bin]]` section.
`examples/double.rs` already proves the pattern works in this repo.

**Audience:** a crates.io consumer deciding whether `bint` is safe to embed, and
any future contributor who needs to see the wrap behavior before changing it.

**Runbook** — exact commands, copy-pasteable, no editing required:

```bash
git clone https://github.com/electronicpanopticon/bint-rs && cd bint-rs
cargo run --example bint_tour
```

**What the observer sees:** four labeled blocks printed to stdout.

1. *The odometer* — a `Bint` with a boundary of 6 counted up 8 times, printing
   `0 1 2 3 4 5 0 1`, then counted down 3 times from 0, printing `5 4 3`. The
   wrap is visible as a number rolling over, not as an assertion passing.
2. *The edges that used to panic* — the three inputs from the Context probe, each
   printed as `input -> result`, all three returning a value. At `4c90d10` the
   process aborted on the first line; at `36265db` it aborted on the first line
   only.
3. *The budget* — a `DrainableBintCell::new(4, 4)` stepped five times, printing
   `Some(1) Some(2) Some(3) Some(0) None`. Exhaustion is a value, not a crash.
4. *The illegal state that remains* — `Bint::new(0)` stepped three times,
   printing `0 0 0`, with the one-line note that a zero boundary absorbs every
   operation and that Phase 2 is what closes it.

**Pass signal:** the process exits 0, block 2 prints three `->` results instead
of a panic message, and block 4 honestly shows the hole that is still open.

**Duration:** under 10 seconds, including the debug build.

**Recorded fallback:** `docs/demos/bint_tour.txt` — real captured stdout,
refreshed whenever `examples/bint_tour.rs` changes; used in a review or an
offline talk where a toolchain is not available.
