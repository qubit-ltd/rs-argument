# Option validate_some Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ownership-preserving validation for present `Option<T>` values and document the recommended downstream composition patterns.

**Architecture:** Extend the existing sealed `OptionArgument<T>` trait with a consuming `validate_some` combinator. Keep `validate_if_some` for borrowed validation, preserve `None` unchanged, and propagate the validator's structured error without modification.

**Tech Stack:** Rust 2024, `qubit-argument`, Cargo integration tests, Markdown documentation.

## Global Constraints

- Publishable crate version is `0.4.1`.
- Existing `OptionArgument` APIs remain compatible.
- Validation must not require `T: Clone` or `T: Copy`.
- English and Chinese READMEs show equivalent generic usage examples.
- Run `./align-ci.sh` and `./ci-check.sh` before committing.

---

### Task 1: Add ownership-preserving optional validation

**Files:**
- Modify: `tests/argument/option_argument_tests.rs`
- Modify: `src/argument/option_argument.rs`

**Interfaces:**
- Produces: `OptionArgument::validate_some(self, validator) -> ArgumentResult<Self>` where `validator: FnOnce(T) -> ArgumentResult<T>`.

- [ ] Add tests for `Some` ownership transformation, `None` skipping, single invocation, and structured error propagation.
- [ ] Run the focused tests and confirm compilation fails because `validate_some` is absent.
- [ ] Add the trait method and `Option<T>` implementation using one `match`.
- [ ] Re-run the focused tests and confirm they pass.

### Task 2: Document common composition patterns and release version

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: `src/argument/README.md`
- Modify: `src/argument/README.zh_CN.md`

- [ ] Set the crate version to `0.4.1` and update the lockfile.
- [ ] Add equivalent English and Chinese examples for chaining, optional validation, domain error conversion, nested paths, and custom rules.
- [ ] Describe the distinct borrowed and consuming optional validation methods.

### Task 3: Verify, commit, and synchronize branches

**Files:**
- Verify all changed files.

- [ ] Run `./align-ci.sh`.
- [ ] Run `./ci-check.sh`.
- [ ] Inspect `git diff --check`, the complete diff, and repository status.
- [ ] Create English commits grouped by API/release and documentation intent.
- [ ] Fetch remote state, push the current branch, fast-forward `dev` and `main`, push them, and return to the original branch.
