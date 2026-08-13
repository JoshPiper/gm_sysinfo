# Security Policy

## Supported versions

Only the **latest release** is supported. There are no maintained LTS branches — if a fix is needed, it ships in the next release, built on top of current `main`.

## Reporting a vulnerability

**Please don't open a public issue for a security report.**

Use GitHub's private vulnerability reporting instead:

**[Report a vulnerability](https://github.com/JoshPiper/gm_sysinfo/security/advisories/new)**

This opens a draft security advisory visible only to the maintainer and you, with room to discuss and fix the issue before anything is public. If you'd rather not use GitHub for this, open a regular issue asking for another contact channel — just don't put report details in it.

Please include, if you can:
- The version (or commit/`get_build_info()` output) affected.
- Steps to reproduce, or a minimal repro.
- What you think the impact is (e.g. crash, memory disclosure, arbitrary code execution in the game process).

## What's in scope

- The Rust module itself (`src/`) and its build script (`build.rs`).
- The release pipeline (`.github/workflows/`) — e.g. anything that could let a malicious binary get attached to a release under this project's name.
- A vulnerable dependency version actually shipped in a release binary — see [`Verifying a release`](README.md#verifying-a-release) for how to check what's in one.

**Out of scope**: vulnerabilities in Garry's Mod itself, in Rust/rustc, or in a dependency that don't have a specific exploitation path through this module — those are better reported upstream. Feel free to flag them here too if you're not sure who else should know; a dependency bump is a one-line fix on this end regardless.

## What to expect

This is a small project maintained in spare time, not a funded security-critical product — there's no formal SLA. That said, a credible report will be acknowledged and triaged as quickly as reasonably possible, and coordinated disclosure is genuinely appreciated: please give a reasonable window to ship a fix before disclosing publicly. Credit is happily given in the release notes if you'd like it.
