# Security Policy

## Purpose

This document exists to:
- Say where to send a vulnerability report.
- Define what's in scope and what isn't.
- Set honest expectations for response time.

---

## Supported versions

Latest release only. No maintained LTS branch — a fix ships on top of current `main` in the next release.

---

## Reporting a vulnerability

**Don't open a public issue for this.**

**[Report a vulnerability](https://github.com/JoshPiper/gm_sysinfo/security/advisories/new)** — GitHub's private advisory flow, visible only to me and you until it's resolved. No GitHub? Open a normal issue asking for another contact, without report details in it.

Include, if you can:
- The version, or `get_build_info()` output, of the binary affected.
- Steps to reproduce, or a minimal repro.
- The impact — crash, memory disclosure, arbitrary code execution in the game process, whatever it is.

---

## Scope

In scope:
- The Rust module (`src/`) and its build script (`build.rs`).
- The release pipeline (`.github/workflows/`) — a malicious binary attached to a release under this project's name.
- A vulnerable dependency actually shipped in a release binary. See [Verifying a release](README.md#verifying-a-release) for how to check what's in one.

Out of scope:
- Garry's Mod itself, Rust/rustc, or a dependency vulnerability with no exploitation path through this module. Report upstream. Flag it here too if unsure — a dependency bump is a one-line fix on this end regardless.

---

## What to expect

No SLA — spare-time project. Credible reports get triaged fast. Give a fair window before public disclosure; credit goes in the release notes if you want it.
