# Security Policy

## Purpose

This document exists to:
- Say plainly where to send a vulnerability report, and where not to.
- Draw the line between what's this project's problem and what isn't.
- Set expectations that are actually true, rather than the boilerplate ones.

---

## Supported versions

Latest release only. There's no maintained LTS branch, and there isn't going to be — if a fix is needed, it ships on top of current `main` in the next release.

---

## Reporting a vulnerability

**Don't open a public issue for this.**

Use GitHub's private vulnerability reporting instead:

**[Report a vulnerability](https://github.com/JoshPiper/gm_sysinfo/security/advisories/new)**

That opens a draft security advisory visible only to me and you, with room to sort the issue out before any of it is public. If you'd rather not use GitHub for it, open a normal issue asking for another way to reach me — just don't put report details in it.

Include, if you can:
- The version, or `get_build_info()` output, of the binary affected.
- Steps to reproduce, or a minimal repro.
- What you think the impact is — crash, memory disclosure, arbitrary code execution in the game process, whatever it is.

---

## Scope

**In scope:**
- The Rust module itself (`src/`) and its build script (`build.rs`).
- The release pipeline (`.github/workflows/`) — a malicious binary ending up attached to a release under this project's name is very much in scope.
- A vulnerable dependency version actually shipped in a release binary. See [Verifying a release](README.md#verifying-a-release) for how to check what's in one.

**Out of scope:**
- Garry's Mod itself, Rust/rustc, or a dependency vulnerability with no exploitation path through this module. Report those upstream. Flag them here too if you're not sure who else should know — a dependency bump is a one-line fix on this end regardless of where the report lands.

---

## What to expect

This is a spare-time project, not a funded security-critical product, and I'm not going to pretend there's an SLA when there isn't one. A credible report gets triaged as fast as I can reasonably manage. In return, give me a fair window to ship a fix before disclosing publicly — coordinated disclosure, not a race. Credit goes in the release notes if you want it.
