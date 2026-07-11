# rs-argument Downstream Gap Completion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the `qubit-argument` API gaps demonstrated by real Qubit downstream validation code before publishing the crate.

**Architecture:** Keep the existing structured `ArgumentError` model and add only three focused extension traits: result path-prefix propagation, `Duration` comparison, and float finiteness. Preserve the existing optional-value validator after tests showed that generalizing its success type weakens type inference. Validate the design through consumer-shaped integration tests while leaving real downstream repositories untouched.

**Tech Stack:** Rust 1.94, edition 2024, standard library only for core features, optional `regex` 1.12, integration tests under `tests/`, shared `.rs-ci` scripts.

## Global Constraints

- Work only in `/home/starfish/working/qubit/rust-common/rs-argument`.
- Do not modify real downstream `rs-*` repositories.
- Use test-driven development: observe each focused test fail before adding production behavior.
- Keep the default feature set empty and add no dependency.
- Preserve values and ownership on success; allocate path prefixes only on failure.
- Keep validated string contents out of structured errors and diagnostics.
- Export every new public API from the crate root and document every function and method in English.
- Keep tests under `tests/` with names following `test_{method}_{scenario}`.
- Do not commit or push unless separately authorized.
- Final verification order is `./align-ci.sh` followed by `./ci-check.sh`.

---

### Task 1: Add failure-only nested path propagation

**Files:**
- Modify: `src/argument/argument_path.rs`
- Modify: `src/argument/argument_error.rs`
- Create: `src/argument/argument_result_ext.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Modify: `tests/argument/argument_path_tests.rs`
- Create: `tests/argument/argument_result_ext_tests.rs`
- Modify: `tests/argument/mod.rs`

**Interfaces:**
- Produces `ArgumentPath::with_prefix`, `ArgumentError::with_path_prefix`, and `ArgumentResultExt::with_path_prefix`.
- Later consumer tests use `ArgumentResultExt` to compose nested HTTP-style paths.

- [ ] Write tests for `retry.max_attempts`, empty prefix/path behavior, transparent `Ok`, and prefixed `Err`.
- [ ] Run `cargo test --test argument_tests argument::argument_path_tests` and
  `cargo test --test argument_tests argument::argument_result_ext_tests`; confirm
  compilation fails because the APIs are missing.
- [ ] Implement the three APIs with `.` composition and error-only allocation.
- [ ] Re-run the focused tests and confirm they pass.

### Task 2: Add structured Duration comparisons

**Files:**
- Modify: `src/argument/argument_value.rs`
- Create: `src/argument/duration_argument.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Modify: `tests/argument/argument_value_tests.rs`
- Create: `tests/argument/duration_argument_tests.rs`
- Modify: `tests/argument/mod.rs`

**Interfaces:**
- Produces `ArgumentValue::Duration(Duration)` and `DurationArgument` for `Duration`.
- Uses existing `ArgumentErrorKind::Comparison` and `ComparisonConstraint`.

- [ ] Write tests proving exact `Duration` capture, display, successful value preservation, positive rejection, and all four comparison constraints.
- [ ] Run the two focused test modules and confirm compilation fails because `DurationArgument` and the value variant are missing.
- [ ] Implement `From<Duration>`, formatting, and the five duration validators.
- [ ] Re-run the focused tests and confirm they pass.

### Task 3: Add float finiteness validation

**Files:**
- Modify: `src/argument/argument_error_kind.rs`
- Modify: `src/argument/argument_error.rs`
- Create: `src/argument/float_argument.rs`
- Modify: `src/argument/mod.rs`
- Modify: `src/lib.rs`
- Create: `tests/argument/float_argument_tests.rs`
- Modify: `tests/argument/argument_error_tests.rs`
- Modify: `tests/argument/mod.rs`

**Interfaces:**
- Produces `FloatArgument::require_finite` for `f32` and `f64`.
- Produces `ArgumentErrorKind::NotFinite { actual: ArgumentValue }` for infinities while retaining `NotANumber` for NaN.

- [ ] Write tests for finite values, positive/negative infinity, NaN, and display output.
- [ ] Run the focused tests and confirm compilation fails because the trait and error variant are missing.
- [ ] Implement the new error kind, display branch, and float-only trait.
- [ ] Re-run the focused tests and confirm they pass.

### Task 4: Add downstream-shaped consumer tests

**Files:**
- Create: `tests/argument/downstream_usage_tests.rs`
- Modify: `tests/argument/mod.rs`

**Interfaces:**
- Consumes `StringArgument`, `NumericArgument`, `OptionArgument`, `DurationArgument`, `FloatArgument`, `ArgumentResultExt`, and `From<ArgumentError>`.

- [ ] Add event-bus topic, thread-pool builder, nested HTTP timeout, and retry-delay scenarios with exact structured assertions.
- [ ] Run `cargo test --test argument_tests argument::downstream_usage_tests` and confirm every consumer scenario passes without `map_err` for domain conversion.
- [ ] Run the complete integration test target to detect API interactions.

### Task 5: Update public documentation

**Files:**
- Modify: `src/lib.rs`
- Modify: `src/argument/mod.rs`
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: `src/argument/README.md`
- Modify: `src/argument/README.zh_CN.md`

**Interfaces:**
- Documents nested path propagation, duration comparisons, and float finiteness.

- [ ] Update crate and module rustdoc with concise examples and API descriptions.
- [ ] Update both README language variants and both argument guides without claiming universal validation coverage.
- [ ] Run doctests for default and all-feature builds.

### Task 6: Verify and review

**Files:**
- Review all files changed by Tasks 1-5.

**Interfaces:**
- Produces a CI-aligned, reviewable working tree without committing or pushing.

- [ ] Run `cargo test --no-default-features` and `cargo test --all-features`.
- [ ] Run `git --no-pager diff --check` and review `git --no-pager diff` for unrelated changes.
- [ ] Run `./align-ci.sh` and inspect any generated changes.
- [ ] Run `./ci-check.sh` and confirm every configured CI step succeeds.
- [ ] Re-read the design requirements and verify every public API and downstream scenario is covered.

### Task 7: Harden the public API before release

**Files:**
- Modify: `src/argument/argument_value.rs`
- Modify: `src/argument/constraint/length_metric.rs`
- Create: `src/argument/sealed.rs`
- Modify: every public validation extension-trait module
- Modify: public documentation

**Interfaces:**
- Keeps validation extension traits usable only for library-supported types.
- Keeps open-ended structured value and metric vocabularies evolvable after 0.4.

- [ ] Add compile-fail doctests for an external trait implementation and exhaustive downstream enum matches; confirm they unexpectedly compile before hardening.
- [ ] Seal `NumericArgument`, `FloatArgument`, `DurationArgument`, `StringArgument`, `CollectionArgument`, `OptionArgument`, and `ArgumentResultExt` with one private marker trait.
- [ ] Mark `ArgumentValue` and `LengthMetric` as `#[non_exhaustive]`.
- [ ] Document the extension boundary and non-exhaustive matching requirement in both language guides.
- [ ] Run focused doctests, `./align-ci.sh`, `./ci-check.sh`, and a clean `cargo publish --dry-run`.
