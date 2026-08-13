# Contributing to gm_sysinfo

A'ite, thanks for taking a look. This is a small project so the process is light — but there's one rule below that isn't optional, and it's worth reading before you open a PR rather than after CI's confused you.

---

## Purpose

This document exists to:
- Explain the one rule that actually matters, and why it's not negotiable.
- Set out what CI gates a PR on, so you're not guessing.
- Save you rediscovering the local build/lint/test setup by trial and error.

---

## Commit messages must be Conventional Commits

Not a style preference. [release-plz](https://release-plz.dev) reads your commit messages to decide whether a release happens, what version it gets, and what the changelog says. Get the type wrong and nothing breaks loudly — nothing happens at all, which is worse. No release, no changelog entry, no warning. The commit just sits there looking normal.

Format: `<type>[optional scope]: <description>`

**Types that matter to the release process:**

| Type | Effect |
|---|---|
| `fix:` | Patch release (`x.y.Z`) |
| `feat:` | Minor release (`x.Y.0`) |
| `feat!:` or a `BREAKING CHANGE:` footer | Major release (`X.0.0`) |
| `chore:`, `build:`, `ci:`, `docs:`, `test:`, `refactor:`, `style:` | No release triggered on their own |

Examples:

```
fix: correct the get_host_name error message

feat: expose sysinfo.get_build_info()

feat!: return memory and swap in bytes instead of KiB

BREAKING CHANGE: sysinfo.get_memory() and sysinfo.get_swap() now
return bytes. Previously they returned KiB.
```

If a PR mixes several logical changes, split the commits by type rather than reaching for whichever tag sounds biggest. release-plz aggregates every commit into the changelog regardless, so there's no upside to lumping it all under one `feat:` — and smaller commits review better besides.

---

## What CI actually gates

`ci.yml` runs, on every PR:
- `cargo fmt --check`
- `cargo clippy -D warnings`, both realms
- The full 10-target build matrix
- [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against real Garry's Mod server instances

All of it needs to pass. There's no merging around a red check.

---

## What you don't need to do

- Bump the version.
- Touch `CHANGELOG.md`.
- Create a tag.

release-plz does all three off your commit messages once your PR's on `main`. If you catch yourself editing the version field in `Cargo.toml`, stop — that's not your job any more, and it'll just conflict with the release PR release-plz maintains automatically. A maintainer merges that when it's time to ship; see the root README for why that's a deliberate gate rather than an oversight.

---

## Local setup

The pinned nightly toolchain in [`rust-toolchain.toml`](rust-toolchain.toml) gets picked up by `rustup` automatically — nothing to install by hand beyond `rustup` itself.

```bash
cargo build --release                    # server realm
cargo build --release --features gmcl    # client realm
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features gmcl -- -D warnings
```

Run both clippy invocations, not just one — the two realms compile different code paths (see the `gmcl` feature gate in `src/lib.rs`), and CI lints both.

Cross-compiling to a specific release target needs that target installed:

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

32-bit Linux targets also need a multilib GCC (`sudo apt-get install gcc-multilib` on Debian/Ubuntu).

### Running the Lua tests locally

CI runs [GLuaTest](https://github.com/CFC-Servers/GLuaTest) against the built server module on every PR (`.github/workflows/ci.yml`). Running it locally needs Docker — see GLuaTest's own docs for the local invocation. Specs live in `lua/tests/sysinfo/`.

---

## Code style

- No comments explaining *what* the code does — the names already do that. A comment earns its place only when it captures a non-obvious *why*: a hidden constraint, a workaround, a decision that would otherwise look like a mistake to a reviewer.
- Keep the `unsafe` surface exactly as small as it is today. Adding a new Lua-exported function? Follow the existing `#[lua_function]` pattern in `src/lib.rs` rather than inventing a new calling convention.
- A new getter that fits the shape of its neighbours extends the existing macros (`err!`, `export_lua_function!`, `set_field!`). It doesn't get a hand-rolled one-off.
- Run `cargo fmt` before you push. CI enforces it, and it will not fix it for you.

---

## Reporting a security issue

Don't put it in a PR or an issue. See [SECURITY.md](SECURITY.md).
