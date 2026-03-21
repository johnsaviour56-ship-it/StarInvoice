# StarInvoice — GitHub Issues

120 unique, contributor-ready GitHub issues for the StarInvoice project.

---

## Core Escrow Functions (Issues #1–#5)

---

### #1 — [feature] Implement `fund_invoice`

**Labels:** `feature`, `good first issue`, `core`

**Description:**
The `fund_invoice` function is currently stubbed with a `todo!()` macro. This is the second step in the escrow flow and must be implemented before any downstream functions can work.

**Acceptance Criteria:**
- Verify the caller is the invoice's `client` field using `require_auth()`
- Load the invoice by `invoice_id` and assert its status is `Pending`
- Transfer `amount` tokens from the client to the contract address using the Soroban token interface
- Update the invoice status to `Funded`
- Emit the `fund_invoice` event via `events.rs`
- Add at least one passing test covering the happy path

**References:** `contracts/invoice/src/lib.rs` — `fund_invoice`

---

### #2 — [feature] Implement `mark_delivered`

**Labels:** `feature`, `good first issue`, `core`

**Description:**
The `mark_delivered` function is stubbed. The freelancer must be able to signal that work is complete so the client can review and approve.

**Acceptance Criteria:**
- Verify the caller is the invoice's `freelancer` using `require_auth()`
- Assert invoice status is `Funded`
- Update status to `Delivered`
- Emit the `mark_delivered` event
- Add tests for both the happy path and invalid-state transitions

**References:** `contracts/invoice/src/lib.rs` — `mark_delivered`

---

### #3 — [feature] Implement `approve_payment`

**Labels:** `feature`, `good first issue`, `core`

**Description:**
The `approve_payment` function is stubbed. The client must be able to approve delivered work before funds are released.

**Acceptance Criteria:**
- Verify the caller is the invoice's `client` using `require_auth()`
- Assert invoice status is `Delivered`
- Update status to `Approved`
- Emit the `approve_payment` event
- Add tests covering happy path and wrong-caller scenarios

**References:** `contracts/invoice/src/lib.rs` — `approve_payment`

---

### #4 — [feature] Implement `release_payment`

**Labels:** `feature`, `good first issue`, `core`

**Description:**
The `release_payment` function is stubbed. This is the final step — transferring escrowed funds to the freelancer.

**Acceptance Criteria:**
- Assert invoice status is `Approved`
- Transfer escrowed tokens from the contract to the `freelancer` address
- Update status to `Completed`
- Emit the `release_payment` event
- Add tests verifying the token balance changes correctly

**References:** `contracts/invoice/src/lib.rs` — `release_payment`

---

### #5 — [feature] Add `Disputed` and `Cancelled` `InvoiceStatus` variants

**Labels:** `feature`, `good first issue`, `core`

**Description:**
The `InvoiceStatus` enum has a TODO comment suggesting `Disputed` and `Cancelled` states. These are important for real-world escrow flows.

**Acceptance Criteria:**
- Add `Disputed` variant — reachable from `Funded` or `Delivered`
- Add `Cancelled` variant — reachable from `Pending` or `Funded`
- Document valid state transitions in a comment or diagram
- Update any match expressions that need exhaustive handling

**References:** `contracts/invoice/src/storage.rs` — `InvoiceStatus`

---

## Invoice Data Model (Issues #6–#15)

---

### #6 — [feature] Add `token` field to `Invoice` for multi-token support

**Labels:** `feature`, `enhancement`

**Description:**
Currently the `Invoice` struct has no `token` field, meaning the contract implicitly assumes a single token. Adding a `token: Address` field enables clients and freelancers to agree on any Stellar asset.

**Acceptance Criteria:**
- Add `token: Address` to the `Invoice` struct
- Pass `token` as a parameter to `create_invoice`
- Use `token` when performing transfers in `fund_invoice` and `release_payment`
- Update tests to supply a mock token address

**References:** `contracts/invoice/src/storage.rs` — `Invoice` TODO comment

---

### #7 — [feature] Add `deadline` field to `Invoice`

**Labels:** `feature`, `enhancement`

**Description:**
Invoices should support an optional expiry/deadline so clients and freelancers have time-bound agreements. Soroban provides `env.ledger().timestamp()` for on-chain time.

**Acceptance Criteria:**
- Add `deadline: u64` (Unix timestamp) to `Invoice`
- Pass `deadline` as a parameter to `create_invoice`
- In `fund_invoice`, reject funding if `env.ledger().timestamp() > deadline`
- Add tests for expired and non-expired invoices

**References:** `contracts/invoice/src/storage.rs` — `Invoice` TODO comment

---

### #8 — [feature] Add `created_at` timestamp to `Invoice`

**Labels:** `feature`, `enhancement`

**Description:**
Recording when an invoice was created helps with auditing and off-chain indexing. Use `env.ledger().timestamp()` at creation time.

**Acceptance Criteria:**
- Add `created_at: u64` to `Invoice`
- Populate it in `create_invoice` using `env.ledger().timestamp()`
- No user-supplied input — always set by the contract

---

### #9 — [feature] Add `title` field to `Invoice`

**Labels:** `feature`, `enhancement`

**Description:**
The current `description` field serves as the only human-readable label. A short `title` field would improve UX for frontends and indexers.

**Acceptance Criteria:**
- Add `title: String` to `Invoice`
- Accept `title` as a parameter in `create_invoice`
- Keep `description` for longer free-text content

---

### #10 — [feature] Add `invoice_count` view function

**Labels:** `feature`, `enhancement`

**Description:**
There is no public way to query how many invoices exist. A `invoice_count` function would help frontends and indexers.

**Acceptance Criteria:**
- Add `pub fn invoice_count(env: Env) -> u64` to `InvoiceContract`
- Return the current value of `DataKey::InvoiceCount`
- Add a test verifying the count increments correctly

---

### #11 — [feature] Add `get_invoice` view function to contract

**Labels:** `feature`, `enhancement`

**Description:**
`storage::get_invoice` is a private helper. There is no public contract function to fetch an invoice by ID. Frontends and other contracts need this.

**Acceptance Criteria:**
- Add `pub fn get_invoice(env: Env, invoice_id: u64) -> Invoice` to `InvoiceContract`
- Return the full `Invoice` struct
- Add a test that creates an invoice and retrieves it via the public function

---

### #12 — [feature] Return graceful error instead of `panic` when invoice not found

**Labels:** `feature`, `refactor`, `error-handling`

**Description:**
`storage::get_invoice` currently calls `.expect("Invoice not found")` which panics. Soroban contracts should use `Result` or a custom error enum for graceful error handling.

**Acceptance Criteria:**
- Define a `ContractError` enum with at least `InvoiceNotFound` variant
- Change `get_invoice` to return `Result<Invoice, ContractError>`
- Propagate errors up through all callers
- Add a test that verifies the error is returned for a missing ID

