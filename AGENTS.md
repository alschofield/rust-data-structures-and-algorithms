# Learning Repo Source Ownership

This is a learning repository. The user writes all production Rust by hand.

- Do not create, delete, or modify production implementation files under `src/`: `.rs` files other than `tests.rs`.
- This includes blank scaffolds. Do not fill modules, add public declarations, create `todo!` bodies, or translate README contracts into Rust unless the user explicitly requests that exact source edit.
- Never rewrite, replace, or delete a completed implementation.
- `tests.rs`, `README.md`, benchmark harnesses, and repository tooling are agent-editable when needed or requested.
- Review production source read-only: report findings, let the user make logic changes, then re-review and verify.
