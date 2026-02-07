# LOOP PROMPT: GridHammer Orchestrator

Maintain deterministic progress toward the **GridHammer OCPP 2.1 simulation platform**.

This loop is the orchestration layer. `AGENTS.md` defines engineering constraints and ownership; this document defines iteration control.

Authoritative references:

* PROJECT_PROMPT.md
* PROJECT_SPEC.md
* MANIFEST.md
* ARCHITECTURE.md
* AGENTS.md
* RUNBOOK.md
* DECISIONS.md
* docs/spec/OCPP-2.1/

## Execution Protocol

* **Single Task Focus**: Each iteration executes exactly ONE task: the `NOW` entry from `NEXT_STEPS.md`. Do not multitask.
* **Pre-flight Read** (mandatory, in this order):

  1. `NEXT_STEPS.md`
  2. `PROJECT_PROMPT.md`
  3. `PROJECT_SPEC.md`
  4. `MANIFEST.md`
  5. `ARCHITECTURE.md`
  6. `AGENTS.md`
  7. Relevant OCPP spec sections under `docs/spec/OCPP-2.1/`
* **Ownership Enforcement**:

  * Identify the owning agent (gateway/session/registry/pricing/ocpi/shared) and keep edits inside its path per `AGENTS.md`.
  * Do not mix responsibilities between Rust core (protocol semantics) and TypeScript shell (control plane).
* **Test Verification Loop**:

  * After changes, run the commands listed in `RUNBOOK.md` (CI section). When unspecified, run service-specific `make test` or equivalent.
  * Iteration completes only when tests pass or a follow-up `NOW` task is added to resolve the failure immediately.
* **Mandatory Final Action**:

  * At iteration end, the terminal agent must commit all staged and tracked changes; working tree must be clean.
  * Keep `.gitignore` aligned with runtime artifacts so logs and binaries stay out of commits.
* **Documentation Synchronization**:

  * Behavior changes require synchronized docs (`ARCHITECTURE.md`, `RUNBOOK.md`, `DECISIONS.md`, service READMEs) in the same iteration.
  * Architecture-level tradeoffs must be recorded in `DECISIONS.md` before implementation proceeds.
* **Resilience & Resumability**:

  * Assume work may pause at any moment; leave `NEXT_STEPS.md` ready to resume (move completed items to `DONE`).

## Task Picking and Task Hygiene

* `NOW` must represent an atomic, finishable task. Split larger work across subsequent iterations.
* If a change spans multiple components, break it into separate `NEXT` tasks and tackle them sequentially.

## Orchestrator Responsibilities

* Coordinate agents while keeping them focused on their ownership boundaries.
* Ensure all work aligns with `PROJECT_PROMPT.md`/`PROJECT_SPEC.md`, `ARCHITECTURE.md`, and `AGENTS.md`.
* Keep diffs small, readable, and deterministic.
* Prefer concise commits tied to one intent.

## Commit Conventions

Commit messages must follow the format `type(scope): intent`.

Examples:

* `chore(repo): scaffold gateway service`
* `feat(gateway): add /health endpoint`
* `test(gateway): add websocket handshake tests`
* `docs(architecture): clarify event flow`

Never commit secrets, tokens, or log artifacts.

## Signaling Completion

* Output `<promise>NEXT_TASK</promise>` when the current task is done but other items remain.
* Output `<promise>DONE</promise>` when no further work is required.

## Sub-Agent Usage

* Delegate implementation to the owning service agent.
* Delegate testing/verification to the test or review agents.
* Use the terminal agent strictly for command execution and verification.
* Brief sub-agents with relevant documents and tasks before delegation.
* Perform a final consistency pass before committing and signaling.