---

### #13 — [feature] Add `cancel_invoice` function

**Labels:** `feature`, `enhancement`

**Description:**
There is no way to cancel an invoice once created. A `cancel_invoice` function would allow the freelancer or client to void a `Pending` invoice.

**Acceptance Criteria:**
- Add `pub fn cancel_invoice(env: Env, invoice_id: u64)`
- Allow cancellation only from `Pending` status
- Require auth from either `freelancer` or `client`
- Update status to `Cancelled` (requires issue #5)
- Emit a `cancelled` event
- Add tests

---

### #14 — [feature] Add `dispute_invoice` function

**Labels:** `feature`, `enhancement`

**Description:**
Either party should be able to raise a dispute if there is a disagreement. This requires the `Disputed` status from issue #5.

**Acceptance Criteria:**
- Add `pub fn dispute_invoice(env: Env, invoice_id: u64)`
- Allow disputes from `Funded` or `Delivered` status
- Require auth from either `freelancer` or `client`
- Update status to `Disputed`
- Emit a `disputed` event
- Add tests

---

### #15 — [feature] Add `refund_client` function for cancelled/disputed invoices

**Labels:** `feature`, `enhancement`

**Description:**
When an invoice is cancelled after funding, or a dispute is resolved in the client's favor, escrowed funds should be returnable to the client.

**Acceptance Criteria:**
- Add `pub fn refund_client(env: Env, invoice_id: u64)`
- Only callable when status is `Cancelled` (post-funding) or `Disputed`
- Transfer escrowed tokens back to `client`
- Update status to `Completed` or a new `Refunded` variant
- Emit a `refunded` event
- Add tests

---

## Events (Issues #16–#22)

---

### #16 — [feature] Add `fund_invoice` event emitter in `events.rs`

**Labels:** `feature`, `good first issue`

**Description:**
`events.rs` has a TODO to add emitters for all state transitions. This issue covers the `fund_invoice` event.

**Acceptance Criteria:**
- Add `pub fn invoice_funded(env: &Env, invoice_id: u64, client: &Address, amount: i128)` to `events.rs`
- Publish with topic `("INVOICE", "funded")`
- Call it from `fund_invoice` in `lib.rs`

**References:** `contracts/invoice/src/events.rs` — TODO comment

---

### #17 — [feature] Add `mark_delivered` event emitter in `events.rs`

**Labels:** `feature`, `good first issue`

**Description:**
Add the event emitter for the `mark_delivered` state transition.

**Acceptance Criteria:**
- Add `pub fn invoice_delivered(env: &Env, invoice_id: u64, freelancer: &Address)` to `events.rs`
- Publish with topic `("INVOICE", "delivered")`
- Call it from `mark_delivered` in `lib.rs`

---

### #18 — [feature] Add `approve_payment` event emitter in `events.rs`

**Labels:** `feature`, `good first issue`

**Description:**
Add the event emitter for the `approve_payment` state transition.

**Acceptance Criteria:**
- Add `pub fn invoice_approved(env: &Env, invoice_id: u64, client: &Address)` to `events.rs`
- Publish with topic `("INVOICE", "approved")`
- Call it from `approve_payment` in `lib.rs`

---

### #19 — [feature] Add `release_payment` event emitter in `events.rs`

**Labels:** `feature`, `good first issue`

**Description:**
Add the event emitter for the `release_payment` state transition.

**Acceptance Criteria:**
- Add `pub fn invoice_released(env: &Env, invoice_id: u64, freelancer: &Address, amount: i128)` to `events.rs`
- Publish with topic `("INVOICE", "released")`
- Call it from `release_payment` in `lib.rs`

---

### #20 — [feature] Add `cancel_invoice` event emitter in `events.rs`

**Labels:** `feature`, `enhancement`

**Description:**
Add an event emitter for invoice cancellation (depends on issue #13).

**Acceptance Criteria:**
- Add `pub fn invoice_cancelled(env: &Env, invoice_id: u64, cancelled_by: &Address)`
- Publish with topic `("INVOICE", "cancelled")`

---

### #21 — [feature] Add `dispute_invoice` event emitter in `events.rs`

**Labels:** `feature`, `enhancement`

**Description:**
Add an event emitter for invoice disputes (depends on issue #14).

**Acceptance Criteria:**
- Add `pub fn invoice_disputed(env: &Env, invoice_id: u64, raised_by: &Address)`
- Publish with topic `("INVOICE", "disputed")`

---

### #22 — [feature] Add `refund_client` event emitter in `events.rs`

**Labels:** `feature`, `enhancement`

**Description:**
Add an event emitter for client refunds (depends on issue #15).

**Acceptance Criteria:**
- Add `pub fn invoice_refunded(env: &Env, invoice_id: u64, client: &Address, amount: i128)`
- Publish with topic `("INVOICE", "refunded")`

---

## Storage & State Management (Issues #23–#32)

---

### #23 — [refactor] Use `persistent` storage consistently for all invoice data

**Labels:** `refactor`

**Description:**
`InvoiceCount` uses `instance` storage while `Invoice` records use `persistent` storage. This inconsistency could cause issues if the contract instance is upgraded. Evaluate and standardize storage type usage.

**Acceptance Criteria:**
- Document the rationale for each storage type used (`instance` vs `persistent` vs `temporary`)
- Migrate `InvoiceCount` to `persistent` if appropriate
- Add a comment explaining the choice

---

### #24 — [feature] Add storage TTL extension for invoice entries

**Labels:** `feature`, `enhancement`

**Description:**
Soroban persistent storage entries expire unless their TTL is extended. The contract should extend TTL on every read/write to prevent invoice data from being evicted.

**Acceptance Criteria:**
- Call `env.storage().persistent().extend_ttl(key, threshold, extend_to)` after every `save_invoice` and `get_invoice`
- Choose sensible TTL values and document them as constants
- Add a test that verifies TTL extension is called

---

### #25 — [feature] Add `has_invoice` helper to `storage.rs`

**Labels:** `feature`, `refactor`

**Description:**
Currently there is no way to check if an invoice exists without panicking. A `has_invoice` helper would allow safe existence checks.

**Acceptance Criteria:**
- Add `pub fn has_invoice(env: &Env, invoice_id: u64) -> bool` to `storage.rs`
- Use it in `get_invoice` to return a `Result` instead of panicking (see issue #12)

---

### #26 — [refactor] Extract invoice status transition validation into `storage.rs`

**Labels:** `refactor`

**Description:**
Status transition checks (e.g., "must be Funded to mark delivered") are currently inline in each function. Centralizing this logic reduces duplication and makes transitions easier to audit.

**Acceptance Criteria:**
- Add a `validate_transition(from: &InvoiceStatus, to: &InvoiceStatus) -> bool` function
- Use it in all state-changing contract functions
- Add unit tests for valid and invalid transitions

---

### #27 — [feature] Add `update_invoice_status` helper to `storage.rs`

**Labels:** `refactor`, `feature`

**Description:**
Each function that changes invoice status loads, mutates, and saves the invoice inline. A dedicated helper would reduce boilerplate.

**Acceptance Criteria:**
- Add `pub fn update_invoice_status(env: &Env, invoice_id: u64, new_status: InvoiceStatus)`
- Use it in all state-changing functions
- Ensure it calls `save_invoice` internally

---

### #28 — [feature] Add `DataKey::InvoicesByFreelancer` index

**Labels:** `feature`, `enhancement`

**Description:**
There is no way to look up all invoices for a given freelancer. An index mapping `Address -> Vec<u64>` would enable this.

**Acceptance Criteria:**
- Add `DataKey::InvoicesByFreelancer(Address)` variant
- Update `save_invoice` to append the invoice ID to the freelancer's list
- Add `pub fn get_invoices_by_freelancer(env: &Env, freelancer: &Address) -> Vec<u64>`
- Add tests

---

### #29 — [feature] Add `DataKey::InvoicesByClient` index

**Labels:** `feature`, `enhancement`

**Description:**
Similar to issue #28, add an index for looking up invoices by client address.

**Acceptance Criteria:**
- Add `DataKey::InvoicesByClient(Address)` variant
- Update `save_invoice` to append the invoice ID to the client's list
- Add `pub fn get_invoices_by_client(env: &Env, client: &Address) -> Vec<u64>`
- Add tests

---

### #30 — [bug] `next_invoice_id` is not atomic — potential race condition under concurrent calls

**Labels:** `bug`, `security`

**Description:**
`next_invoice_id` reads the count, increments it, and writes it back in two separate storage operations. While Soroban transactions are atomic per-transaction, document whether this is safe and add a comment explaining the guarantee.

**Acceptance Criteria:**
- Add a comment in `next_invoice_id` explaining Soroban's transaction atomicity model
- If any risk exists, refactor to a single atomic operation
- Add a test that creates multiple invoices and verifies unique IDs

---

### #31 — [feature] Add `amount` validation in `create_invoice`

**Labels:** `feature`, `bug`

**Description:**
`create_invoice` accepts any `i128` value for `amount`, including zero and negative numbers. These should be rejected.

**Acceptance Criteria:**
- Assert `amount > 0` at the start of `create_invoice`
- Return or panic with a descriptive error for invalid amounts
- Add tests for zero and negative amounts

---

### #32 — [feature] Add `description` length validation in `create_invoice`

**Labels:** `feature`, `enhancement`

**Description:**
There is no limit on the `description` field length. Very long descriptions waste on-chain storage. Add a maximum length check.

**Acceptance Criteria:**
- Define a `MAX_DESCRIPTION_LEN` constant (e.g., 256 bytes)
- Validate `description.len() <= MAX_DESCRIPTION_LEN` in `create_invoice`
- Add tests for boundary values

---

## Testing (Issues #33–#45)

---

### #33 — [test] Add tests for `fund_invoice` happy path

**Labels:** `test`, `good first issue`

**Description:**
Once `fund_invoice` is implemented (issue #1), add a comprehensive test covering the happy path: client funds a pending invoice and status becomes `Funded`.

**Acceptance Criteria:**
- Mock token contract and client auth
- Assert invoice status is `Funded` after the call
- Assert token balances changed correctly

---

### #34 — [test] Add tests for `mark_delivered` happy path

**Labels:** `test`, `good first issue`

**Description:**
Add tests for `mark_delivered` once implemented (issue #2).

**Acceptance Criteria:**
- Start from a `Funded` invoice
- Assert status becomes `Delivered` after the call
- Assert the correct event was emitted

---

### #35 — [test] Add tests for `approve_payment` happy path

**Labels:** `test`, `good first issue`

**Description:**
Add tests for `approve_payment` once implemented (issue #3).

**Acceptance Criteria:**
- Start from a `Delivered` invoice
- Assert status becomes `Approved`
- Assert the correct event was emitted

---

### #36 — [test] Add tests for `release_payment` happy path

**Labels:** `test`, `good first issue`

**Description:**
Add tests for `release_payment` once implemented (issue #4).

**Acceptance Criteria:**
- Start from an `Approved` invoice
- Assert status becomes `Completed`
- Assert freelancer received the correct token amount

---

### #37 — [test] Add negative tests for wrong-caller authorization

**Labels:** `test`, `security`

**Description:**
Each function that requires auth should be tested with the wrong caller to ensure unauthorized access is rejected.

**Acceptance Criteria:**
- Test `fund_invoice` called by the freelancer (should fail)
- Test `mark_delivered` called by the client (should fail)
- Test `approve_payment` called by the freelancer (should fail)
- Each test should assert the call panics or returns an auth error

---

### #38 — [test] Add tests for invalid status transitions

**Labels:** `test`

**Description:**
Each state-changing function should reject calls when the invoice is in the wrong state.

**Acceptance Criteria:**
- Test `fund_invoice` on an already-`Funded` invoice
- Test `mark_delivered` on a `Pending` invoice
- Test `approve_payment` on a `Funded` (not yet delivered) invoice
- Test `release_payment` on a `Delivered` (not yet approved) invoice

---

### #39 — [test] Add end-to-end test covering the full escrow flow

**Labels:** `test`

**Description:**
Add a single integration test that walks through the entire flow: `create_invoice → fund_invoice → mark_delivered → approve_payment → release_payment`.

**Acceptance Criteria:**
- Single test function covering all five steps
- Assert status at each step
- Assert final token balances

---

### #40 — [test] Add test for `create_invoice` with duplicate IDs (regression)

**Labels:** `test`, `bug`

**Description:**
Verify that creating multiple invoices always produces unique, incrementing IDs.

**Acceptance Criteria:**
- Create 10 invoices in a loop
- Assert each has a unique ID from 0 to 9

---

### #41 — [test] Add test for `get_invoice` with non-existent ID

**Labels:** `test`, `bug`

**Description:**
Verify that fetching a non-existent invoice ID returns an error rather than panicking unexpectedly (depends on issue #12).

**Acceptance Criteria:**
- Call `get_invoice` with an ID that was never created
- Assert the expected error variant is returned

---

### #42 — [test] Add test for `cancel_invoice` (depends on #13)

**Labels:** `test`

**Description:**
Add tests for the `cancel_invoice` function once implemented.

**Acceptance Criteria:**
- Test cancellation from `Pending` status by freelancer
- Test cancellation from `Pending` status by client
- Test that cancellation from `Funded` is rejected (or allowed, per spec)

---

### #43 — [test] Add test for `dispute_invoice` (depends on #14)

**Labels:** `test`

**Description:**
Add tests for the `dispute_invoice` function once implemented.

**Acceptance Criteria:**
- Test dispute from `Funded` and `Delivered` states
- Test that dispute from `Pending` is rejected

---

### #44 — [test] Add test for expired invoice rejection (depends on #7)

**Labels:** `test`

**Description:**
Once the `deadline` field is added, verify that `fund_invoice` rejects funding after the deadline.

**Acceptance Criteria:**
- Create an invoice with a past deadline
- Assert `fund_invoice` fails with an expiry error

---

### #45 — [test] Add test for zero and negative `amount` in `create_invoice` (depends on #31)

**Labels:** `test`

**Description:**
Verify that `create_invoice` rejects invalid amounts.

**Acceptance Criteria:**
- Test with `amount = 0` — should fail
- Test with `amount = -100` — should fail
- Test with `amount = 1` — should succeed

---

## Documentation (Issues #46–#58)

---

### #46 — [docs] Update README status table when `fund_invoice` is implemented

**Labels:** `docs`, `good first issue`

**Description:**
The README has a status table showing which functions are implemented. Once `fund_invoice` is complete, update the table.

**Acceptance Criteria:**
- Change `fund_invoice` row from `🚧 TODO` to `✅ Implemented`
- This issue should be closed as part of the PR for issue #1

---

### #47 — [docs] Update README status table when `mark_delivered` is implemented

**Labels:** `docs`, `good first issue`

**Description:**
Update the README status table once `mark_delivered` is complete (issue #2).

**Acceptance Criteria:**
- Change `mark_delivered` row from `🚧 TODO` to `✅ Implemented`

---

### #48 — [docs] Update README status table when `approve_payment` is implemented

**Labels:** `docs`, `good first issue`

**Description:**
Update the README status table once `approve_payment` is complete (issue #3).

---

### #49 — [docs] Update README status table when `release_payment` is implemented

**Labels:** `docs`, `good first issue`

**Description:**
Update the README status table once `release_payment` is complete (issue #4).

---

### #50 — [docs] Add state machine diagram to README

**Labels:** `docs`, `enhancement`

**Description:**
The contract flow is described as a linear sequence, but the actual state machine (including `Disputed`, `Cancelled`) is more complex. A diagram would help contributors understand valid transitions.

**Acceptance Criteria:**
- Add a Mermaid state diagram to README showing all `InvoiceStatus` transitions
- Include `Disputed` and `Cancelled` states (depends on issue #5)

---

### #51 — [docs] Add inline doc comments to all public functions in `lib.rs`

**Labels:** `docs`, `good first issue`

**Description:**
`create_invoice` has a doc comment but the stub functions only have TODO comments. Once implemented, all public functions should have proper `///` doc comments.

**Acceptance Criteria:**
- Add `///` doc comments to `fund_invoice`, `mark_delivered`, `approve_payment`, `release_payment`
- Document parameters, return values, and error conditions
- Run `cargo doc` and verify no warnings

---

### #52 — [docs] Add inline doc comments to all functions in `storage.rs`

**Labels:** `docs`, `good first issue`

**Description:**
`storage.rs` functions lack doc comments. Add `///` comments to all public functions.

**Acceptance Criteria:**
- Document `next_invoice_id`, `save_invoice`, `get_invoice`
- Document the `Invoice` struct fields
- Document the `InvoiceStatus` variants

---

### #53 — [docs] Add inline doc comments to all functions in `events.rs`

**Labels:** `docs`, `good first issue`

**Description:**
`events.rs` has one function with no doc comment. Add `///` comments to all event emitters.

**Acceptance Criteria:**
- Document `invoice_created` and all future event functions
- Describe the event topic and data payload

---

### #54 — [docs] Add architecture overview section to README

**Labels:** `docs`, `enhancement`

**Description:**
The README describes the contract flow but not the overall architecture (Soroban, Stellar token interface, storage model). A brief architecture section would help new contributors.

**Acceptance Criteria:**
- Add an "Architecture" section to README
- Explain the role of `lib.rs`, `storage.rs`, and `events.rs`
- Briefly describe how Soroban token transfers work

---

### #55 — [docs] Add `SECURITY.md` with responsible disclosure policy

**Labels:** `docs`, `security`

**Description:**
Open-source smart contract projects should have a security policy so researchers know how to report vulnerabilities.

**Acceptance Criteria:**
- Create `SECURITY.md` at the repo root
- Include contact method for reporting vulnerabilities
- Include scope (what is in/out of scope)
- Reference GitHub's security advisory feature

---

### #56 — [docs] Add `CHANGELOG.md`

**Labels:** `docs`

**Description:**
A changelog helps users and contributors track what changed between versions.

**Acceptance Criteria:**
- Create `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com) format
- Add an `[Unreleased]` section listing current TODOs
- Add a `[0.1.0]` section for the initial `create_invoice` implementation

---

### #57 — [docs] Add code of conduct (`CODE_OF_CONDUCT.md`)

**Labels:** `docs`, `community`

**Description:**
A code of conduct sets expectations for community behavior and is standard for open-source projects.

**Acceptance Criteria:**
- Add `CODE_OF_CONDUCT.md` using the Contributor Covenant template
- Reference it from `CONTRIBUTING.md`

---

### #58 — [docs] Document the Soroban token interface used for transfers

**Labels:** `docs`, `enhancement`

**Description:**
Contributors implementing `fund_invoice` and `release_payment` need to understand how to call the Soroban token interface. Add a section or comment explaining this.

**Acceptance Criteria:**
- Add a comment block in `lib.rs` or a `docs/token-interface.md` file
- Show a minimal example of calling `token::Client::new(&env, &token_address).transfer(...)`

---

## Developer Experience & Tooling (Issues #59–#72)

---

### #59 — [tooling] Add `Makefile` with common dev commands

**Labels:** `tooling`, `good first issue`

**Description:**
Contributors currently need to remember multiple `cargo` commands. A `Makefile` would simplify the workflow.

**Acceptance Criteria:**
- Add `Makefile` with targets: `build`, `test`, `fmt`, `lint`, `clean`
- `build` compiles to `wasm32-unknown-unknown --release`
- `lint` runs `cargo clippy -- -D warnings`
- Document targets in `CONTRIBUTING.md`

---

### #60 — [tooling] Add GitHub Actions CI workflow

**Labels:** `tooling`, `ci`

**Description:**
There is no CI pipeline. A GitHub Actions workflow should run on every PR to catch regressions.

**Acceptance Criteria:**
- Create `.github/workflows/ci.yml`
- Run `cargo fmt --check`, `cargo clippy`, and `cargo test` on push and PR
- Target `ubuntu-latest` with the stable Rust toolchain
- Cache `~/.cargo` for faster runs

---

### #61 — [tooling] Add `wasm32-unknown-unknown` target to CI

**Labels:** `tooling`, `ci`

**Description:**
The contract must compile to WASM. CI should verify this, not just native compilation.

**Acceptance Criteria:**
- Add a CI step that runs `cargo build --target wasm32-unknown-unknown --release`
- Ensure the `wasm32-unknown-unknown` target is installed in the CI environment

---

### #62 — [tooling] Add `.github/ISSUE_TEMPLATE` for bug reports

**Labels:** `tooling`, `community`

**Description:**
Structured issue templates help contributors file better bug reports.

**Acceptance Criteria:**
- Create `.github/ISSUE_TEMPLATE/bug_report.md`
- Include fields: description, steps to reproduce, expected behavior, actual behavior, environment

---

### #63 — [tooling] Add `.github/ISSUE_TEMPLATE` for feature requests

**Labels:** `tooling`, `community`

**Description:**
Add a feature request issue template.

**Acceptance Criteria:**
- Create `.github/ISSUE_TEMPLATE/feature_request.md`
- Include fields: problem statement, proposed solution, alternatives considered

---

### #64 — [tooling] Add `.github/pull_request_template.md`

**Labels:** `tooling`, `community`

**Description:**
A PR template ensures contributors include necessary information when opening pull requests.

**Acceptance Criteria:**
- Create `.github/pull_request_template.md`
- Include: description, related issue, testing done, checklist (fmt, clippy, tests pass)

---

### #65 — [tooling] Add `rustfmt.toml` configuration file

**Labels:** `tooling`

**Description:**
A `rustfmt.toml` ensures consistent formatting across all contributors' environments.

**Acceptance Criteria:**
- Create `rustfmt.toml` at the repo root
- Set at minimum: `edition = "2021"`, `max_width = 100`
- Document in `CONTRIBUTING.md` that `cargo fmt` must pass before merging

---

### #66 — [tooling] Add `.clippy.toml` or `clippy.toml` with project lint rules

**Labels:** `tooling`

**Description:**
Configuring Clippy ensures consistent lint rules across contributors.

**Acceptance Criteria:**
- Create `clippy.toml` or add `[lints.clippy]` to `Cargo.toml`
- Enable `pedantic` lints or document which lints are explicitly allowed
- Ensure CI runs `cargo clippy -- -D warnings`

---

### #67 — [tooling] Add `.editorconfig` for consistent editor settings

**Labels:** `tooling`, `good first issue`

**Description:**
An `.editorconfig` file ensures consistent indentation and line endings across editors.

**Acceptance Criteria:**
- Create `.editorconfig` at the repo root
- Set: `indent_style = space`, `indent_size = 4`, `end_of_line = lf`, `charset = utf-8`

---

### #68 — [tooling] Add `cargo-audit` step to CI for dependency vulnerability scanning

**Labels:** `tooling`, `security`, `ci`

**Description:**
`cargo-audit` checks dependencies against the RustSec advisory database. This should run in CI.

**Acceptance Criteria:**
- Add a CI job that installs and runs `cargo audit`
- Fail the build on any high-severity advisories
- Document how to run it locally in `CONTRIBUTING.md`

---

### #69 — [tooling] Pin Rust toolchain version with `rust-toolchain.toml`

**Labels:** `tooling`

**Description:**
Without a pinned toolchain, different contributors may use different Rust versions, causing inconsistent behavior.

**Acceptance Criteria:**
- Create `rust-toolchain.toml` specifying the stable channel and version
- Add `wasm32-unknown-unknown` as a required target component

---

### #70 — [tooling] Add `cargo-deny` for license and dependency policy enforcement

**Labels:** `tooling`, `security`

**Description:**
`cargo-deny` can enforce allowed licenses and ban specific crates. Useful for a project that may be used in production.

**Acceptance Criteria:**
- Add `deny.toml` configuration
- Configure allowed licenses (e.g., MIT, Apache-2.0)
- Add a CI step running `cargo deny check`

---

### #71 — [tooling] Add GitHub Actions release workflow to publish WASM artifact

**Labels:** `tooling`, `ci`

**Description:**
When a version tag is pushed, CI should build the WASM artifact and attach it to a GitHub Release.

**Acceptance Criteria:**
- Create `.github/workflows/release.yml`
- Trigger on `v*` tags
- Build `--target wasm32-unknown-unknown --release`
- Upload the `.wasm` file as a release asset

---

### #72 — [tooling] Add `soroban contract optimize` step to build process

**Labels:** `tooling`, `enhancement`

**Description:**
The Soroban CLI includes a `contract optimize` command that shrinks WASM binary size. This should be part of the build process.

**Acceptance Criteria:**
- Add `soroban contract optimize` to the `Makefile` `build` target
- Document the command in README under "Build"
- Add it to the CI release workflow (issue #71)

---

## Security & Access Control (Issues #73–#83)

---

### #73 — [security] Verify `fund_invoice` cannot be called twice on the same invoice

**Labels:** `security`, `bug`

**Description:**
If `fund_invoice` does not strictly check the `Pending` status, a client could fund the same invoice twice, locking double the tokens in escrow.

**Acceptance Criteria:**
- Add an explicit status check at the start of `fund_invoice`
- Add a test that calls `fund_invoice` twice and asserts the second call fails

---

### #74 — [security] Verify `release_payment` cannot be called before `approve_payment`

**Labels:** `security`, `bug`

**Description:**
`release_payment` must only execute when status is `Approved`. Verify this check is enforced and cannot be bypassed.

**Acceptance Criteria:**
- Add a test calling `release_payment` on a `Delivered` (not yet approved) invoice
- Assert the call fails with the correct error

---

### #75 — [security] Add reentrancy analysis comment for token transfer calls

**Labels:** `security`, `docs`

**Description:**
Soroban's execution model prevents traditional reentrancy, but contributors should understand why. Add a comment in `fund_invoice` and `release_payment` explaining Soroban's reentrancy guarantees.

**Acceptance Criteria:**
- Add a `// SAFETY:` comment near each token transfer call
- Reference the Soroban documentation on cross-contract call semantics

---

### #76 — [security] Ensure `client` address in `create_invoice` is not the same as `freelancer`

**Labels:** `security`, `bug`

**Description:**
Creating an invoice where `client == freelancer` is nonsensical and could lead to unexpected behavior. Add a validation check.

**Acceptance Criteria:**
- Assert `freelancer != client` in `create_invoice`
- Add a test verifying this check

---

### #77 — [security] Add overflow check for `amount` in `fund_invoice`

**Labels:** `security`, `bug`

**Description:**
`amount` is `i128`. While Soroban enables overflow checks in release builds (via `overflow-checks = true` in `Cargo.toml`), document this explicitly and add a test with a very large amount.

**Acceptance Criteria:**
- Add a comment referencing `overflow-checks = true` in `Cargo.toml`
- Add a test with `amount = i128::MAX` to verify behavior

---

### #78 — [security] Audit all `require_auth` call sites

**Labels:** `security`

**Description:**
Every function that modifies state on behalf of a user must call `require_auth()`. Audit all functions and document which address is being authenticated and why.

**Acceptance Criteria:**
- Review all state-changing functions
- Add a comment above each `require_auth()` call explaining which role is being verified
- Open follow-up issues for any missing auth checks

---

### #79 — [security] Prevent `create_invoice` from being called with a zero `client` address

**Labels:** `security`, `bug`

**Description:**
Soroban `Address` types cannot be zero in practice, but document this assumption and add a note for future contributors.

**Acceptance Criteria:**
- Add a comment in `create_invoice` noting that `Address` is always a valid account
- Reference Soroban's address validation guarantees

---

### #80 — [security] Add a maximum `amount` cap to prevent unreasonably large invoices

**Labels:** `security`, `enhancement`

**Description:**
Very large invoice amounts could be used to lock significant funds. Consider adding a configurable maximum.

**Acceptance Criteria:**
- Define `MAX_INVOICE_AMOUNT: i128` constant
- Validate `amount <= MAX_INVOICE_AMOUNT` in `create_invoice`
- Add tests for boundary values

---

### #81 — [security] Ensure token address in `fund_invoice` matches the invoice's `token` field (depends on #6)

**Labels:** `security`, `bug`

**Description:**
Once multi-token support is added (issue #6), `fund_invoice` must verify the token being transferred matches the token recorded on the invoice.

**Acceptance Criteria:**
- Add a check that the token used in the transfer matches `invoice.token`
- Add a test attempting to fund with a different token

---

### #82 — [security] Add admin/arbitrator role for dispute resolution (depends on #14)

**Labels:** `security`, `feature`

**Description:**
When a dispute is raised, there needs to be a mechanism for resolution. An admin or arbitrator address should be able to resolve disputes.

**Acceptance Criteria:**
- Add an `admin: Address` field stored in contract instance storage
- Add `set_admin` function callable only by the current admin
- Add `resolve_dispute(invoice_id, winner: Address)` callable only by admin
- Add tests

---

### #83 — [security] Add `initialize` function to set contract admin

**Labels:** `security`, `feature`

**Description:**
The contract has no initialization function. An `initialize` function should set the admin address and prevent re-initialization.

**Acceptance Criteria:**
- Add `pub fn initialize(env: Env, admin: Address)`
- Store admin in instance storage
- Panic if already initialized
- Add tests for initialization and re-initialization attempt

---

## Refactoring & Code Quality (Issues #84–#96)

---

### #84 — [refactor] Replace `todo!()` macros with proper `unimplemented!()` or custom error

**Labels:** `refactor`

**Description:**
`todo!()` and `unimplemented!()` both panic, but `todo!()` signals "not yet done" while `unimplemented!()` signals "not planned." Once functions are implemented, ensure no `todo!()` macros remain.

**Acceptance Criteria:**
- Search for all `todo!()` calls in the codebase
- Replace each with a proper implementation or a `ContractError` (see issue #12)
- Add a CI lint step that fails if any `todo!()` remains in non-test code

---

### #85 — [refactor] Move contract constants to a dedicated `constants.rs` module

**Labels:** `refactor`

**Description:**
As constants are added (max amount, max description length, TTL values), they should live in a dedicated module rather than scattered across files.

**Acceptance Criteria:**
- Create `contracts/invoice/src/constants.rs`
- Move all constants there
- Import them where needed

---

### #86 — [refactor] Split `lib.rs` into separate modules per function group

**Labels:** `refactor`

**Description:**
As the contract grows, `lib.rs` will become large. Consider splitting into logical modules (e.g., `escrow.rs`, `views.rs`).

**Acceptance Criteria:**
- Evaluate whether splitting is warranted after all core functions are implemented
- If yes, create separate files and re-export from `lib.rs`
- Ensure all tests still pass

---

### #87 — [refactor] Add `#[allow(dead_code)]` removal pass after all functions are implemented

**Labels:** `refactor`

**Description:**
The stub functions use `_env` and `_invoice_id` prefixed with `_` to suppress dead code warnings. Once implemented, remove the underscores.

**Acceptance Criteria:**
- After each function is implemented, rename `_env` → `env` and `_invoice_id` → `invoice_id`
- Ensure no unused variable warnings remain

---

### #88 — [refactor] Use `soroban_sdk::panic_with_error!` instead of `panic!` for contract errors

**Labels:** `refactor`, `best-practice`

**Description:**
Soroban provides `panic_with_error!` which encodes error codes into the contract's return value, making errors inspectable off-chain. Replace raw `panic!` calls.

**Acceptance Criteria:**
- Define a `#[contracterror]` enum
- Replace all `panic!` and `.expect()` calls with `panic_with_error!`
- Add tests that verify the correct error code is returned

---

### #89 — [refactor] Add `#[contractmeta]` to document contract version and description

**Labels:** `refactor`, `best-practice`

**Description:**
Soroban supports `contractmeta!` for embedding metadata in the WASM binary. This helps tools and explorers identify the contract.

**Acceptance Criteria:**
- Add `soroban_sdk::contractmeta!(key = "Description", val = "StarInvoice escrow contract")`
- Add `soroban_sdk::contractmeta!(key = "Version", val = "0.1.0")`

---

### #90 — [refactor] Remove unused `pub use storage::Invoice` re-export if not needed externally

**Labels:** `refactor`

**Description:**
`lib.rs` re-exports `Invoice` from `storage`. Evaluate whether this is needed for the public contract interface or if it can be removed.

**Acceptance Criteria:**
- Check if `Invoice` needs to be part of the public ABI
- Remove the re-export if it is not needed
- Document the decision in a comment

---

### #91 — [refactor] Add `derive(Debug)` to `Invoice` and `InvoiceStatus` for easier test output

**Labels:** `refactor`, `dx`

**Description:**
`Invoice` and `InvoiceStatus` do not derive `Debug`, making test failure messages less informative.

**Acceptance Criteria:**
- Add `#[derive(Debug)]` to both types (if compatible with `contracttype`)
- Verify tests still compile and run

---

### #92 — [refactor] Consolidate test helpers into a `tests/helpers.rs` module

**Labels:** `refactor`, `test`

**Description:**
As tests grow, common setup code (creating env, registering contract, generating addresses) will be duplicated. Extract into shared helpers.

**Acceptance Criteria:**
- Create `contracts/invoice/src/tests/helpers.rs` (or use a `#[cfg(test)]` module)
- Add `setup_env()` and `create_test_invoice()` helpers
- Refactor existing tests to use them

---

### #93 — [refactor] Use named constants for test amounts instead of magic numbers

**Labels:** `refactor`, `test`

**Description:**
The existing test uses `1000` as the invoice amount. Replace magic numbers with named constants for clarity.

**Acceptance Criteria:**
- Define `const TEST_AMOUNT: i128 = 1000` in the test module
- Replace all magic number amounts in tests

---

### #94 — [refactor] Add `clippy::pedantic` and fix all warnings

**Labels:** `refactor`, `code-quality`

**Description:**
Running `cargo clippy -- -W clippy::pedantic` may reveal additional code quality issues. Fix them all.

**Acceptance Criteria:**
- Run `cargo clippy -- -W clippy::pedantic`
- Fix all warnings or explicitly `#[allow(...)]` with a justification comment
- Add pedantic lints to CI

---

### #95 — [refactor] Ensure all `match` expressions on `InvoiceStatus` are exhaustive

**Labels:** `refactor`

**Description:**
As new `InvoiceStatus` variants are added (issues #5), all `match` expressions must handle them. Add a CI check or comment to catch this.

**Acceptance Criteria:**
- Search for all `match` on `InvoiceStatus`
- Ensure none use a wildcard `_` arm that would silently ignore new variants
- Add a comment warning contributors to update matches when adding variants

---

### #96 — [refactor] Extract token transfer logic into a helper function

**Labels:** `refactor`

**Description:**
`fund_invoice` and `release_payment` both perform token transfers. Extract this into a shared helper to avoid duplication.

**Acceptance Criteria:**
- Add `fn transfer_tokens(env: &Env, token: &Address, from: &Address, to: &Address, amount: i128)` in a suitable module
- Use it in both `fund_invoice` and `release_payment`

---

## Community & Project Health (Issues #97–#108)

---

### #97 — [community] Add issue labels configuration (`.github/labels.yml`)

**Labels:** `community`, `tooling`

**Description:**
Define a standard set of GitHub issue labels so all issues are consistently categorized.

**Acceptance Criteria:**
- Create `.github/labels.yml` with labels: `bug`, `feature`, `docs`, `refactor`, `test`, `security`, `tooling`, `good first issue`, `help wanted`, `ci`, `enhancement`, `community`
- Add a GitHub Actions workflow to sync labels using `actions/github-script` or `EndBug/add-and-delete-labels`

---

### #98 — [community] Add `FUNDING.yml` for GitHub Sponsors or Open Collective

**Labels:** `community`

**Description:**
Allow the community to financially support the project.

**Acceptance Criteria:**
- Create `.github/FUNDING.yml`
- Add at least one funding platform (GitHub Sponsors, Open Collective, etc.)

---

### #99 — [community] Add a "Good First Issues" section to `CONTRIBUTING.md`

**Labels:** `community`, `docs`, `good first issue`

**Description:**
New contributors need clear guidance on where to start. The existing table in `CONTRIBUTING.md` is a good start but could be expanded.

**Acceptance Criteria:**
- Add a "Good First Issues" section listing issues #1–#7 and documentation issues
- Include estimated difficulty and required background knowledge for each

---

### #100 — [community] Add a GitHub Discussions configuration

**Labels:** `community`

**Description:**
Enable GitHub Discussions for Q&A, ideas, and community conversation. Add a reference in `CONTRIBUTING.md`.

**Acceptance Criteria:**
- Enable Discussions in the GitHub repo settings (document the step)
- Add categories: `Q&A`, `Ideas`, `Show and Tell`
- Reference Discussions in `CONTRIBUTING.md` under "Questions?"

---

### #101 — [community] Add a project roadmap to `README.md`

**Labels:** `community`, `docs`

**Description:**
A roadmap helps contributors understand the project's direction and prioritize their contributions.

**Acceptance Criteria:**
- Add a "Roadmap" section to README
- List v0.1 (core escrow), v0.2 (disputes/cancellation), v0.3 (multi-token, deadlines) milestones
- Link to relevant issues

---

### #102 — [community] Create GitHub Milestones for v0.1 and v0.2

**Labels:** `community`, `tooling`

**Description:**
GitHub Milestones help track progress toward releases. Create milestones for the first two versions.

**Acceptance Criteria:**
- Create `v0.1.0` milestone: issues #1–#4 (core escrow functions)
- Create `v0.2.0` milestone: issues #5, #13, #14, #15 (disputes and cancellation)
- Assign relevant issues to milestones

---

### #103 — [community] Add a `CONTRIBUTORS.md` file

**Labels:** `community`, `docs`

**Description:**
Recognize contributors by maintaining a `CONTRIBUTORS.md` file.

**Acceptance Criteria:**
- Create `CONTRIBUTORS.md` with a table: Name, GitHub handle, Contribution
- Add the initial author(s)
- Reference it from `CONTRIBUTING.md`

---

### #104 — [community] Add a "Claiming Issues" section to `CONTRIBUTING.md`

**Labels:** `community`, `docs`, `good first issue`

**Description:**
The current `CONTRIBUTING.md` says "comment to claim it" but doesn't explain the process in detail.

**Acceptance Criteria:**
- Add a "Claiming Issues" section explaining: comment on the issue, wait for maintainer acknowledgment, start work within 7 days or the issue is re-opened
- Add a note about not opening PRs for unclaimed issues

---

### #105 — [community] Add a stale issue bot configuration

**Labels:** `community`, `tooling`

**Description:**
Stale issues and PRs accumulate over time. A stale bot keeps the backlog clean.

**Acceptance Criteria:**
- Create `.github/stale.yml` or use `actions/stale`
- Mark issues stale after 30 days of inactivity
- Close stale issues after 7 more days
- Exempt issues with `pinned` or `security` labels

---

### #106 — [community] Add a `SUPPORT.md` file

**Labels:** `community`, `docs`

**Description:**
A `SUPPORT.md` file tells users where to get help, reducing noise in the issue tracker.

**Acceptance Criteria:**
- Create `SUPPORT.md`
- Direct users to GitHub Discussions for questions
- Direct users to the issue tracker only for bugs and feature requests

---

### #107 — [community] Add a "Related Projects" section to README

**Labels:** `community`, `docs`

**Description:**
Link to related Stellar/Soroban projects and resources to help contributors get context.

**Acceptance Criteria:**
- Add a "Related Projects & Resources" section to README
- Link to: Soroban docs, Stellar developer docs, example Soroban contracts, Stellar token interface docs

---

### #108 — [community] Add a project logo or banner to README

**Labels:** `community`, `docs`

**Description:**
A visual identity makes the project more recognizable and professional.

**Acceptance Criteria:**
- Create a simple SVG or PNG logo for StarInvoice
- Add it to the top of `README.md`
- Store it in `assets/` or `.github/`

---

## Advanced Features (Issues #109–#120)

---

### #109 — [feature] Add partial payment support

**Labels:** `feature`, `enhancement`

**Description:**
Currently invoices are all-or-nothing. Some freelance arrangements involve milestone-based partial payments. Add support for partial funding and release.

**Acceptance Criteria:**
- Add `amount_funded: i128` and `amount_released: i128` fields to `Invoice`
- Allow `fund_invoice` to accept a partial amount
- Allow `release_payment` to release a partial amount
- Update status logic accordingly
- Add tests

---

### #110 — [feature] Add invoice versioning / amendment support

**Labels:** `feature`, `enhancement`

**Description:**
Clients and freelancers may need to amend an invoice (e.g., change amount or description) before funding. Add an `amend_invoice` function.

**Acceptance Criteria:**
- Add `pub fn amend_invoice(env: Env, invoice_id: u64, new_amount: i128, new_description: String)`
- Only callable when status is `Pending`
- Require auth from both `freelancer` and `client` (or just freelancer, document the choice)
- Emit an `amended` event
- Add tests

---

### #111 — [feature] Add a `get_invoices_by_status` view function

**Labels:** `feature`, `enhancement`

**Description:**
Frontends need to filter invoices by status (e.g., show all `Pending` invoices). Add a view function for this.

**Acceptance Criteria:**
- Add `pub fn get_invoices_by_status(env: Env, status: InvoiceStatus) -> Vec<u64>`
- This may require maintaining a status index in storage
- Add tests

---

### #112 — [feature] Add platform fee support

**Labels:** `feature`, `enhancement`

**Description:**
A real-world escrow protocol may charge a small platform fee on `release_payment`. Add configurable fee support.

**Acceptance Criteria:**
- Add `fee_bps: u32` (basis points) to contract instance storage, set during `initialize`
- Deduct fee from `amount` during `release_payment` and send to admin address
- Add tests verifying correct fee calculation and transfer

---

### #113 — [feature] Add `extend_deadline` function (depends on #7)

**Labels:** `feature`, `enhancement`

**Description:**
Once deadlines are supported, allow the freelancer and client to mutually agree to extend the deadline.

**Acceptance Criteria:**
- Add `pub fn extend_deadline(env: Env, invoice_id: u64, new_deadline: u64)`
- Require auth from both parties
- Assert `new_deadline > invoice.deadline`
- Emit a `deadline_extended` event
- Add tests

---

### #114 — [feature] Add `tip` function to allow client to send a bonus

**Labels:** `feature`, `enhancement`

**Description:**
Allow clients to send an optional tip to the freelancer on top of the invoice amount.

**Acceptance Criteria:**
- Add `pub fn tip(env: Env, invoice_id: u64, tip_amount: i128)`
- Only callable when status is `Completed`
- Transfer `tip_amount` directly from client to freelancer
- Emit a `tipped` event
- Add tests

---

### #115 — [feature] Add contract upgrade mechanism

**Labels:** `feature`, `security`

**Description:**
Soroban supports contract upgrades via `env.deployer().update_current_contract_wasm()`. Add an admin-controlled upgrade function.

**Acceptance Criteria:**
- Add `pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>)`
- Require auth from admin (depends on issue #83)
- Call `env.deployer().update_current_contract_wasm(new_wasm_hash)`
- Add a test using `env.deployer()`

---

### #116 — [feature] Add off-chain metadata URI field to `Invoice`

**Labels:** `feature`, `enhancement`

**Description:**
Store a URI pointing to off-chain invoice details (PDF, IPFS hash, etc.) to keep on-chain storage minimal.

**Acceptance Criteria:**
- Add `metadata_uri: String` to `Invoice` (optional, can be empty)
- Accept it as a parameter in `create_invoice`
- Add length validation (max 512 bytes)
- Add tests

---

### #117 — [feature] Add multi-milestone invoice support

**Labels:** `feature`, `enhancement`

**Description:**
Large projects are often broken into milestones. Add support for invoices with multiple milestones, each with its own amount and status.

**Acceptance Criteria:**
- Design a `Milestone` struct with `id`, `amount`, `status`, `description`
- Add `milestones: Vec<Milestone>` to `Invoice` (or a separate storage key)
- Add functions to fund, deliver, approve, and release individual milestones
- Add tests

---

### #118 — [feature] Add invoice search by amount range

**Labels:** `feature`, `enhancement`

**Description:**
Allow querying invoices within a specific amount range for analytics and frontend filtering.

**Acceptance Criteria:**
- Add `pub fn get_invoices_by_amount_range(env: Env, min: i128, max: i128) -> Vec<u64>`
- This may require a sorted index or full scan (document the trade-off)
- Add tests

---

### #119 — [feature] Add Soroban contract events indexer guide

**Labels:** `docs`, `feature`

**Description:**
Contributors building frontends need to know how to index contract events. Add a guide explaining how to use Horizon or a custom indexer to listen for StarInvoice events.

**Acceptance Criteria:**
- Create `docs/indexing-events.md`
- Explain the event topic structure used in `events.rs`
- Provide a minimal JavaScript/TypeScript example using `stellar-sdk` to subscribe to events
- Reference from README

---

### #120 — [feature] Add a JavaScript/TypeScript SDK wrapper for the contract

**Labels:** `feature`, `enhancement`

**Description:**
Provide a typed JS/TS client library that wraps the Soroban contract, making it easy for frontend developers to integrate StarInvoice.

**Acceptance Criteria:**
- Create a `sdk/` directory with a TypeScript package
- Use `@stellar/stellar-sdk` and the generated contract bindings
- Export typed functions: `createInvoice`, `fundInvoice`, `markDelivered`, `approvePayment`, `releasePayment`
- Add a `README.md` in `sdk/` with usage examples
- Add basic tests using Jest or Vitest

---

## Summary

| Range     | Category                        | Count |
|-----------|---------------------------------|-------|
| #1–#5     | Core Escrow Functions           | 5     |
| #6–#15    | Invoice Data Model              | 10    |
| #16–#22   | Events                          | 7     |
| #23–#32   | Storage & State Management      | 10    |
| #33–#45   | Testing                         | 13    |
| #46–#58   | Documentation                   | 13    |
| #59–#72   | Developer Experience & Tooling  | 14    |
| #73–#83   | Security & Access Control       | 11    |
| #84–#96   | Refactoring & Code Quality      | 13    |
| #97–#108  | Community & Project Health      | 12    |
| #109–#120 | Advanced Features               | 12    |
| **Total** |                                 | **120** |
